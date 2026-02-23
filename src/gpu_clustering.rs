use std::sync::Arc;

use anyhow::{Context, Result};
use log::debug;
use vulkano::{
    buffer::{Buffer, BufferCreateInfo, BufferUsage, Subbuffer},
    command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage},
    descriptor_set::{DescriptorSet, WriteDescriptorSet, layout::DescriptorSetLayout},
    memory::allocator::{AllocationCreateInfo, MemoryTypeFilter},
    pipeline::{
        ComputePipeline, Pipeline, PipelineLayout, PipelineShaderStageCreateInfo,
        compute::ComputePipelineCreateInfo, layout::PipelineDescriptorSetLayoutCreateInfo,
    },
    query::{QueryPool, QueryPoolCreateInfo, QueryType},
    sync::GpuFuture,
};

use crate::{gpu_voxel::VoxelGpuContext, init_gpu::VulkanContext};

#[derive(bytemuck::Pod, bytemuck::Zeroable, Clone, Copy)]
#[repr(C)]
pub struct PushConsts {
    pub num_points: i32,
    pub table_size: i32,
    pub voxel_size: f32,
    pub _pad: i32,
}

pub struct ClusteringGpuContext {
    vulkan_context: VulkanContext,

    compute_pipeline_union: Arc<ComputePipeline>,
    compute_pipeline_find: Arc<ComputePipeline>,
    pipeline_layout_union: Arc<PipelineLayout>,
    pipeline_layout_find: Arc<PipelineLayout>,
    descriptor_set_layout_union: Arc<DescriptorSetLayout>,
    descriptor_set_layout_find: Arc<DescriptorSetLayout>,

    // pub d_buf_keys: Option<Subbuffer<[u32]>>,
    pub d_buf_out_pts: Option<Subbuffer<[f32]>>,
    // pub d_buf_counter: Option<Subbuffer<[u32]>>,
    pub d_buf_table_voxel_indices: Option<Subbuffer<[u32]>>,
    pub d_buf_cluster_ids: Option<Subbuffer<[u32]>>,
    pub d_buf_sync_flag: Option<Subbuffer<[u32]>>,

    staging_buf_output_pts: Option<Subbuffer<[f32]>>,
    pub staging_buf_sync_flag: Option<Subbuffer<[u32]>>,
    pub staging_buf_cluster_ids: Option<Subbuffer<[u32]>>,
    // staging_buf_output_counter: Option<Subbuffer<[u32]>>,
    pub current_capacity_pts: usize,

    pub num_points: i32,
    pub table_size: i32,
    pub voxel_size: f32,
}

impl ClusteringGpuContext {
    pub fn new(vulkan_context: VulkanContext) -> Result<Self> {
        mod cs_union {
            vulkano_shaders::shader! {
                ty: "compute",
                path: "src/kernels/clustering/cluster_union.glsl",
            }
        }

        mod cs_find {
            vulkano_shaders::shader! {
                ty: "compute",
                path: "src/kernels/clustering/cluster_find.glsl",
            }
        }

        let shader_union =
            cs_union::load(vulkan_context.device.clone()).context("Failed to load union shader")?;
        let shader_find =
            cs_find::load(vulkan_context.device.clone()).context("Failed to load find shader")?;

        let cs_union = shader_union
            .entry_point("main")
            .context("Failed to find entry point in union shader")?;
        let cs_find = shader_find
            .entry_point("main")
            .context("Failed to find entry point in find shader")?;

        let stage_union = PipelineShaderStageCreateInfo::new(cs_union);
        let stage_find = PipelineShaderStageCreateInfo::new(cs_find);

        let layout_union = PipelineLayout::new(
            vulkan_context.device.clone(),
            PipelineDescriptorSetLayoutCreateInfo::from_stages([&stage_union])
                .into_pipeline_layout_create_info(vulkan_context.device.clone())
                .context("Failed to create pipeline layout")?,
        )
        .context("Failed to create pipeline layout")?;
        let layout_find = PipelineLayout::new(
            vulkan_context.device.clone(),
            PipelineDescriptorSetLayoutCreateInfo::from_stages([&stage_find])
                .into_pipeline_layout_create_info(vulkan_context.device.clone())
                .context("Failed to create pipeline layout")?,
        )
        .context("Failed to create pipeline layout")?;

        let compute_pipeline_union = ComputePipeline::new(
            vulkan_context.device.clone(),
            None,
            ComputePipelineCreateInfo::stage_layout(stage_union, layout_union),
        )
        .context("Failed to create compute pipeline union")?;
        let compute_pipeline_find = ComputePipeline::new(
            vulkan_context.device.clone(),
            None,
            ComputePipelineCreateInfo::stage_layout(stage_find, layout_find),
        )
        .context("Failed to create compute pipeline find")?;

        let pipeline_layout_union = compute_pipeline_union.layout();
        let pipeline_layout_find = compute_pipeline_find.layout();

        let descriptor_set_layout_union = pipeline_layout_union
            .set_layouts()
            .get(0)
            .context("Failed to get descriptor set layout for union")?;
        let descriptor_set_layout_find = pipeline_layout_find
            .set_layouts()
            .get(0)
            .context("Failed to get descriptor set layout for find")?;

        Ok(Self {
            vulkan_context: vulkan_context.clone(),
            compute_pipeline_union: compute_pipeline_union.clone(),
            compute_pipeline_find: compute_pipeline_find.clone(),
            pipeline_layout_union: pipeline_layout_union.clone(),
            pipeline_layout_find: pipeline_layout_find.clone(),
            descriptor_set_layout_union: descriptor_set_layout_union.clone(),
            descriptor_set_layout_find: descriptor_set_layout_find.clone(),
            d_buf_out_pts: None,
            d_buf_table_voxel_indices: None,
            d_buf_cluster_ids: None,
            d_buf_sync_flag: None,
            staging_buf_output_pts: None,
            staging_buf_sync_flag: None,
            staging_buf_cluster_ids: None,
            current_capacity_pts: 0,
            num_points: 0,
            table_size: 0,
            voxel_size: 0.0,
        })
    }

    pub fn clustering(
        &mut self,
        voxel_gpu_context: &VoxelGpuContext,
        pts: &[[f32; 3]],
        num_pts: usize,
        voxel_size: f32,
    ) -> Result<Vec<u32>> {
        if num_pts == 0 {
            return Ok(vec![]);
        }

        let device = &self.vulkan_context.device;
        let queue = &self.vulkan_context.queue;
        let memory_allocator = &self.vulkan_context.memory_allocator;
        let descriptor_set_allocator = &self.vulkan_context.descriptor_set_allocator;
        let command_buffer_allocator = &self.vulkan_context.command_buffer_allocator;
        let pipeline_layout_union = &self.pipeline_layout_union;
        let pipeline_layout_find = &self.pipeline_layout_find;
        let compute_pipeline_union = &self.compute_pipeline_union;
        let compute_pipeline_find = &self.compute_pipeline_find;

        let table_size = num_pts * 4;
        self.num_points = num_pts as i32;
        self.table_size = voxel_gpu_context.table_size;
        self.voxel_size = voxel_size;
        let consts_data = PushConsts {
            num_points: self.num_points,
            table_size: self.table_size,
            voxel_size: self.voxel_size,
            _pad: 0,
        };

        if self.current_capacity_pts < num_pts {
            debug!("Reallocating buffers for {} points", num_pts);

            let new_capacity = (num_pts as f64 * 1.5) as usize;
            self.current_capacity_pts = new_capacity;

            self.d_buf_sync_flag = Some(Buffer::new_slice::<u32>(
                memory_allocator.clone(),
                BufferCreateInfo {
                    usage: BufferUsage::STORAGE_BUFFER
                        | BufferUsage::TRANSFER_DST
                        | BufferUsage::TRANSFER_SRC,
                    ..Default::default()
                },
                AllocationCreateInfo {
                    memory_type_filter: MemoryTypeFilter::PREFER_DEVICE,
                    ..Default::default()
                },
                1,
            )?);

            self.staging_buf_sync_flag = Some(Buffer::new_slice::<u32>(
                memory_allocator.clone(),
                BufferCreateInfo {
                    usage: BufferUsage::TRANSFER_SRC | BufferUsage::TRANSFER_DST,
                    ..Default::default()
                },
                AllocationCreateInfo {
                    memory_type_filter: MemoryTypeFilter::PREFER_HOST
                        | MemoryTypeFilter::HOST_RANDOM_ACCESS,
                    ..Default::default()
                },
                1,
            )?);

            self.staging_buf_cluster_ids = Some(Buffer::new_slice::<u32>(
                memory_allocator.clone(),
                BufferCreateInfo {
                    usage: BufferUsage::TRANSFER_DST,
                    ..Default::default()
                },
                AllocationCreateInfo {
                    memory_type_filter: MemoryTypeFilter::PREFER_HOST
                        | MemoryTypeFilter::HOST_RANDOM_ACCESS,
                    ..Default::default()
                },
                self.current_capacity_pts as u64,
            )?);
        }

        let descriptor_set_union = DescriptorSet::new(
            descriptor_set_allocator.clone(),
            self.descriptor_set_layout_union.clone(),
            [
                WriteDescriptorSet::buffer(
                    0,
                    voxel_gpu_context
                        .d_buf_out_pts
                        .as_ref()
                        .context("Failed to get buffer for descriptor set")?
                        .clone(),
                ),
                WriteDescriptorSet::buffer(
                    1,
                    voxel_gpu_context
                        .d_buf_keys
                        .as_ref()
                        .context("Failed to get buffer for descriptor set")?
                        .clone(),
                ),
                WriteDescriptorSet::buffer(
                    2,
                    voxel_gpu_context
                        .d_buf_table_voxel_indices
                        .as_ref()
                        .context("Failed to get buffer for descriptor set")?
                        .clone(),
                ),
                WriteDescriptorSet::buffer(
                    3,
                    voxel_gpu_context
                        .d_buf_cluster_ids
                        .as_ref()
                        .context("Failed to get buffer for descriptor set")?
                        .clone(),
                ),
                WriteDescriptorSet::buffer(
                    4,
                    self.d_buf_sync_flag
                        .as_ref()
                        .context("Failed to get buffer for descriptor set")?
                        .clone(),
                ),
            ],
            [],
        )?;

        let descriptor_set_find = DescriptorSet::new(
            descriptor_set_allocator.clone(),
            self.descriptor_set_layout_find.clone(),
            [WriteDescriptorSet::buffer(
                0,
                voxel_gpu_context
                    .d_buf_cluster_ids
                    .as_ref()
                    .context("Failed to get buffer for descriptor set")?
                    .clone(),
            )],
            [],
        )?;

        let mut command_buffer_builder = AutoCommandBufferBuilder::primary(
            command_buffer_allocator.clone(),
            queue.queue_family_index().clone(),
            CommandBufferUsage::OneTimeSubmit,
        )
        .context("Failed to create command buffer builder")?;

        const LOCAL_SIZE: u32 = 256;
        let group_count_x = (num_pts as u32 + LOCAL_SIZE - 1) / LOCAL_SIZE;
        let work_group_count = [group_count_x, 1, 1];

        if let Some(staging_buf) = &self.staging_buf_sync_flag {
            let mut mapping = staging_buf.write()?;
            mapping[0] = 0;
        }
        command_buffer_builder.copy_buffer(vulkano::command_buffer::CopyBufferInfo::buffers(
            self.staging_buf_sync_flag
                .as_ref()
                .unwrap()
                .clone()
                .slice(0..1),
            self.d_buf_sync_flag.as_ref().unwrap().clone().slice(0..1),
        ))?;

        unsafe {
            command_buffer_builder
                .bind_pipeline_compute(compute_pipeline_union.clone())?
                .push_constants(pipeline_layout_union.clone(), 0, consts_data)?
                .bind_descriptor_sets(
                    vulkano::pipeline::PipelineBindPoint::Compute,
                    pipeline_layout_union.clone(),
                    0,
                    descriptor_set_union.clone(),
                )?
                .dispatch(work_group_count)?;

            let num_iterations = 5;
            for _ in 0..num_iterations {
                command_buffer_builder.dispatch(work_group_count)?;
            }

            command_buffer_builder
                .bind_pipeline_compute(compute_pipeline_find.clone())?
                .push_constants(pipeline_layout_find.clone(), 0, consts_data)?
                .bind_descriptor_sets(
                    vulkano::pipeline::PipelineBindPoint::Compute,
                    pipeline_layout_find.clone(),
                    0,
                    descriptor_set_find.clone(),
                )?
                .dispatch(work_group_count)?;
        }

        command_buffer_builder.copy_buffer(vulkano::command_buffer::CopyBufferInfo::buffers(
            self.d_buf_sync_flag.as_ref().unwrap().clone().slice(0..1),
            self.staging_buf_sync_flag
                .as_ref()
                .unwrap()
                .clone()
                .slice(0..1),
        ))?;

        command_buffer_builder.copy_buffer(vulkano::command_buffer::CopyBufferInfo::buffers(
            voxel_gpu_context
                .d_buf_cluster_ids
                .as_ref()
                .unwrap()
                .clone()
                .slice(0..num_pts as u64),
            self.staging_buf_cluster_ids
                .as_ref()
                .unwrap()
                .clone()
                .slice(0..num_pts as u64),
        ))?;

        let command_buffer = command_buffer_builder.build()?;

        let compute_start_time = std::time::Instant::now();
        let future = vulkano::sync::now(device.clone())
            .then_execute(queue.clone(), command_buffer)?
            .then_signal_fence_and_flush()?;

        future.wait(None)?;
        let compute_elapsed = compute_start_time.elapsed();
        debug!(
            "Compute clustering shader execution time: {:.2?}",
            compute_elapsed
        );

        let cluster_ids_content = self.staging_buf_cluster_ids.as_ref().unwrap().read()?;
        let result_ids: Vec<u32> = cluster_ids_content.iter().take(num_pts).copied().collect();

        Ok(result_ids)
    }
}
