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

    pub fn create_command_pool(
        &self,
        flags: vk::CommandPoolCreateFlags,
    ) -> Result<vk::CommandPool, vk::Result> {
        let info = vk::CommandPoolCreateInfo::default()
            .flags(flags)
            .queue_family_index(self.graphics_idx);

        unsafe { self.handle.create_command_pool(&info, None) }
    }

    pub fn allocate_command_buffer(
        &self,
        pool: vk::CommandPool,
        level: vk::CommandBufferLevel,
    ) -> Result<vk::CommandBuffer, vk::Result> {
        let alloc_info = vk::CommandBufferAllocateInfo::default()
            .level(level)
            .command_pool(pool)
            .command_buffer_count(1);

        let buffers = unsafe { self.handle.allocate_command_buffers(&alloc_info) }?;
        // TODO: support multiple buffers allocation in one go
        Ok(buffers[0])
    }

    pub fn create_semaphore(
        &self,
        flags: vk::SemaphoreCreateFlags,
    ) -> Result<vk::Semaphore, vk::Result> {
        let info = vk::SemaphoreCreateInfo::default().flags(flags);
        unsafe { self.handle.create_semaphore(&info, None) }
    }

    pub fn create_fence(&self, flags: vk::FenceCreateFlags) -> Result<vk::Fence, vk::Result> {
        let info = vk::FenceCreateInfo::default().flags(flags);
        unsafe { self.handle.create_fence(&info, None) }
    }

    pub fn wait_fence(&self, fence: vk::Fence, timeout: u64) -> Result<(), vk::Result> {
        unsafe { self.handle.wait_for_fences(&[fence], true, timeout) }
    }

    pub fn reset_fence(&self, fence: vk::Fence) -> Result<(), vk::Result> {
        unsafe { self.handle.reset_fences(&[fence]) }
    }

    pub fn destroy_command_pool(&self, pool: vk::CommandPool) {
        unsafe { self.handle.destroy_command_pool(pool, None) };
    }
    pub fn destroy_semaphore(&self, semaphore: vk::Semaphore) {
        unsafe { self.handle.destroy_semaphore(semaphore, None) };
    }

    pub fn destroy_fence(&self, fence: vk::Fence) {
        unsafe { self.handle.destroy_fence(fence, None) };
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        log::trace!("Destroying vulkan device");
        unsafe { self.handle.destroy_device(None) }
    }
}
