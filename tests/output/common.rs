#[cfg(target_os = "linux")]
pub static PYTHON: &str = "python3";
#[cfg(target_os = "windows")]
pub static PYTHON: &str = "py";