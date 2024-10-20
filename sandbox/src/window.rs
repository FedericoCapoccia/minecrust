use std::error::Error;

use winit::{dpi::LogicalSize, event_loop::ActiveEventLoop};

pub struct Window {
    pub handle: Option<winit::window::Window>,
    pub title: String,
    pub width: u32,
    pub height: u32,
}

impl Default for Window {
    fn default() -> Self {
        Self {
            handle: None,
            title: "Placeholder".to_owned(),
            width: 800,
            height: 600,
        }
    }
}

impl Window {
    pub fn new(title: String, width: u32, height: u32) -> Self {
        Self {
            handle: None,
            title,
            width,
            height,
        }
    }

    pub fn init_handle(&mut self, event_loop: &ActiveEventLoop) -> Result<(), Box<dyn Error>> {
        let size = LogicalSize::new(self.width, self.height);
        let window_attr = winit::window::Window::default_attributes()
            .with_inner_size(size)
            .with_title(self.title.clone());
        let handle = event_loop.create_window(window_attr)?;
        self.handle = Some(handle);
        Ok(())
    }

    pub fn has_handle(&self) -> bool {
        self.handle.is_some()
    }

    pub fn handle(&self) -> &winit::window::Window {
        if let Some(handle) = &self.handle {
            return handle;
        }
        panic!("Tried to retrieve an handle reference without first initialize it");
    }
}
