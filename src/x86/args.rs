#[allow(unused_imports)]
use crate::x86;

pub macro args($($args:tt)*) {
    crate::args::args!(u32, $($args)*)
}
pub macro args_in($buffer:expr, $($args:tt)*) {
    crate::args::args_in!(u32, $buffer, $($args)*)
}

pub macro arg_pop {
    ($index:tt) => { x86::assemble!("mov ecx, [ecx + 4 * (" $index ")]") },
    () => { x86::assemble!("mov ecx, [ecx]") },
}
pub macro arg_next {
    (@ 0) => { "" },
    (@ $($count:tt)+) => { x86::assemble!("add ecx, 4 * (" $($count)+ ")") },

    ($count:tt)                 => { arg_next!(@ $count) },
    ($count:tt + $($arg:tt)*)   => { crate::args::count_types!(arg_next ($count), $($arg)*)},
    ($($arg:tt)*)               => { crate::args::count_types!(arg_next (0), $($arg)*)},
}
pub macro arg_prev {
    (@ 0) => { "" },
    (@ $($count:tt)+) => { x86::assemble!("sub ecx, 4 * (" $($count)+ ")") },

    ($count:tt)                 => { arg_prev!(@ $count) },
    ($count:tt + $($arg:tt)*)   => { crate::args::count_types!(arg_prev ($count), $($arg)*)},
    ($($arg:tt)*)               => { crate::args::count_types!(arg_prev (0), $($arg)*)},
}

pub macro arg_load {
    ("eax", $index:expr) => { x64::assemble!("mov eax, dword ptr [rcx + 4 * (" $index ")]") },
    ("edx", $index:expr) => { x64::assemble!("mov edx, dword ptr [rcx + 4 * (" $index ")]") },
}
pub macro arg_store {
    ("eax", $index:expr) => { x64::assemble!("mov dword ptr [rcx + 4 * (" $index ")], eax") },
    ("edx", $index:expr) => { x64::assemble!("mov dword ptr [rcx + 4 * (" $index ")], edx") },
}
pub macro arg_swap($index_a:expr, $index_b:expr) {
    concat!(
        arg_load!("eax", $index_a),
        arg_load!("edx", $index_b),
        arg_store!("edx", $index_a),
        arg_store!("eax", $index_b),
    )
}
pub macro arg_copy($index_dst:expr, $index_src:expr) {
    concat!(arg_load!("eax", $index_src), arg_store!("eax", $index_dst),)
}
