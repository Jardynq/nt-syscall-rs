#![allow(clippy::missing_safety_doc)]

#[cfg(target_arch = "x86_64")]
pub mod x64;
#[cfg(target_arch = "x86_64")]
pub use x64::*;

#[cfg(target_arch = "x86")]
pub mod x86;
#[cfg(target_arch = "x86")]
pub use x86::*;
