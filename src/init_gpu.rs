use anyhow::Result;
use std::sync::Arc;

use vulkano::{
    Version, VulkanLibrary,
    command_buffer::allocator::StandardCommandBufferAllocator,
    descriptor_set::allocator::StandardDescriptorSetAllocator,
    device::{
        Device, DeviceCreateInfo, DeviceExtensions, DeviceFeatures, Queue, QueueCreateInfo,
        QueueFlags, physical::PhysicalDevice,
    },
    instance::{Instance, InstanceCreateFlags, InstanceCreateInfo, InstanceExtensions},
    memory::allocator::{FreeListAllocator, GenericMemoryAllocator, StandardMemoryAllocator},
};

#[derive(Clone)]
pub struct VulkanContext {
    // _library: Arc<VulkanLibrary>,
    // instance: Arc<Instance>,
    // physical_device: Arc<PhysicalDevice>,
    pub device: Arc<Device>,
    pub queue: Arc<Queue>,
    pub memory_allocator: Arc<StandardMemoryAllocator>,
    pub command_buffer_allocator: Arc<StandardCommandBufferAllocator>,
    pub descriptor_set_allocator: Arc<StandardDescriptorSetAllocator>,
}

impl VulkanContext {
    pub fn new() -> Result<Self> {
        let library = VulkanLibrary::new().expect("Failed to load vulkan library");
        let mut required_extensions = InstanceExtensions::empty();

        if required_extensions.khr_get_physical_device_properties2 {
            required_extensions.khr_get_physical_device_properties2 = true;
        }
        let mut flags = InstanceCreateFlags::empty();
        if required_extensions.khr_portability_enumeration {
            required_extensions.khr_portability_enumeration = true;
            flags |= InstanceCreateFlags::ENUMERATE_PORTABILITY;
        }

        let instance = Instance::new(
            library,
            InstanceCreateInfo {
                enabled_extensions: required_extensions,
                flags,
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
                    .contains(QueueFlags::COMPUTE | QueueFlags::TRANSFER)
            })
            .expect("Could not find a compute queue family!")
            as u32;

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
            physical_device.clone(),
            DeviceCreateInfo {
                enabled_extensions: DeviceExtensions {
                    ext_shader_atomic_float: true,
                    #[cfg(target_os = "macos")]
                    khr_portability_subset: true,
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

        let command_buffer_allocator = Arc::new(StandardCommandBufferAllocator::new(
            device.clone(),
            Default::default(),
        ));

        let descriptor_set_allocator = Arc::new(StandardDescriptorSetAllocator::new(
            device.clone(),
            Default::default(),
        ));

        println!("=== Vulkan Device Information ===");
        println!("Device Name: {}", physical_device.properties().device_name);
        println!(
            "Device Type: {:?}",
            physical_device.properties().device_type
        );
        println!("Vulkan context initialized successfully.\n");

        Ok(VulkanContext {
            device,
            queue,
            memory_allocator,
            command_buffer_allocator,
            descriptor_set_allocator,
        })
    }
}
