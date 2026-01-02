#[allow(unused_imports)]
use crate::x64;

pub macro callconv_syscall {
    (@arg 0) => { "" },
    (@arg 1) => {
        x64::assemble!("mov r10, qword ptr [rcx]")
    },
    (@arg 2) => {
        concat!(
            x64::assemble!("mov rdx, qword ptr [rcx + 0x8]"),
            x64::callconv_syscall!(@arg 1)
        )
    },
    (@arg 3) => {
        concat!(
            x64::assemble!("mov r8, qword ptr [rcx + 0x10]"),
            x64::callconv_syscall!(@arg 2)
        )
    },
    (@arg 4) => {
        concat!(
            x64::assemble!("mov r9, qword ptr [rcx + 0x18]"),
            x64::callconv_syscall!(@arg 3)
        )
    },
    (@arg 5)  => { x64::callconv_syscall!(@inner 5, 4) },
    (@arg 6)  => { x64::callconv_syscall!(@inner 6, 5) },
    (@arg 7)  => { x64::callconv_syscall!(@inner 7, 6) },
    (@arg 8)  => { x64::callconv_syscall!(@inner 8, 7) },
    (@arg 9)  => { x64::callconv_syscall!(@inner 9, 8) },
    (@arg 10) => { x64::callconv_syscall!(@inner 10, 9) },
    (@arg 11) => { x64::callconv_syscall!(@inner 11, 10) },
    (@arg 12) => { x64::callconv_syscall!(@inner 12, 11) },
    (@arg 13) => { x64::callconv_syscall!(@inner 13, 12) },
    (@arg 14) => { x64::callconv_syscall!(@inner 14, 13) },
    (@arg 15) => { x64::callconv_syscall!(@inner 15, 14) },
    (@arg 16) => { x64::callconv_syscall!(@inner 16, 15) },
    (@arg 17) => { x64::callconv_syscall!(@inner 17, 16) },
    (@arg 18) => { x64::callconv_syscall!(@inner 18, 17) },
    (@arg 19) => { x64::callconv_syscall!(@inner 19, 18) },
    (@arg 20) => { x64::callconv_syscall!(@inner 20, 19) },
    (@inner $count:tt, $prev:tt) => {
        concat!(
            x64::assemble!("mov rax, qword ptr [rcx + 0x20 + 8 * (" $count "- 5)]"),
            x64::assemble!("mov qword ptr [rsp + 0x28 + 8 * (" $count "- 5)], rax"),
            x64::callconv_syscall!(@arg $prev),
        )
    },

    (@ret) => {
        concat!(
            x64::assemble!("mov rdx, qword ptr [rcx]"),
            x64::assemble!("mov qword ptr [rdx], rax")
        )
    },

    (@stack 0) => { x64::assemble!("sub rsp, 0x30") },
    (@stack 1) => { x64::assemble!("sub rsp, 0x30") },
    (@stack 2) => { x64::assemble!("sub rsp, 0x30") },
    (@stack 3) => { x64::assemble!("sub rsp, 0x30") },
    (@stack $count:tt) => { x64::assemble!("sub rsp, 0x30 + 0x10 * ((" $count "- 4) / 2)") },
}
