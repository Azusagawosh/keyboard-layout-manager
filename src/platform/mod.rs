#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "linux")]
pub mod linux;

pub use platform_impl::*;

#[cfg(target_os = "windows")]
mod platform_impl {
    pub use super::windows::*;
}

#[cfg(target_os = "linux")]
mod platform_impl {
    pub use super::linux::*;
}

#[cfg(not(any(target_os = "windows", target_os = "linux")))]
compile_error!("Unsupported OS");