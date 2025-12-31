#[allow(unused_imports)]
use crate::x64;

pub macro callconv_syscall {
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
    // TODO either make a prev macro to do $count - 1 or use recursion with tt munching
    /*(@arg $count:tt) => {
        concat!(
            x64::assemble!("mov rax, qword ptr [rcx + 0x20 + 8 * (" $count "- 5)]"),
            x64::assemble!("mov qword ptr [rsp + 0x28 + 8 * (" $count "- 5)], rax"),
            x64::callconv_syscall!(@arg $count - 1),
        )
    },*/

    (@arg 5) => {
        concat!(
            x64::assemble!("mov rax, qword ptr [rcx + 0x20]"),
            x64::assemble!("mov qword ptr [rsp + 0x28], rax"),
            x64::callconv_syscall!(@arg 4),
        )
    },
    (@arg 6) => {
        concat!(
            x64::assemble!("mov rax, qword ptr [rcx + 0x28]"),
            x64::assemble!("mov qword ptr [rsp + 0x30], rax"),
            x64::callconv_syscall!(@arg 5),
        )
    },
    (@arg 7) => {
        concat!(
            x64::assemble!("mov rax, qword ptr [rcx + 0x30]"),
            x64::assemble!("mov qword ptr [rsp + 0x38], rax"),
            x64::callconv_syscall!(@arg 6),
        )
    },
    (@arg 8) => {
        concat!(
            x64::assemble!("mov rax, qword ptr [rcx + 0x38]"),
            x64::assemble!("mov qword ptr [rsp + 0x40], rax"),
            x64::callconv_syscall!(@arg 7),
        )
    },
    (@arg 9) => {
        concat!(
            x64::assemble!("mov rax, qword ptr [rcx + 0x40]"),
            x64::assemble!("mov qword ptr [rsp + 0x48], rax"),
            x64::callconv_syscall!(@arg 8),
        )
    },
    (@arg 10) => {
        concat!(
            x64::assemble!("mov rax, qword ptr [rcx + 0x48]"),
            x64::assemble!("mov qword ptr [rsp + 0x50], rax"),
            x64::callconv_syscall!(@arg 9),
        )
    },
    (@arg 11) => {
        concat!(
            x64::assemble!("mov rax, qword ptr [rcx + 0x50]"),
            x64::assemble!("mov qword ptr [rsp + 0x58], rax"),
            x64::callconv_syscall!(@arg 10),
        )
    },
    (@arg 12) => {
        concat!(
            x64::assemble!("mov rax, qword ptr [rcx + 0x58]"),
            x64::assemble!("mov qword ptr [rsp + 0x60], rax"),
            x64::callconv_syscall!(@arg 11),
        )
    },
    (@arg 13) => {
        concat!(
            x64::assemble!("mov rax, qword ptr [rcx + 0x60]"),
            x64::assemble!("mov qword ptr [rsp + 0x68], rax"),
            x64::callconv_syscall!(@arg 12),
        )
    },
}

// TODO this does not support f32 since i use movsd for all float args/returns
// fixing would require a complex muncher or maybe just use a proc macro for once.
pub macro callconv_win64 {
    (@ret int) => {
        concat!(
            x64::assemble!("mov rdx, qword ptr [rcx]"),
            x64::assemble!("mov qword ptr [rdx], rax")
        )
    },
    (@ret float) => {
        concat!(
            x64::assemble!("mov rdx, qword ptr [rcx]"),
            x64::assemble!("movsd qword ptr [rdx], xmm0")
        )
    },

    (@stack 0) => { x64::assemble!("sub rsp, 0x30") },
    (@stack 1) => { x64::assemble!("sub rsp, 0x30") },
    (@stack 2) => { x64::assemble!("sub rsp, 0x30") },
    (@stack 3) => { x64::assemble!("sub rsp, 0x30") },
    (@stack $count:tt) => { x64::assemble!("sub rsp, 0x30 + 0x10 * ((" $count "- 4) / 2)") },


    (@arg 0) => { "" },
    (@arg 1) => {
        concat!(
            x64::assemble!("movsd xmm0, qword ptr [rcx]"),
            x64::assemble!("mov rcx, qword ptr [rcx]")
        )
    },
    (@arg 2) => {
        concat!(
            x64::assemble!("movsd xmm1, qword ptr [rcx + 0x8]"),
            x64::assemble!("mov rdx, qword ptr [rcx + 0x8]"),
            x64::callconv_win64!(@arg 1)
        )
    },
    (@arg 3) => {
        concat!(
            x64::assemble!("movsd xmm2, qword ptr [rcx + 0x10]"),
            x64::assemble!("mov r8, qword ptr [rcx + 0x10]"),
            x64::callconv_win64!(@arg 2)
        )
    },
    (@arg 4) => {
        concat!(
            x64::assemble!("movsd xmm3, qword ptr [rcx + 0x18]"),
            x64::assemble!("mov r9, qword ptr [rcx + 0x18]"),
            x64::callconv_win64!(@arg 3)
        )
    },
    (@arg 5) => {
        concat!(
            x64::assemble!("mov rax, qword ptr [rcx + 0x20]"),
            x64::assemble!("mov qword ptr [rsp + 0x20], rax"),
            x64::callconv_win64!(@arg 4),
        )
    },
    (@arg 6) => {
        concat!(
            x64::assemble!("mov rax, qword ptr [rcx + 0x28]"),
            x64::assemble!("mov qword ptr [rsp + 0x28], rax"),
            x64::callconv_win64!(@arg 5),
        )
    },
    (@arg 7) => {
        concat!(
            x64::assemble!("mov rax, qword ptr [rcx + 0x30]"),
            x64::assemble!("mov qword ptr [rsp + 0x30], rax"),
            x64::callconv_win64!(@arg 6),
        )
    },
    (@arg 8) => {
        concat!(
            x64::assemble!("mov rax, qword ptr [rcx + 0x38]"),
            x64::assemble!("mov qword ptr [rsp + 0x38], rax"),
            x64::callconv_win64!(@arg 7),
        )
    },
    (@arg 9) => {
        concat!(
            x64::assemble!("mov rax, qword ptr [rcx + 0x40]"),
            x64::assemble!("mov qword ptr [rsp + 0x40], rax"),
            x64::callconv_win64!(@arg 8),
        )
    },
    (@arg 10) => {
        concat!(
            x64::assemble!("mov rax, qword ptr [rcx + 0x48]"),
            x64::assemble!("mov qword ptr [rsp + 0x48], rax"),
            x64::callconv_win64!(@arg 9),
        )
    },
    (@arg 11) => {
        concat!(
            x64::assemble!("mov rax, qword ptr [rcx + 0x50]"),
            x64::assemble!("mov qword ptr [rsp + 0x50], rax"),
            x64::callconv_win64!(@arg 10),
        )
    },
    (@arg 12) => {
        concat!(
            x64::assemble!("mov rax, qword ptr [rcx + 0x58]"),
            x64::assemble!("mov qword ptr [rsp + 0x58], rax"),
            x64::callconv_win64!(@arg 11),
        )
    },
    (@arg 13) => {
        concat!(
            x64::assemble!("mov rax, qword ptr [rcx + 0x60]"),
            x64::assemble!("mov qword ptr [rsp + 0x60], rax"),
            x64::callconv_win64!(@arg 12),
        )
    },
}
