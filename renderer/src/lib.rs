use std::ffi::CString;

use ash::{ext, khr, vk};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};

mod core;

/*
*NOTE:
* [] Create Swapchain
* [] expose allocated image to the app to be drawn on
* [] expose queue/swapchain functions to the app
* [] clear color
*
*NOTE:
* [] expose logical device handle to create pipelines
* [] load shaders
* [] try to render a triangle with hardcoded vertex in shader
* [] create a vertex buffer to draw a triangle
* [] implement index buffer
* [] draw a square
* [] implement a camera
* [] create a cube
*
*NOTE:
* -- the drawing stage inside the renderer would look like this if I understood
* -- 1. wait on current fence
* -- 2. acquire new swapchain image
* -- 3. start recording command buffer:
* --- 1a. reset
* --- 2a. begin
* --- 3a. transition image to a general format
* --- 4a. clear color
* --- 5a. transition image to color attachment format
* --- 6a. begin rendering
* --- 7a. somehow run a function from the game that takes the command buffer in use in the renderer,
* the pipeline created from the game (maybe register it with a function and store it in a hash map,
* so the client needs to pass a u64 key value), then it needs to somehow pass the chunk buffer to
* draw, (each chunk has his own vertex and index buffer)
* --- 8a. end rendering
* -- 4. end command buffer
* -- 5 submit to queue
* -- 6. present to swapcain
* -- 7 advance to next frame
* ```
* fn draw_frame(&mut self) {
        self.renderer.begin_frame(); // Begin the frame rendering process

        self.renderer.get_command_buffer();

        // Pass the renderer and draw the chunks
        for chunk in &self.chunks {
            // draw code WIP
            //self.renderer.draw_chunk(chunk);
        }

        self.renderer.end_frame();   // Submit command buffer and present the frame
    }
* ```
*/

// TODO: remove panics and unwrapping and move them to be handled in client code

const MAX_FRAMES_IN_FLIGHT: usize = 2;

// NOTE: rust calls Drop implementations in order of member declaration.
// This is stupid imho but it is what it is
#[allow(dead_code)]
pub struct Renderer {
    frame_number: usize,
    frames: [FrameData; MAX_FRAMES_IN_FLIGHT],
    device: core::Device,
    surface: core::Surface,
    instance: core::Instance,
}

struct FrameData {
    pub pool: vk::CommandPool,
    pub buffer: vk::CommandBuffer,

    pub swapchain_sem: vk::Semaphore,
    pub render_sem: vk::Semaphore,
    pub render_fen: vk::Fence,
}

impl Default for FrameData {
    fn default() -> Self {
        Self {
            pool: vk::CommandPool::null(),
            buffer: vk::CommandBuffer::null(),
            swapchain_sem: vk::Semaphore::null(),
            render_sem: vk::Semaphore::null(),
            render_fen: vk::Fence::null(),
        }
    }
}

impl Renderer {
    pub fn new<T>(window: &T, app_name: CString, validation: bool) -> Self
    where
        T: HasDisplayHandle + HasWindowHandle,
    {
        let mut layers = vec![];
        let rwh = window.display_handle().unwrap().as_raw();
        let mut extensions = ash_window::enumerate_required_extensions(rwh)
            .unwrap()
            .to_vec();

        if validation {
            extensions.push(ash::ext::debug_utils::NAME.as_ptr());
            layers.push(c"VK_LAYER_KHRONOS_validation".as_ptr());
        }

        let instance_spec = core::InstanceSpec {
            app_name,
            extensions,
            layers,
            validation,
        };

        let instance = match core::Instance::new(instance_spec) {
            Ok(val) => val,
            Err(err) => {
                log::error!("Instance creation error: {}", err);
                panic!("{}", err);
            }
        };
        log::info!("Vulkan instance created successfully");

        let surface = match instance.create_surface(window) {
            Ok(val) => val,
            Err(err) => {
                log::error!("Surface creation error: {}", err);
                panic!("{}", err);
            }
        };
        log::info!("Vulkan surface created successfully");

        let extensions = vec![
            khr::swapchain::NAME.as_ptr(),
            khr::dynamic_rendering::NAME.as_ptr(),
            khr::synchronization2::NAME.as_ptr(),
            khr::buffer_device_address::NAME.as_ptr(),
            ext::descriptor_indexing::NAME.as_ptr(),
        ];

        let (gpu, graphics_family_index) =
            match core::select_gpu(instance.handle(), &surface, &extensions) {
                Ok(val) => val,
                Err(err) => {
                    log::error!("GPU selection failed: {}", err);
                    panic!();
                }
            };

        let device = match instance.create_device(gpu, graphics_family_index, &extensions) {
            Ok(val) => val,
            Err(err) => {
                log::error!("Device creation failed: {}", err);
                panic!();
            }
        };
        log::info!("Device created succesfully");

        let frames = match Self::create_frames_structs(&device) {
            Ok(val) => val,
            Err(err) => {
                log::error!("Failed to initialize frames data: {}", err);
                panic!();
            }
        };

        Self {
            frame_number: 0,
            frames,
            instance,
            surface,
            device,
        }
    }

    fn create_frames_structs(device: &core::Device) -> Result<[FrameData; 2], vk::Result> {
        // Init frames data
        let mut frames: [FrameData; MAX_FRAMES_IN_FLIGHT] =
            [FrameData::default(), FrameData::default()];

        for frame in &mut frames {
            let pool =
                device.create_command_pool(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER)?;
            let buffer = device.allocate_command_buffer(pool, vk::CommandBufferLevel::PRIMARY)?;
            let swapchain_sem = device.create_semaphore(vk::SemaphoreCreateFlags::default())?;
            let render_sem = device.create_semaphore(vk::SemaphoreCreateFlags::default())?;
            let render_fen = device.create_fence(vk::FenceCreateFlags::SIGNALED)?;

            frame.pool = pool;
            frame.buffer = buffer;
            frame.render_fen = render_fen;
            frame.render_sem = render_sem;
            frame.swapchain_sem = swapchain_sem;
        }

        Ok(frames)
    }

    fn get_current_frame(&self) -> &FrameData {
        &self.frames[self.frame_number % MAX_FRAMES_IN_FLIGHT]
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        log::trace!("Destroying Renderer");
        self.device.wait_idle();

        for frame in &self.frames {
            self.device.destroy_command_pool(frame.pool);
            self.device.destroy_semaphore(frame.swapchain_sem);
            self.device.destroy_semaphore(frame.render_sem);
            self.device.destroy_fence(frame.render_fen);
        }
    }
}
