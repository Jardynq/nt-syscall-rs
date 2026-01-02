mod callconv;
mod shellcode;

pub use callconv::*;
pub use const_asm::assemble32 as assemble;
pub use shellcode::*;

pub macro args($($args:tt)*) {
    crate::shared::args!(u32, $($args)*)
}
pub macro next_args {
    (@ 0) => { "" },
    (@ $($count:tt)+) => { assemble!("add ecx, 4 * (" $($count)+ ")") },

    ($count:tt)                 => { next_args!(@ $count) },
    ($count:tt + $($arg:tt)*)   => { crate::shared::count_types!(next_args ($count), $($arg)*)},
    ($($arg:tt)*)               => { crate::shared::count_types!(next_args (0), $($arg)*)},
}
