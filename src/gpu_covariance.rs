use std::sync::Arc;

use anyhow::Result;
use vulkano::{
    buffer::{Buffer, BufferCreateInfo, BufferUsage, Subbuffer},
    command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage, CopyBufferInfo},
    descriptor_set::{DescriptorSet, layout::DescriptorSetLayout},
    memory::allocator::{AllocationCreateInfo, MemoryTypeFilter},
    pipeline::{
        ComputePipeline, Pipeline, PipelineLayout, PipelineShaderStageCreateInfo,
        compute::ComputePipelineCreateInfo, layout::PipelineDescriptorSetLayoutCreateInfo,
    },
    query::{QueryPool, QueryPoolCreateInfo, QueryType},
    sync::{self, GpuFuture},
};

use crate::init_gpu::VulkanContext;

#[derive(bytemuck::Pod, bytemuck::Zeroable, Clone, Copy)]
#[repr(C)]
struct PushConsts {
    num_points: i32,
}

pub struct CovarianceGpuContext {
    vulkan_context: VulkanContext,

    compute_pipeline: Arc<ComputePipeline>,
    pipeline_layout: Arc<PipelineLayout>,
    descriptor_set_layout: Arc<DescriptorSetLayout>,

    d_buf_input_pts: Option<Subbuffer<[f32]>>,
    d_buf_output_covs: Option<Subbuffer<[f32]>>,
}

impl CovarianceGpuContext {
    pub fn new(vulkan_context: VulkanContext) -> Result<Self> {
        mod cs_covariance {
            vulkano_shaders::shader! {
                ty: "compute",
                path: "src/kernels/covariance/covariance.glsl",
            }
        }

        let shader_covariance = cs_covariance::load(vulkan_context.device.clone())?;

        let cs_covariance = shader_covariance.entry_point("main").unwrap();

        let stage_covariance = PipelineShaderStageCreateInfo::new(cs_covariance);

        let layout_covariance = PipelineLayout::new(
            vulkan_context.device.clone(),
            PipelineDescriptorSetLayoutCreateInfo::from_stages([&stage_covariance])
                .into_pipeline_layout_create_info(vulkan_context.device.clone())
                .expect("Failed to create pipeline layout"),
        )
        .expect("Failed to create pipeline layout");

        let compute_pipeline_covariance = ComputePipeline::new(
            vulkan_context.device.clone(),
            None,
            ComputePipelineCreateInfo::stage_layout(stage_covariance, layout_covariance),
        )
        .expect("Failed to create compute pipeline");

        let pipeline_layout_covariance = compute_pipeline_covariance.layout().clone();

        let descriptor_set_layout_covariance =
            pipeline_layout_covariance.set_layouts().get(0).unwrap();

        Ok(Self {
            vulkan_context: vulkan_context.clone(),
            compute_pipeline: compute_pipeline_covariance.clone(),
            pipeline_layout: pipeline_layout_covariance.clone(),
            descriptor_set_layout: descriptor_set_layout_covariance.clone(),
            d_buf_input_pts: None,
            d_buf_output_covs: None,
        })
    }

    pub fn compute_covariances(
        &mut self,
        pts: &[[f32; 3]],
        num_pts: usize,
    ) -> Result<Vec<[f32; 9]>> {
        let device = &self.vulkan_context.device;
        let queue = &self.vulkan_context.queue;
        let memory_allocator = &self.vulkan_context.memory_allocator;
        let descriptor_set_allocator = &self.vulkan_context.descriptor_set_allocator;
        let command_buffer_allocator = &self.vulkan_context.command_buffer_allocator;
        let pipeline_layout = &self.pipeline_layout;
        let compute_pipeline = &self.compute_pipeline;

        let consts = PushConsts {
            num_points: num_pts as i32,
        };

        // input points
        let flattened_pts: Vec<f32> = pts.iter().flat_map(|arr| arr.iter().copied()).collect();
        self.d_buf_input_pts = Buffer::from_iter(
            memory_allocator.clone(),
            BufferCreateInfo {
                usage: BufferUsage::STORAGE_BUFFER,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                    | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            flattened_pts,
        )
        .expect("Failed to create buf_pts buffer!")
        .into();

        self.d_buf_output_covs = Buffer::new_slice::<f32>(
            memory_allocator.clone(),
            BufferCreateInfo {
                usage: BufferUsage::STORAGE_BUFFER | BufferUsage::TRANSFER_SRC,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE,
                ..Default::default()
            },
            (num_pts * 9) as u64,
        )
        .expect("Failed to create buf_out_pts buffer")
        .into();

        let descriptor_set = DescriptorSet::new(
            descriptor_set_allocator.clone(),
            self.descriptor_set_layout.clone(),
            [
                vulkano::descriptor_set::WriteDescriptorSet::buffer(
                    0,
                    self.d_buf_input_pts.as_ref().unwrap().clone(),
                ),
                vulkano::descriptor_set::WriteDescriptorSet::buffer(
                    1,
                    self.d_buf_output_covs.as_ref().unwrap().clone(),
                ),
            ],
            [],
        )
        .expect("Failed to create descriptor set");

        let mut command_buffer_builder = AutoCommandBufferBuilder::primary(
            command_buffer_allocator.clone(),
            queue.queue_family_index().clone(),
            CommandBufferUsage::OneTimeSubmit,
        )
        .unwrap();

        const LOCAL_SIZE: u32 = 256;
        let group_count_x = (num_pts as u32 + LOCAL_SIZE - 1) / LOCAL_SIZE;
        let work_group_count = [group_count_x, 1, 1];

        let mut create_info = QueryPoolCreateInfo::query_type(QueryType::Timestamp);
        create_info.query_count = 6;

        let query_pool = QueryPool::new(device.clone(), create_info).unwrap();

        unsafe {
            command_buffer_builder
                .write_timestamp(query_pool.clone(), 0, sync::PipelineStage::ComputeShader)
                .unwrap()
                .bind_pipeline_compute(compute_pipeline.clone())
                .unwrap()
                .push_constants(pipeline_layout.clone(), 0, consts)
                .unwrap()
                .bind_descriptor_sets(
                    vulkano::pipeline::PipelineBindPoint::Compute,
                    pipeline_layout.clone(),
                    0,
                    descriptor_set.clone(),
                )
                .unwrap()
                .dispatch(work_group_count)
                .unwrap()
                .write_timestamp(query_pool.clone(), 1, sync::PipelineStage::ComputeShader)
                .unwrap()
                .push_constants(pipeline_layout.clone(), 0, consts)
                .unwrap()
                .bind_descriptor_sets(
                    vulkano::pipeline::PipelineBindPoint::Compute,
                    pipeline_layout.clone(),
                    0,
                    descriptor_set.clone(),
                )
                .unwrap()
                .dispatch(work_group_count)
                .unwrap()
                .write_timestamp(query_pool.clone(), 2, sync::PipelineStage::ComputeShader)
                .unwrap();
        }

        let command_buffer = command_buffer_builder.build().unwrap();

        let compute_start_time = std::time::Instant::now();
        let future = sync::now(device.clone())
            .then_execute(queue.clone(), command_buffer)
            .unwrap()
            .then_signal_fence_and_flush()
            .unwrap();

        future.wait(None).unwrap();

        let compute_end_time = compute_start_time.elapsed();
        println!("Compute shader execution time: {:?}", compute_end_time);

        let staging_out_covs = Buffer::new_slice::<f32>(
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
            (num_pts * 9) as u64,
        )?;

        let mut copy_builder = AutoCommandBufferBuilder::primary(
            command_buffer_allocator.clone(),
            queue.queue_family_index().clone(),
            CommandBufferUsage::OneTimeSubmit,
        )?;

        copy_builder.copy_buffer(CopyBufferInfo::buffers(
            self.d_buf_output_covs.as_ref().unwrap().clone(),
            staging_out_covs.clone(),
        ))?;

        let copy_command_buffer = copy_builder.build()?;

        let copy_future = sync::now(device.clone())
            .then_execute(queue.clone(), copy_command_buffer)?
            .then_signal_fence_and_flush()?;

        copy_future.wait(None)?;

        let out_pts_covs_content = staging_out_covs.read()?;
        let output_covs: Vec<[f32; 9]> = out_pts_covs_content
            .chunks_exact(9)
            .take(num_pts)
            .map(|chunk| {
                chunk
                    .try_into()
                    .expect("Chunk should have exactly 9 elements")
            })
            .collect();

        Ok(output_covs)
    }
}
