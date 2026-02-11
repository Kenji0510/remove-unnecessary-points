use std::sync::Arc;

use anyhow::Result;
use vulkano::{VulkanLibrary, command_buffer::allocator::StandardCommandBufferAllocator, descriptor_set::allocator::StandardDescriptorSetAllocator, device::{Device, DeviceCreateInfo, DeviceExtensions, DeviceFeatures, QueueCreateInfo, QueueFlags, physical::PhysicalDeviceType}, instance::{Instance, InstanceCreateFlags, InstanceCreateInfo}, memory::allocator::StandardMemoryAllocator};



pub fn voxelization(
    // pts: &[f32],
    // voxel_size: f32,
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

    Ok(())
}