#![feature(abi_vectorcall, portable_simd, c_variadic, decl_macro)]
#![allow(clippy::missing_safety_doc, unused, improper_ctypes_definitions)]

include!("../../src/float.rs");

#[cfg(target_arch = "x86_64")]
pub mod x64;
#[cfg(target_arch = "x86_64")]
pub use x64::*;

#[cfg(target_arch = "x86")]
pub mod x86;
#[cfg(target_arch = "x86")]
pub use x86::*;
