#[macro_export]
macro_rules! core_trace {
    ($($arg:tt)+) => (log::trace!(target: "renderer", $($arg)+));
}

#[macro_export]
macro_rules! core_info {
    ($($arg:tt)+) => (log::info!(target: "renderer", $($arg)+));
}

#[macro_export]
macro_rules! core_warn {
    ($($arg:tt)+) => (log::warn!(target: "renderer", $($arg)+));
}

#[macro_export]
macro_rules! core_error {
    ($($arg:tt)+) => (log::error!(target: "renderer", $($arg)+));
}

#[macro_export]
macro_rules! core_fatal {
    ($($arg:tt)+) => {{
        log::error!(target: "renderer", $($arg)+);
        panic!($($arg)+);
    }};
}
