use core::num;
use std::{any, clone, sync::Arc, time::Instant};

use anyhow::{Context, Result};
use vulkano::{
    Version, VulkanLibrary,
    buffer::{Buffer, BufferCreateInfo, BufferUsage},
    command_buffer::{
        AutoCommandBufferBuilder, CommandBufferUsage, CopyBufferInfo,
        allocator::{StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo},
    },
    descriptor_set::{
        DescriptorSet, WriteDescriptorSet, allocator::StandardDescriptorSetAllocator,
    },
    device::{
        Device, DeviceCreateInfo, DeviceExtensions, DeviceFeatures, QueueCreateInfo, QueueFlags,
    },
    instance::{Instance, InstanceCreateFlags, InstanceCreateInfo, InstanceExtensions},
    memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator},
    pipeline::{
        ComputePipeline, Pipeline, PipelineLayout, PipelineShaderStageCreateInfo,
        compute::ComputePipelineCreateInfo, layout::PipelineDescriptorSetLayoutCreateInfo,
    },
    query::{QueryPool, QueryPoolCreateInfo, QueryType},
    sync::{self, GpuFuture},
};

#[derive(bytemuck::Pod, bytemuck::Zeroable, Clone, Copy)]
#[repr(C)]
struct PushConsts {
    num_points: i32,
    table_size: i32,
    voxel_size: f32,
    _pad: i32,
}

pub fn voxelization(pts: &[[f32; 3]], voxel_size: f32) -> Result<Vec<[f32; 3]>> {
    let library = VulkanLibrary::new().expect("Failed to load vulkan library");
    let required_extensions = InstanceExtensions::empty();
    let instance = Instance::new(
        library,
        InstanceCreateInfo {
            enabled_extensions: required_extensions,
            flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
            max_api_version: Some(Version::V1_3),
            ..Default::default()
        },
    )
    .expect("Failed to create instance");

    let physical_device = instance
        .enumerate_physical_devices()
        .expect("Could not enumerate devices!")
        .next()
        .expect("No devices available");

    let queue_family_index = physical_device
        .queue_family_properties()
        .iter()
        .enumerate()
        .position(|(_, queue_family_properties)| {
            queue_family_properties
                .queue_flags
                .contains(QueueFlags::COMPUTE)
        })
        .expect("Could not find a compute queue family!") as u32;

    let supported_extensions = physical_device.supported_extensions();
    if !supported_extensions.ext_shader_atomic_float {
        eprintln!("Warning: Device does not support ext_shader_atomic_float extension!");
        anyhow::bail!("Device does not support required extension");
    }

    let features = physical_device.supported_features();
    if !features.shader_buffer_float32_atomic_add {
        eprintln!("Warning: Device does not support float32 atomic add!");
        anyhow::bail!("Device does not support float32 atomic add");
    }

    if !features.shader_shared_float32_atomic_add {
        eprintln!("Error: Device does not support shader_shared_float32_atomic_add!");
        anyhow::bail!("Missing required feature for shared memory atomics");
    }

    let (device, mut queues) = Device::new(
        physical_device,
        DeviceCreateInfo {
            enabled_extensions: DeviceExtensions {
                ext_shader_atomic_float: true,
                ..Default::default()
            },
            enabled_features: DeviceFeatures {
                shader_buffer_float32_atomic_add: true,
                shader_shared_float32_atomic_add: true,
                ..Default::default()
            },
            queue_create_infos: vec![QueueCreateInfo {
                queue_family_index,
                ..Default::default()
            }],
            ..Default::default()
        },
    )
    .expect("Failed to create device!");

    let queue = queues.next().unwrap();

    let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));

    let num_points = pts.len();
    let table_size = num_points * 4;
    let consts_data = PushConsts {
        num_points: num_points as i32,
        table_size: table_size as i32,
        voxel_size,
        _pad: 0,
    };

    // input points
    let flattened_pts: Vec<f32> = pts.iter().flat_map(|arr| arr.iter().copied()).collect();
    let buf_input_pts = Buffer::from_iter(
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
    .expect("Failed to create buf_pts buffer!");

    let buf_keys = Buffer::new_slice::<u32>(
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
    .context("Failed to create buf_keys buffer")?;

    let buf_centroids = Buffer::new_slice::<f32>(
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
    .context("Failed to create buf_centroids buffer")?;

    let buf_counts = Buffer::new_slice::<i32>(
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
    .context("Failed to create buf_counts buffer")?;

    // let buf_uniform = Buffer::from_data(
    //     memory_allocator.clone(),
    //     BufferCreateInfo {
    //         usage: BufferUsage::UNIFORM_BUFFER,
    //         ..Default::default()
    //     },
    //     AllocationCreateInfo {
    //         memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
    //             | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
    //         ..Default::default()
    //     },
    //     consts_data,
    // )
    // .expect("Failed to create buf_uniform buffer!");

    let buf_out_pts = Buffer::new_slice::<f32>(
        memory_allocator.clone(),
        BufferCreateInfo {
            usage: BufferUsage::STORAGE_BUFFER | BufferUsage::TRANSFER_SRC,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_DEVICE,
            ..Default::default()
        },
        (num_points * 3) as u64,
    )
    .context("Failed to create buf_out_pts buffer")?;

    let buf_counter = Buffer::new_slice::<i32>(
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
    .context("Failed to create buf_counter buffer")?;

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

    let shader_init = cs_init::load(device.clone()).expect("Failed to load shader!");
    let shader_insert = cs_insert::load(device.clone()).expect("Failed to load shader!");
    let shader_compact = cs_compact::load(device.clone()).expect("Failed to load shader!");

    let cs_init = shader_init.entry_point("main").unwrap();
    let cs_insert = shader_insert.entry_point("main").unwrap();
    let cs_compact = shader_compact.entry_point("main").unwrap();

    let stage_init = PipelineShaderStageCreateInfo::new(cs_init);
    let stage_insert = PipelineShaderStageCreateInfo::new(cs_insert);
    let stage_compact = PipelineShaderStageCreateInfo::new(cs_compact);

    let layout_init = PipelineLayout::new(
        device.clone(),
        PipelineDescriptorSetLayoutCreateInfo::from_stages([&stage_init])
            .into_pipeline_layout_create_info(device.clone())
            .expect("Failed to create pipeline layout"),
    )
    .expect("Failed to create pipeline layout");
    let layout_insert = PipelineLayout::new(
        device.clone(),
        PipelineDescriptorSetLayoutCreateInfo::from_stages([&stage_insert])
            .into_pipeline_layout_create_info(device.clone())
            .expect("Failed to create pipeline layout"),
    )
    .expect("Failed to create pipeline layout");
    let layout_compact = PipelineLayout::new(
        device.clone(),
        PipelineDescriptorSetLayoutCreateInfo::from_stages([&stage_compact])
            .into_pipeline_layout_create_info(device.clone())
            .expect("Failed to create pipeline layout"),
    )
    .expect("Failed to create pipeline layout");

    let compute_pipeline_init = ComputePipeline::new(
        device.clone(),
        None,
        ComputePipelineCreateInfo::stage_layout(stage_init, layout_init),
    )
    .expect("Failed to create compute pipeline init");
    let compute_pipeline_insert = ComputePipeline::new(
        device.clone(),
        None,
        ComputePipelineCreateInfo::stage_layout(stage_insert, layout_insert),
    )
    .expect("Failed to create compute pipeline insert");
    let compute_pipeline_compact = ComputePipeline::new(
        device.clone(),
        None,
        ComputePipelineCreateInfo::stage_layout(stage_compact, layout_compact),
    )
    .expect("Failed to create compute pipeline compact");

    let descriptor_set_allocator = Arc::new(StandardDescriptorSetAllocator::new(
        device.clone(),
        Default::default(),
    ));

    let pipeline_layout_init = compute_pipeline_init.layout();
    let pipeline_layout_insert = compute_pipeline_insert.layout();
    let pipeline_layout_compact = compute_pipeline_compact.layout();

    let descriptor_set_layout_init = pipeline_layout_init.set_layouts().get(0).unwrap();
    let descriptor_set_layout_insert = pipeline_layout_insert.set_layouts().get(0).unwrap();
    let descriptor_set_layout_compact = pipeline_layout_compact.set_layouts().get(0).unwrap();

    let descriptor_set_init = DescriptorSet::new(
        descriptor_set_allocator.clone(),
        descriptor_set_layout_init.clone(),
        [
            // WriteDescriptorSet::buffer(0, buf_input_pts.clone()),
            WriteDescriptorSet::buffer(1, buf_keys.clone()),
            WriteDescriptorSet::buffer(2, buf_centroids.clone()),
            WriteDescriptorSet::buffer(3, buf_counts.clone()),
            // WriteDescriptorSet::buffer(4, buf_out_pts.clone()),
            WriteDescriptorSet::buffer(5, buf_counter.clone()),
        ],
        [],
    )
    .expect("Failed to set descriptor_set_init");

    let descriptor_set_insert = DescriptorSet::new(
        descriptor_set_allocator.clone(),
        descriptor_set_layout_insert.clone(),
        [
            WriteDescriptorSet::buffer(0, buf_input_pts.clone()),
            WriteDescriptorSet::buffer(1, buf_keys.clone()),
            WriteDescriptorSet::buffer(2, buf_centroids.clone()),
            WriteDescriptorSet::buffer(3, buf_counts.clone()),
            // WriteDescriptorSet::buffer(4, buf_out_pts.clone()),
            // WriteDescriptorSet::buffer(5, buf_counter.clone()),
        ],
        [],
    )
    .expect("Failed to set descriptor_set_insert");

    let descriptor_set_compact = DescriptorSet::new(
        descriptor_set_allocator.clone(),
        descriptor_set_layout_compact.clone(),
        [
            // WriteDescriptorSet::buffer(0, buf_input_pts.clone()),
            WriteDescriptorSet::buffer(1, buf_keys.clone()),
            WriteDescriptorSet::buffer(2, buf_centroids.clone()),
            WriteDescriptorSet::buffer(3, buf_counts.clone()),
            WriteDescriptorSet::buffer(4, buf_out_pts.clone()),
            WriteDescriptorSet::buffer(5, buf_counter.clone()),
        ],
        [],
    )
    .expect("Failed to set descriptor_set_compact");

    let command_buffer_allocator = Arc::new(StandardCommandBufferAllocator::new(
        device.clone(),
        StandardCommandBufferAllocatorCreateInfo::default(),
    ));

    let mut command_buffer_builder = AutoCommandBufferBuilder::primary(
        command_buffer_allocator.clone(),
        queue.queue_family_index().clone(),
        CommandBufferUsage::OneTimeSubmit,
    )
    .unwrap();

    const LOCAL_SIZE: u32 = 256;
    let group_count_x = (table_size as u32 + LOCAL_SIZE - 1) / LOCAL_SIZE;
    let work_group_count = [group_count_x, 1, 1];

    let mut create_info = QueryPoolCreateInfo::query_type(QueryType::Timestamp);
    create_info.query_count = 6;

    let query_pool = QueryPool::new(device.clone(), create_info).unwrap();

    unsafe {
        command_buffer_builder
            .write_timestamp(query_pool.clone(), 0, sync::PipelineStage::ComputeShader)
            .unwrap()
            .write_timestamp(query_pool.clone(), 1, sync::PipelineStage::ComputeShader)
            .unwrap()
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
            .unwrap()
            .write_timestamp(query_pool.clone(), 2, sync::PipelineStage::ComputeShader)
            .unwrap()
            .bind_pipeline_compute(compute_pipeline_insert.clone())
            .unwrap()
            .push_constants(pipeline_layout_init.clone(), 0, consts_data)
            .unwrap()
            .bind_descriptor_sets(
                vulkano::pipeline::PipelineBindPoint::Compute,
                pipeline_layout_insert.clone(),
                0,
                descriptor_set_insert.clone(),
            )
            .unwrap()
            .dispatch(work_group_count)
            .unwrap()
            .write_timestamp(query_pool.clone(), 3, sync::PipelineStage::ComputeShader)
            .unwrap()
            .bind_pipeline_compute(compute_pipeline_compact.clone())
            .unwrap()
            .push_constants(pipeline_layout_init.clone(), 0, consts_data)
            .unwrap()
            .bind_descriptor_sets(
                vulkano::pipeline::PipelineBindPoint::Compute,
                pipeline_layout_compact.clone(),
                0,
                descriptor_set_compact.clone(),
            )
            .unwrap()
            .dispatch(work_group_count)
            .unwrap()
            .write_timestamp(query_pool.clone(), 4, sync::PipelineStage::ComputeShader)
            .unwrap();
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
    println!("Compute shader execution time: {:?}", compute_end_time);

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
        (num_points * 3) as u64,
    )?;

    let staging_counter = Buffer::new_slice::<i32>(
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
        buf_out_pts.clone(),
        staging_out_pts.clone(),
    ))?;
    copy_builder.copy_buffer(CopyBufferInfo::buffers(
        buf_counter.clone(),
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
