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
}

impl Drop for Surface {
    fn drop(&mut self) {
        log::trace!("Destroying SurfaceKHR");
        unsafe { self.loader.destroy_surface(self.handle, None) }
    }
}
