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

use crate::{
    gpu_voxel::{PushConsts, VoxelGpuContext},
    init_gpu::VulkanContext,
};

pub struct CovarianceGpuContext {
    vulkan_context: VulkanContext,

    compute_pipeline: Arc<ComputePipeline>,
    pipeline_layout: Arc<PipelineLayout>,
    descriptor_set_layout: Arc<DescriptorSetLayout>,

    d_buf_input_pts: Option<Subbuffer<[f32]>>,
    d_buf_output_covs: Option<Subbuffer<[f32]>>,
    
    staging_buf_output_covs: Option<Subbuffer<[f32]>>,

    pub current_capacity_pts: usize,
}

impl CovarianceGpuContext {
    pub fn new(vulkan_context: VulkanContext) -> Result<Self> {
        mod cs_covariance {
            vulkano_shaders::shader! {
                ty: "compute",
                path: "src/kernels/covariance/covariance.glsl",
            }
        }

        let shader_covariance = cs_covariance::load(vulkan_context.device.clone())
            .expect("Failed to load covariance shader!");

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
            staging_buf_output_covs: None,
            current_capacity_pts: 0,
        })
    }

    pub fn compute_covariances(
        &mut self,
        voxel_gpu_context: &VoxelGpuContext,
        pts: &[[f32; 3]],
        num_pts: usize,
        only_compute_covs: bool,
    ) -> Result<Vec<[f32; 9]>> {
        if num_pts == 0 {
            return Ok(vec![[0.0; 9]; 0]);
        }

        let device = &self.vulkan_context.device;
        let queue = &self.vulkan_context.queue;
        let memory_allocator = &self.vulkan_context.memory_allocator;
        let descriptor_set_allocator = &self.vulkan_context.descriptor_set_allocator;
        let command_buffer_allocator = &self.vulkan_context.command_buffer_allocator;
        let pipeline_layout = &self.pipeline_layout;
        let compute_pipeline = &self.compute_pipeline;

        let consts = PushConsts {
            num_points: voxel_gpu_context.num_points,
            table_size: voxel_gpu_context.table_size,
            voxel_size: voxel_gpu_context.voxel_size,
            _pad: 0,
        };

        if self.current_capacity_pts < num_pts {
            println!("Reallocating buffers for {} points", num_pts);

            let new_capacity = (num_pts as f64 * 1.5) as usize;
            self.current_capacity_pts = new_capacity;

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
                (new_capacity * 9) as u64,
            )
            .expect("Failed to create buf_out_pts buffer")
            .into();

            self.staging_buf_output_covs = Some(
                Buffer::new_slice::<f32>(
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
                    (new_capacity * 9) as u64,
                )
                .expect("Failed to create staging_buf_output_covs buffer")
                .into()
            );
        }

        // input points
        // let flattened_pts: Vec<f32> = pts.iter().flat_map(|arr| arr.iter().copied()).collect();
        // self.d_buf_input_pts = Buffer::from_iter(
        //     memory_allocator.clone(),
        //     BufferCreateInfo {
        //         usage: BufferUsage::STORAGE_BUFFER,
        //         ..Default::default()
        //     },
        //     AllocationCreateInfo {
        //         memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
        //             | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
        //         ..Default::default()
        //     },
        //     flattened_pts,
        // )
        // .expect("Failed to create buf_pts buffer!")
        // .into();

        let descriptor_set = DescriptorSet::new(
            descriptor_set_allocator.clone(),
            self.descriptor_set_layout.clone(),
            [
                vulkano::descriptor_set::WriteDescriptorSet::buffer(
                    0,
                    voxel_gpu_context.d_buf_out_pts.as_ref().unwrap().clone(),
                ),
                vulkano::descriptor_set::WriteDescriptorSet::buffer(
                    1,
                    voxel_gpu_context.d_buf_keys.as_ref().unwrap().clone(),
                ),
                vulkano::descriptor_set::WriteDescriptorSet::buffer(
                    2,
                    voxel_gpu_context.d_buf_centroids.as_ref().unwrap().clone(),
                ),
                vulkano::descriptor_set::WriteDescriptorSet::buffer(
                    3,
                    voxel_gpu_context.d_buf_counts.as_ref().unwrap().clone(),
                ),
                vulkano::descriptor_set::WriteDescriptorSet::buffer(
                    4,
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

        // Check if timestamps are supported
        let queue_family_props = device
            .physical_device()
            .queue_family_properties()
            .get(queue.queue_family_index() as usize)
            .unwrap();
        let timestamps_supported = queue_family_props
            .timestamp_valid_bits
            .map_or(false, |bits| bits > 0);

        let query_pool = if timestamps_supported {
            let mut create_info = QueryPoolCreateInfo::query_type(QueryType::Timestamp);
            create_info.query_count = 6;
            Some(QueryPool::new(device.clone(), create_info).unwrap())
        } else {
            None
        };

        unsafe {
            // if let Some(ref qp) = query_pool {
            //     command_buffer_builder
            //         .write_timestamp(qp.clone(), 0, sync::PipelineStage::ComputeShader)
            //         .unwrap();
            // }
            command_buffer_builder
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
                .unwrap();
            // if let Some(ref qp) = query_pool {
            //     command_buffer_builder
            //         .write_timestamp(qp.clone(), 1, sync::PipelineStage::ComputeShader)
            //         .unwrap();
            // }
            // command_buffer_builder
            //     .push_constants(pipeline_layout.clone(), 0, consts)
            //     .unwrap()
            //     .bind_descriptor_sets(
            //         vulkano::pipeline::PipelineBindPoint::Compute,
            //         pipeline_layout.clone(),
            //         0,
            //         descriptor_set.clone(),
            //     )
            //     .unwrap()
            //     .dispatch(work_group_count)
            //     .unwrap();
            // if let Some(ref qp) = query_pool {
            //     command_buffer_builder
            //         .write_timestamp(qp.clone(), 2, sync::PipelineStage::ComputeShader)
            //         .unwrap();
            // }
        }

        let copy_out_src = self.d_buf_output_covs.as_ref().unwrap().clone().slice(0..(num_pts * 9) as u64);
        let copy_out_dst = self.staging_buf_output_covs.as_ref().unwrap().clone().slice(0..(num_pts * 9) as u64);
        command_buffer_builder.copy_buffer(CopyBufferInfo::buffers(copy_out_src, copy_out_dst))?;

        let command_buffer = command_buffer_builder.build().unwrap();

        let compute_start_time = std::time::Instant::now();
        let future = sync::now(device.clone())
            .then_execute(queue.clone(), command_buffer)
            .unwrap()
            .then_signal_fence_and_flush()
            .unwrap();

        future.wait(None).unwrap();

        let compute_end_time = compute_start_time.elapsed();
        println!(
            "Compute covariance shader execution time: {:?}",
            compute_end_time
        );

        let out_pts_covs_content = self.staging_buf_output_covs.as_ref().unwrap().read()?;
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
