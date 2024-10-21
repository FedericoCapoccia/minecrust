use ash::vk;

pub struct Surface {
    loader: ash::khr::surface::Instance,
    handle: vk::SurfaceKHR,
}

impl Surface {
    pub(in crate::core) fn new(
        loader: ash::khr::surface::Instance,
        handle: vk::SurfaceKHR,
    ) -> Self {
        Self { loader, handle }
    }

    pub fn support_presenting(
        &self,
        gpu: vk::PhysicalDevice,
        queue_index: u32,
    ) -> Result<bool, vk::Result> {
        unsafe {
            self.loader
                .get_physical_device_surface_support(gpu, queue_index, self.handle)
        }
    }
}

impl Drop for Surface {
    fn drop(&mut self) {
        log::trace!("Destroying SurfaceKHR");
        unsafe { self.loader.destroy_surface(self.handle, None) }
    }
}
