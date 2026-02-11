use std::sync::Arc;

use anyhow::{Context, Result};
use vulkano::{VulkanLibrary, buffer::{Buffer, BufferCreateInfo, BufferUsage}, command_buffer::allocator::StandardCommandBufferAllocator, descriptor_set::allocator::StandardDescriptorSetAllocator, device::{Device, DeviceCreateInfo, DeviceExtensions, DeviceFeatures, QueueCreateInfo, QueueFlags, physical::PhysicalDeviceType}, instance::{Instance, InstanceCreateFlags, InstanceCreateInfo}, memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator}, pipeline::{PipelineLayout, PipelineShaderStageCreateInfo, layout::PipelineDescriptorSetLayoutCreateInfo}};


#[derive(bytemuck::Pod, bytemuck::Zeroable, Clone, Copy)]
#[repr(C)]
struct PushConsts {
    num_points: i32,
    table_size: i32,
    vixel_size: f32,
    _pad: i32
}

pub fn voxelization(
    pts: &[f32],
    voxel_size: f32,
) -> Result<()> {
    let library = VulkanLibrary::new()?;
    let instance = Instance::new(
        library,
        InstanceCreateInfo {
            flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
            ..Default::default()
        },
    )?;

    let device_extensions = DeviceExtensions {
        khr_storage_buffer_storage_class: true,
        ext_shader_atomic_float: true,
        ..DeviceExtensions::empty()
    };

    let (physical_device, queue_family_index) = instance
        .enumerate_physical_devices()?
        .filter(|p| p.supported_extensions().contains(&device_extensions))
        .filter_map(|p| {
            p.queue_family_properties()
                .iter()
                .position(|q| q.queue_flags.contains(QueueFlags::COMPUTE))
                .map(|i| (p, i as u32))
        })
        .min_by_key(|(p, _)| {
            match p.properties().device_type {
                PhysicalDeviceType::DiscreteGpu => 0,
                PhysicalDeviceType::IntegratedGpu => 1,
                _ => 2,
            }
        }).unwrap();

    println!("Device: {}", physical_device.properties().device_name);

    let features = DeviceFeatures {
        shader_buffer_float32_atomics: true,
        shader_shared_float32_atomics: true,
        ..DeviceFeatures::empty()
    };

    let (device, mut queues) = Device::new(
        physical_device,
        DeviceCreateInfo {
            enabled_extensions: device_extensions,
            enabled_features: features,
            queue_create_infos: vec![
                QueueCreateInfo {
                    queue_family_index,
                    ..Default::default()
                }
            ],
            ..Default::default()
        },
    )?;

    let queue = queues.next().unwrap();
    let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));
    let descriptor_set_allocator = StandardDescriptorSetAllocator::new(device.clone(), Default::default());
    let command_buffer_allocator = StandardCommandBufferAllocator::new(device.clone(), Default::default());

    let num_points = pts.len() as u32 / 4;
    let table_size = num_points * 3;

    let create_buffer = |size: u64, usage: BufferUsage, host_visible: bool| {
        Buffer::new_slice::<u8>(
            memory_allocator.clone(),
            BufferCreateInfo {
                usage,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: if host_visible {
                    MemoryTypeFilter::PREFER_HOST | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE
                } else {
                    MemoryTypeFilter::PREFER_DEVICE
                },
                ..Default::default()
            },
            size,
        )
        .expect("Failed to create buffer");
    };

    let input_buffer = Buffer::from_iter(
        memory_allocator.clone(), 
        BufferCreateInfo {
            usage: BufferUsage::STORAGE_BUFFER,
            ..Default::default()
        }, 
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_DEVICE | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
            ..Default::default()
        }, 
        pts.iter().cloned(),
    )
    .context("Failed to create input buffer")?;

    let buf_keys = create_buffer(
        (table_size * 4) as u64, BufferUsage::STORAGE_BUFFER, false);
    let buf_centroids = create_buffer((table_size * 3 * 4) as u64, BufferUsage::STORAGE_BUFFER, false);
    let buf_counts = create_buffer((table_size * 4) as u64, BufferUsage::STORAGE_BUFFER, false);

    let buf_out_points = create_buffer(
        (num_points * 3 * 4) as u64, BufferUsage::STORAGE_BUFFER, true
    );
    let buf_out_count = create_buffer(
        4, BufferUsage::STORAGE_BUFFER, true
    );

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

    let pc_data = PushConsts {
        num_points: num_points as i32,
        table_size: table_size as i32,
        vixel_size: voxel_size,
        _pad: 0
    };

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
    

    Ok(())
}