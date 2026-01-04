mod callconv;
mod shellcode;

pub use callconv::*;
pub use const_asm::assemble64 as assemble;
pub use shellcode::*;

pub macro args($($args:tt)*) {
    crate::args::args!(u64, $($args)*)
}
pub macro next_args {
    (@ 0) => { "" },
    (@ $($count:tt)+) => { assemble!("add rcx, 8 * (" $($count)+ ")") },

    ($count:tt)                 => { next_args!(@ $count) },
    ($count:tt + $($arg:tt)*)   => { crate::args::count_types!(next_args ($count), $($arg)*)},
    ($($arg:tt)*)               => { crate::args::count_types!(next_args (0), $($arg)*)},
}
