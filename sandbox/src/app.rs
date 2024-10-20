use std::ffi::CString;

use renderer::Renderer;
use winit::{application::ApplicationHandler, event::WindowEvent};

use crate::window::Window;

#[derive(Default)]
pub struct App {
    window: Window,
    renderer: Option<Renderer>,
}

impl App {
    pub fn new(window: Window) -> Self {
        Self {
            window,
            renderer: None,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if !self.window.has_handle() {
            if self.window.init_handle(event_loop).is_err() {
                log::error!("Failed to create winit window");
                panic!();
            }
            log::info!("Winit window created successfully");

            let app_name = CString::new(self.window.title.clone()).unwrap();
            let renderer = Renderer::new(app_name, true);
            self.renderer = Some(renderer);
            log::info!("Renderer created succesfully");
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        event_loop.exit(); // TODO: remove
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(_) => (),
            _ => (),
        }
    }
}
