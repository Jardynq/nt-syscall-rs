mod callconv;
mod shellcode;

pub use crate::shared::args32 as args;
pub use callconv::*;
pub use const_asm::assemble32 as assemble;
pub use shellcode::*;
