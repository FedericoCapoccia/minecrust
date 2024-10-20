fn main() {
    init_logger();
    let _renderer = renderer::Renderer::new();
}

fn init_logger() {
    env_logger::Builder::new()
        .filter(None, log::LevelFilter::Trace)
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
