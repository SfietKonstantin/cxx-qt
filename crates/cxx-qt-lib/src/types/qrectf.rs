#[cfg(any(target_arch = "x86", target_arch = "arm"))]
mod qrectf32;
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
mod qrectf64;

#[cfg(any(target_arch = "x86", target_arch = "arm"))]
pub use qrectf32::*;

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
pub use qrectf64::*;
