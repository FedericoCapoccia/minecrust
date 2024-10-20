mod utils;

/*
*NOTE:
* [] Create Instance
* [] Create Debug messenger
* [] Create Surface
* [] Select and create device
* [] Create Swapchain
* [] setup a double frame in flight system
* [] - [framedata, 2], each app loop end increment frame count
* [] - app can access sync structures and command buffer of the current usuable frame with a function
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

//use ash::vk;

#[allow(dead_code)]
pub struct Renderer {
    instance: utils::Instance,
}

impl Renderer {
    pub fn new() -> Self {
        let validation = true;

        let mut layers = vec![];
        let mut extensions = vec![];

        if validation {
            extensions.push(ash::ext::debug_utils::NAME.as_ptr());
            layers.push(c"VK_LAYER_KHRONOS_validation".as_ptr());
        }

        let instance_spec = utils::InstanceSpec {
            app_name: c"app".into(),
            extensions,
            layers,
            validation,
        };

        let instance = match utils::Instance::new(instance_spec) {
            Ok(val) => val,
            Err(err) => {
                log::error!("Instance creation error: {}", err);
                panic!("{}", err);
            }
        };

        Self { instance }
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {}
}
