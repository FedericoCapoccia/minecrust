use ash::vk;

pub struct Device {
    gpu: vk::PhysicalDevice,
    handle: ash::Device,
    graphics: vk::Queue,
    graphics_idx: u32,
}

impl Device {
    pub(in crate::core) fn new(
        gpu: vk::PhysicalDevice,
        handle: ash::Device,
        graphics: vk::Queue,
        graphics_idx: u32,
    ) -> Self {
        Self {
            gpu,
            handle,
            graphics,
            graphics_idx,
        }
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        log::trace!("Destroying vulkan device");
        unsafe { self.handle.destroy_device(None) }
    }
}
