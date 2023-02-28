#[cfg(any(target_arch = "x86", target_arch = "arm"))]
mod qpointf32;
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
mod qpointf64;

#[cfg(any(target_arch = "x86", target_arch = "arm"))]
pub use qpointf32::*;

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
pub use qpointf64::*;
