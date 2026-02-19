use anyhow::{Context, Result};
use std::{sync::Arc, time::Instant};
use vulkano::{
    buffer::{Buffer, BufferCreateInfo, BufferUsage, Subbuffer},
    command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage, CopyBufferInfo},
    descriptor_set::{
        DescriptorSet, WriteDescriptorSet, allocator::StandardDescriptorSetAllocator,
        layout::DescriptorSetLayout,
    },
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
pub struct PushConsts {
    pub num_points: i32,
    pub table_size: i32,
    pub voxel_size: f32,
    pub _pad: i32,
}

pub struct VoxelGpuContext {
    vulkan_context: VulkanContext,

    compute_pipeline_init: Arc<ComputePipeline>,
    compute_pipeline_insert: Arc<ComputePipeline>,
    compute_pipeline_compact: Arc<ComputePipeline>,
    pipeline_layout_init: Arc<PipelineLayout>,
    pipeline_layout_insert: Arc<PipelineLayout>,
    pipeline_layout_compact: Arc<PipelineLayout>,
    descriptor_set_layout_init: Arc<DescriptorSetLayout>,
    descriptor_set_layout_insert: Arc<DescriptorSetLayout>,
    descriptor_set_layout_compact: Arc<DescriptorSetLayout>,

    staging_h_buf_input_pts: Option<Subbuffer<[f32]>>,
    pub d_buf_input_pts: Option<Subbuffer<[f32]>>,
    pub d_buf_keys: Option<Subbuffer<[u32]>>,
    pub d_buf_centroids: Option<Subbuffer<[u32]>>,
    pub d_buf_counts: Option<Subbuffer<[u32]>>,
    pub d_buf_out_pts: Option<Subbuffer<[f32]>>,
    pub d_buf_counter: Option<Subbuffer<[u32]>>,

    pub num_points: i32,
    pub table_size: i32,
    pub voxel_size: f32,
}

impl VoxelGpuContext {
    pub fn new(vulkan_context: VulkanContext) -> Result<Self> {
        mod cs_init {
            vulkano_shaders::shader! {
                ty: "compute",
                path: "src/kernels/voxelization/init.glsl",
            }
        }

        mod cs_insert {
            vulkano_shaders::shader! {
                ty: "compute",
                path: "src/kernels/voxelization/insert.glsl",
            }
        }

        mod cs_compact {
            vulkano_shaders::shader! {
                ty: "compute",
                path: "src/kernels/voxelization/compact.glsl",
            }
        }

        let shader_init =
            cs_init::load(vulkan_context.device.clone()).expect("Failed to load shader!");
        let shader_insert =
            cs_insert::load(vulkan_context.device.clone()).expect("Failed to load shader!");
        let shader_compact =
            cs_compact::load(vulkan_context.device.clone()).expect("Failed to load shader!");

        let cs_init = shader_init.entry_point("main").unwrap();
        let cs_insert = shader_insert.entry_point("main").unwrap();
        let cs_compact = shader_compact.entry_point("main").unwrap();

        let stage_init = PipelineShaderStageCreateInfo::new(cs_init);
        let stage_insert = PipelineShaderStageCreateInfo::new(cs_insert);
        let stage_compact = PipelineShaderStageCreateInfo::new(cs_compact);

        let layout_init = PipelineLayout::new(
            vulkan_context.device.clone(),
            PipelineDescriptorSetLayoutCreateInfo::from_stages([&stage_init])
                .into_pipeline_layout_create_info(vulkan_context.device.clone())
                .expect("Failed to create pipeline layout"),
        )
        .expect("Failed to create pipeline layout");
        let layout_insert = PipelineLayout::new(
            vulkan_context.device.clone(),
            PipelineDescriptorSetLayoutCreateInfo::from_stages([&stage_insert])
                .into_pipeline_layout_create_info(vulkan_context.device.clone())
                .expect("Failed to create pipeline layout"),
        )
        .expect("Failed to create pipeline layout");
        let layout_compact = PipelineLayout::new(
            vulkan_context.device.clone(),
            PipelineDescriptorSetLayoutCreateInfo::from_stages([&stage_compact])
                .into_pipeline_layout_create_info(vulkan_context.device.clone())
                .expect("Failed to create pipeline layout"),
        )
        .expect("Failed to create pipeline layout");

        let compute_pipeline_init = ComputePipeline::new(
            vulkan_context.device.clone(),
            None,
            ComputePipelineCreateInfo::stage_layout(stage_init, layout_init),
        )
        .expect("Failed to create compute pipeline init");
        let compute_pipeline_insert = ComputePipeline::new(
            vulkan_context.device.clone(),
            None,
            ComputePipelineCreateInfo::stage_layout(stage_insert, layout_insert),
        )
        .expect("Failed to create compute pipeline insert");
        let compute_pipeline_compact = ComputePipeline::new(
            vulkan_context.device.clone(),
            None,
            ComputePipelineCreateInfo::stage_layout(stage_compact, layout_compact),
        )
        .expect("Failed to create compute pipeline compact");

        let descriptor_set_allocator = Arc::new(StandardDescriptorSetAllocator::new(
            vulkan_context.device.clone(),
            Default::default(),
        ));

        let pipeline_layout_init = compute_pipeline_init.layout();
        let pipeline_layout_insert = compute_pipeline_insert.layout();
        let pipeline_layout_compact = compute_pipeline_compact.layout();

        let descriptor_set_layout_init = pipeline_layout_init.set_layouts().get(0).unwrap();
        let descriptor_set_layout_insert = pipeline_layout_insert.set_layouts().get(0).unwrap();
        let descriptor_set_layout_compact = pipeline_layout_compact.set_layouts().get(0).unwrap();

        Ok(Self {
            vulkan_context: vulkan_context.clone(),
            compute_pipeline_init: compute_pipeline_init.clone(),
            compute_pipeline_insert: compute_pipeline_insert.clone(),
            compute_pipeline_compact: compute_pipeline_compact.clone(),
            pipeline_layout_init: pipeline_layout_init.clone(),
            pipeline_layout_insert: pipeline_layout_insert.clone(),
            pipeline_layout_compact: pipeline_layout_compact.clone(),
            descriptor_set_layout_init: descriptor_set_layout_init.clone(),
            descriptor_set_layout_insert: descriptor_set_layout_insert.clone(),
            descriptor_set_layout_compact: descriptor_set_layout_compact.clone(),
            staging_h_buf_input_pts: None,
            d_buf_input_pts: None,
            d_buf_keys: None,
            d_buf_centroids: None,
            d_buf_counts: None,
            d_buf_out_pts: None,
            d_buf_counter: None,
            num_points: 0,
            table_size: 0,
            voxel_size: 0.0,
        })
    }

    pub fn voxelization(
        &mut self,
        pts: &[[f32; 3]],
        num_pts: usize,
        voxel_size: f32,
    ) -> Result<Vec<[f32; 3]>> {
        let device = &self.vulkan_context.device;
        let queue = &self.vulkan_context.queue;
        let memory_allocator = &self.vulkan_context.memory_allocator;
        let descriptor_set_allocator = &self.vulkan_context.descriptor_set_allocator;
        let command_buffer_allocator = &self.vulkan_context.command_buffer_allocator;
        let pipeline_layout_init = &self.pipeline_layout_init;
        let pipeline_layout_insert = &self.pipeline_layout_insert;
        let pipeline_layout_compact = &self.pipeline_layout_compact;
        let compute_pipeline_init = &self.compute_pipeline_init;
        let compute_pipeline_insert = &self.compute_pipeline_insert;
        let compute_pipeline_compact = &self.compute_pipeline_compact;

        let table_size = num_pts * 4;
        self.num_points = num_pts as i32;
        self.table_size = table_size as i32;
        self.voxel_size = voxel_size;
        let consts_data = PushConsts {
            num_points: self.num_points,
            table_size: self.table_size,
            voxel_size: self.voxel_size,
            _pad: 0,
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

        self.d_buf_keys = Buffer::new_slice::<u32>(
            memory_allocator.clone(),
            BufferCreateInfo {
                usage: BufferUsage::STORAGE_BUFFER | BufferUsage::TRANSFER_DST,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE,
                ..Default::default()
            },
            table_size as u64,
        )
        .expect("Failed to create buf_keys buffer")
        .into();

        self.d_buf_centroids = Buffer::new_slice::<u32>(
            memory_allocator.clone(),
            BufferCreateInfo {
                usage: BufferUsage::STORAGE_BUFFER | BufferUsage::TRANSFER_DST,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE,
                ..Default::default()
            },
            (table_size * 3) as u64,
        )
        .expect("Failed to create buf_centroids buffer")
        .into();

        self.d_buf_counts = Buffer::new_slice::<u32>(
            memory_allocator.clone(),
            BufferCreateInfo {
                usage: BufferUsage::STORAGE_BUFFER | BufferUsage::TRANSFER_DST,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE,
                ..Default::default()
            },
            table_size as u64,
        )
        .expect("Failed to create buf_counts buffer")
        .into();

        self.d_buf_out_pts = Buffer::new_slice::<f32>(
            memory_allocator.clone(),
            BufferCreateInfo {
                usage: BufferUsage::STORAGE_BUFFER | BufferUsage::TRANSFER_SRC,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE,
                ..Default::default()
            },
            (num_pts * 3) as u64,
        )
        .expect("Failed to create buf_out_pts buffer")
        .into();

        self.d_buf_counter = Buffer::new_slice::<u32>(
            memory_allocator.clone(),
            BufferCreateInfo {
                usage: BufferUsage::STORAGE_BUFFER | BufferUsage::TRANSFER_SRC,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE,
                ..Default::default()
            },
            1,
        )
        .expect("Failed to create buf_counter buffer")
        .into();

        let descriptor_set_init = DescriptorSet::new(
            descriptor_set_allocator.clone(),
            self.descriptor_set_layout_init.clone(),
            [
                // WriteDescriptorSet::buffer(0, buf_input_pts.clone()),
                WriteDescriptorSet::buffer(1, self.d_buf_keys.as_ref().unwrap().clone()),
                WriteDescriptorSet::buffer(2, self.d_buf_centroids.as_ref().unwrap().clone()),
                WriteDescriptorSet::buffer(3, self.d_buf_counts.as_ref().unwrap().clone()),
                // WriteDescriptorSet::buffer(4, buf_out_pts.clone()),
                WriteDescriptorSet::buffer(5, self.d_buf_counter.as_ref().unwrap().clone()),
            ],
            [],
        )
        .expect("Failed to set descriptor_set_init");

        let descriptor_set_insert = DescriptorSet::new(
            descriptor_set_allocator.clone(),
            self.descriptor_set_layout_insert.clone(),
            [
                WriteDescriptorSet::buffer(0, self.d_buf_input_pts.as_ref().unwrap().clone()),
                WriteDescriptorSet::buffer(1, self.d_buf_keys.as_ref().unwrap().clone()),
                WriteDescriptorSet::buffer(2, self.d_buf_centroids.as_ref().unwrap().clone()),
                WriteDescriptorSet::buffer(3, self.d_buf_counts.as_ref().unwrap().clone()),
                // WriteDescriptorSet::buffer(4, buf_out_pts.clone()),
                // WriteDescriptorSet::buffer(5, buf_counter.clone()),
            ],
            [],
        )
        .expect("Failed to set descriptor_set_insert");

        let descriptor_set_compact = DescriptorSet::new(
            descriptor_set_allocator.clone(),
            self.descriptor_set_layout_compact.clone(),
            [
                // WriteDescriptorSet::buffer(0, buf_input_pts.clone()),
                WriteDescriptorSet::buffer(1, self.d_buf_keys.as_ref().unwrap().clone()),
                WriteDescriptorSet::buffer(2, self.d_buf_centroids.as_ref().unwrap().clone()),
                WriteDescriptorSet::buffer(3, self.d_buf_counts.as_ref().unwrap().clone()),
                WriteDescriptorSet::buffer(4, self.d_buf_out_pts.as_ref().unwrap().clone()),
                WriteDescriptorSet::buffer(5, self.d_buf_counter.as_ref().unwrap().clone()),
            ],
            [],
        )
        .expect("Failed to set descriptor_set_compact");

        let mut command_buffer_builder = AutoCommandBufferBuilder::primary(
            command_buffer_allocator.clone(),
            queue.queue_family_index().clone(),
            CommandBufferUsage::OneTimeSubmit,
        )
        .unwrap();

        const LOCAL_SIZE: u32 = 256;
        let group_count_x = (table_size as u32 + LOCAL_SIZE - 1) / LOCAL_SIZE;
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
            //         .unwrap()
            //         .write_timestamp(qp.clone(), 1, sync::PipelineStage::ComputeShader)
            //         .unwrap();
            // }
            command_buffer_builder
                .bind_pipeline_compute(compute_pipeline_init.clone())
                .unwrap()
                .push_constants(pipeline_layout_init.clone(), 0, consts_data)
                .unwrap()
                .bind_descriptor_sets(
                    vulkano::pipeline::PipelineBindPoint::Compute,
                    pipeline_layout_init.clone(),
                    0,
                    descriptor_set_init.clone(),
                )
                .unwrap()
                .dispatch(work_group_count)
                .unwrap();
            // if let Some(ref qp) = query_pool {
            //     command_buffer_builder
            //         .write_timestamp(qp.clone(), 2, sync::PipelineStage::ComputeShader)
            //         .unwrap();
            // }
            command_buffer_builder
                .bind_pipeline_compute(compute_pipeline_insert.clone())
                .unwrap()
                .push_constants(pipeline_layout_insert.clone(), 0, consts_data)
                .unwrap()
                .bind_descriptor_sets(
                    vulkano::pipeline::PipelineBindPoint::Compute,
                    pipeline_layout_insert.clone(),
                    0,
                    descriptor_set_insert.clone(),
                )
                .unwrap()
                .dispatch(work_group_count)
                .unwrap();
            // if let Some(ref qp) = query_pool {
            //     command_buffer_builder
            //         .write_timestamp(qp.clone(), 3, sync::PipelineStage::ComputeShader)
            //         .unwrap();
            // }
            command_buffer_builder
                .bind_pipeline_compute(compute_pipeline_compact.clone())
                .unwrap()
                .push_constants(pipeline_layout_compact.clone(), 0, consts_data)
                .unwrap()
                .bind_descriptor_sets(
                    vulkano::pipeline::PipelineBindPoint::Compute,
                    pipeline_layout_compact.clone(),
                    0,
                    descriptor_set_compact.clone(),
                )
                .unwrap()
                .dispatch(work_group_count)
                .unwrap();
            // if let Some(ref qp) = query_pool {
            //     command_buffer_builder
            //         .write_timestamp(qp.clone(), 4, sync::PipelineStage::ComputeShader)
            //         .unwrap();
            // }
        }

        let command_buffer = command_buffer_builder.build().unwrap();

        let compute_start_time = Instant::now();
        let future = sync::now(device.clone())
            .then_execute(queue.clone(), command_buffer)
            .unwrap()
            .then_signal_fence_and_flush()
            .unwrap();

        future.wait(None).unwrap();

        let compute_end_time = compute_start_time.elapsed();
        println!("Compute voxelization shader execution time: {:?}", compute_end_time);

        let staging_out_pts = Buffer::new_slice::<f32>(
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
            (num_pts * 3) as u64,
        )?;

        let staging_counter = Buffer::new_slice::<u32>(
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
            1,
        )?;

        let mut copy_builder = AutoCommandBufferBuilder::primary(
            command_buffer_allocator.clone(),
            queue.queue_family_index().clone(),
            CommandBufferUsage::OneTimeSubmit,
        )?;

        copy_builder.copy_buffer(CopyBufferInfo::buffers(
            self.d_buf_out_pts.as_ref().unwrap().clone(),
            staging_out_pts.clone(),
        ))?;
        copy_builder.copy_buffer(CopyBufferInfo::buffers(
            self.d_buf_counter.as_ref().unwrap().clone(),
            staging_counter.clone(),
        ))?;

        let copy_command_buffer = copy_builder.build()?;

        let copy_future = sync::now(device.clone())
            .then_execute(queue.clone(), copy_command_buffer)?
            .then_signal_fence_and_flush()?;

        copy_future.wait(None)?;

        let counter_content = staging_counter.read()?;
        let num_output_points = counter_content[0] as usize;
        println!("Number of output points: {}", num_output_points);

        let out_pts_content = staging_out_pts.read()?;
        let output_points: Vec<[f32; 3]> = out_pts_content
            .chunks_exact(3)
            .take(num_output_points)
            .map(|chunk| chunk.try_into().unwrap())
            .collect();

        Ok(output_points)
    }
}
