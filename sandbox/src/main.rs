mod app;
mod window;

use winit::event_loop::EventLoop;

use crate::app::App;
use crate::window::Window;

fn main() {
    init_logger();

    let window = Window::new("Minecrust".into(), 1280, 720);

    let event_loop = EventLoop::new().unwrap();
    let mut app = App::new(window);
    let _ = event_loop.run_app(&mut app);
}

fn init_logger() {
    env_logger::Builder::new()
        .filter(Some("renderer"), log::LevelFilter::Trace)
        .filter(Some("sandbox"), log::LevelFilter::Trace)
        .format(|buf, record| {
            use std::io::Write;
            let style = buf.default_level_style(record.level());
            let time = chrono::Local::now().format("%H:%M:%S%.3f");
            writeln!(
                buf,
                "{style}[{}] {} - {}{style:#}",
                time,
                record.target(),
                record.args()
            )
        })
        .init()
}
