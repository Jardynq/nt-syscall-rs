#[allow(unused_imports)]
use crate::x64;

pub macro args($($args:tt)*) {
    crate::args::args!(u64, $($args)*)
}
pub macro args_in($buffer:expr, $($args:tt)*) {
    crate::args::args_in!(u64, $buffer, $($args)*)
}

pub macro arg_pop {
    ($index:expr) => { x64::assemble!("mov rcx, [rcx + 8 * (" $index ")]") },
    () => { x64::assemble!("mov rcx, [rcx]") },
}
pub macro arg_next {
    (@ 0) => { "" },
    (@ $($count:tt)+) => { x64::assemble!("add rcx, 8 * (" $($count)+ ")") },

    ($count:tt)                 => { arg_next!(@ $count) },
    ($count:tt + $($arg:tt)*)   => { crate::args::count_types!(arg_next ($count), $($arg)*)},
    ($($arg:tt)*)               => { crate::args::count_types!(arg_next (0), $($arg)*)},
}
pub macro arg_prev {
    (@ 0) => { "" },
    (@ $($count:tt)+) => { x64::assemble!("sub rcx, 8 * (" $($count)+ ")") },

    ($count:tt)                 => { arg_prev!(@ $count) },
    ($count:tt + $($arg:tt)*)   => { crate::args::count_types!(arg_prev ($count), $($arg)*)},
    ($($arg:tt)*)               => { crate::args::count_types!(arg_prev (0), $($arg)*)},
}

pub macro arg_load {
    ("rax", $index:expr) => { x64::assemble!("mov rax, qword ptr [rcx + 8 * (" $index ")]") },
    ("rdx", $index:expr) => { x64::assemble!("mov rdx, qword ptr [rcx + 8 * (" $index ")]") },
    ("r8", $index:expr) => { x64::assemble!("mov r8, qword ptr [rcx + 8 * (" $index ")]") },
    ("r9", $index:expr) => { x64::assemble!("mov r9, qword ptr [rcx + 8 * (" $index ")]") },
    ("r10", $index:expr) => { x64::assemble!("mov r10, qword ptr [rcx + 8 * (" $index ")]") },
    ("r11", $index:expr) => { x64::assemble!("mov r11, qword ptr [rcx + 8 * (" $index ")]") },
}
pub macro arg_store {
    ("rax", $index:expr) => { x64::assemble!("mov qword ptr [rcx + 8 * (" $index ")], rax") },
    ("rdx", $index:expr) => { x64::assemble!("mov qword ptr [rcx + 8 * (" $index ")], rdx") },
    ("r8", $index:expr) => { x64::assemble!("mov qword ptr [rcx + 8 * (" $index ")], r8") },
    ("r9", $index:expr) => { x64::assemble!("mov qword ptr [rcx + 8 * (" $index ")], r9") },
    ("r10", $index:expr) => { x64::assemble!("mov qword ptr [rcx + 8 * (" $index ")], r10") },
    ("r11", $index:expr) => { x64::assemble!("mov qword ptr [rcx + 8 * (" $index ")], r11") },
}
pub macro arg_swap($index_a:expr, $index_b:expr) {
    concat!(
        arg_load!("rax", $index_a),
        arg_load!("rdx", $index_b),
        arg_store!("rdx", $index_a),
        arg_store!("rax", $index_b),
    )
}
pub macro arg_copy($index_dst:expr, $index_src:expr) {
    concat!(arg_load!("rax", $index_src), arg_store!("rax", $index_dst),)
}
