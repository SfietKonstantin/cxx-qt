#[cfg(target_pointer_width = "32")]
mod qsizef32;
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
mod qsizef64;

#[cfg(target_pointer_width = "32")]
pub use qsizef32::*;

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
pub use qsizef64::*;
