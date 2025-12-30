#[allow(unused_imports)]
use crate::x64;

// TODO syscall and win64 are ver similar, can we merge some code?

pub macro callconv_syscall {
    (@ret) => {
        concat!(
            x64::shellcode::encode!("mov rdx, qword [rcx]"),
            x64::shellcode::encode!("mov qword [rdx], rax")
        )
    },

    (@stack 0) => { x64::shellcode::encode!("sub rsp, 0x30") },
    (@stack 1) => { x64::shellcode::encode!("sub rsp, 0x30") },
    (@stack 2) => { x64::shellcode::encode!("sub rsp, 0x30") },
    (@stack 3) => { x64::shellcode::encode!("sub rsp, 0x30") },
    (@stack 4) => { x64::shellcode::encode!("sub rsp, 0x30") },
    (@stack 5) => { x64::shellcode::encode!("sub rsp, 0x30") },
    (@stack 6) => { x64::shellcode::encode!("sub rsp, 0x40") },
    (@stack 7) => { x64::shellcode::encode!("sub rsp, 0x40") },
    (@stack 8) => { x64::shellcode::encode!("sub rsp, 0x50") },
    (@stack 9) => { x64::shellcode::encode!("sub rsp, 0x50") },
    (@stack 10) => { x64::shellcode::encode!("sub rsp, 0x60") },
    (@stack 11) => { x64::shellcode::encode!("sub rsp, 0x60") },
    (@stack 12) => { x64::shellcode::encode!("sub rsp, 0x70") },
    (@stack 13) => { x64::shellcode::encode!("sub rsp, 0x70") },

    (@arg 0) => { "" },
    (@arg 1) => {
        x64::shellcode::encode!("mov r10, qword [rcx]")
    },
    (@arg 2) => {
        concat!(
            x64::shellcode::encode!("mov rdx, qword [rcx + 0x8]"),
            x64::shellcode::callconv_syscall!(@arg 1)
        )
    },
    (@arg 3) => {
        concat!(
            x64::shellcode::encode!("mov r8, qword [rcx + 0x10]"),
            x64::shellcode::callconv_syscall!(@arg 2)
        )
    },
    (@arg 4) => {
        concat!(
            x64::shellcode::encode!("mov r9, qword [rcx + 0x18]"),
            x64::shellcode::callconv_syscall!(@arg 3)
        )
    },
    (@arg 5) => {
        concat!(
            x64::shellcode::encode!("mov rax, qword [rcx + 0x20]"),
            x64::shellcode::encode!("mov qword [rsp + 0x28], rax"),
            x64::shellcode::callconv_syscall!(@arg 4),
        )
    },
    (@arg 6) => {
        concat!(
            x64::shellcode::encode!("mov rax, qword [rcx + 0x28]"),
            x64::shellcode::encode!("mov qword [rsp + 0x30], rax"),
            x64::shellcode::callconv_syscall!(@arg 5),
        )
    },
    (@arg 7) => {
        concat!(
            x64::shellcode::encode!("mov rax, qword [rcx + 0x30]"),
            x64::shellcode::encode!("mov qword [rsp + 0x38], rax"),
            x64::shellcode::callconv_syscall!(@arg 6),
        )
    },
    (@arg 8) => {
        concat!(
            x64::shellcode::encode!("mov rax, qword [rcx + 0x38]"),
            x64::shellcode::encode!("mov qword [rsp + 0x40], rax"),
            x64::shellcode::callconv_syscall!(@arg 7),
        )
    },
    (@arg 9) => {
        concat!(
            x64::shellcode::encode!("mov rax, qword [rcx + 0x40]"),
            x64::shellcode::encode!("mov qword [rsp + 0x48], rax"),
            x64::shellcode::callconv_syscall!(@arg 8),
        )
    },
    (@arg 10) => {
        concat!(
            x64::shellcode::encode!("mov rax, qword [rcx + 0x48]"),
            x64::shellcode::encode!("mov qword [rsp + 0x50], rax"),
            x64::shellcode::callconv_syscall!(@arg 9),
        )
    },
    (@arg 11) => {
        concat!(
            x64::shellcode::encode!("mov rax, qword [rcx + 0x50]"),
            x64::shellcode::encode!("mov qword [rsp + 0x58], rax"),
            x64::shellcode::callconv_syscall!(@arg 10),
        )
    },
    (@arg 12) => {
        concat!(
            x64::shellcode::encode!("mov rax, qword [rcx + 0x58]"),
            x64::shellcode::encode!("mov qword [rsp + 0x60], rax"),
            x64::shellcode::callconv_syscall!(@arg 11),
        )
    },
    (@arg 13) => {
        concat!(
            x64::shellcode::encode!("mov rax, qword [rcx + 0x60]"),
            x64::shellcode::encode!("mov qword [rsp + 0x68], rax"),
            x64::shellcode::callconv_syscall!(@arg 12),
        )
    },
}

// TODO this does not support f32 since i use movsd for all float args/returns
// fixing would require a complex muncher or maybe just use a proc macro for once.
pub macro callconv_win64 {
    (@ret int) => {
        concat!(
            x64::shellcode::encode!("mov rdx, qword [rcx]"),
            x64::shellcode::encode!("mov qword [rdx], rax")
        )
    },
    (@ret float) => {
        concat!(
            x64::shellcode::encode!("mov rdx, qword [rcx]"),
            x64::shellcode::encode!("movsd qword [rdx], xmm0")
        )
    },


    (@stack 0) => { x64::shellcode::encode!("sub rsp, 0x30") },
    (@stack 1) => { x64::shellcode::encode!("sub rsp, 0x30") },
    (@stack 2) => { x64::shellcode::encode!("sub rsp, 0x30") },
    (@stack 3) => { x64::shellcode::encode!("sub rsp, 0x30") },
    (@stack 4) => { x64::shellcode::encode!("sub rsp, 0x30") },
    (@stack 5) => { x64::shellcode::encode!("sub rsp, 0x30") },
    (@stack 6) => { x64::shellcode::encode!("sub rsp, 0x40") },
    (@stack 7) => { x64::shellcode::encode!("sub rsp, 0x40") },
    (@stack 8) => { x64::shellcode::encode!("sub rsp, 0x50") },
    (@stack 9) => { x64::shellcode::encode!("sub rsp, 0x50") },
    (@stack 10) => { x64::shellcode::encode!("sub rsp, 0x60") },
    (@stack 11) => { x64::shellcode::encode!("sub rsp, 0x60") },
    (@stack 12) => { x64::shellcode::encode!("sub rsp, 0x70") },
    (@stack 13) => { x64::shellcode::encode!("sub rsp, 0x70") },

    (@arg 0) => { "" },
    (@arg 1) => {
        concat!(
            x64::shellcode::encode!("movsd xmm0, qword [rcx]"),
            x64::shellcode::encode!("mov rcx, qword [rcx]")
        )
    },
    (@arg 2) => {
        concat!(
            x64::shellcode::encode!("movsd xmm1, qword [rcx + 0x8]"),
            x64::shellcode::encode!("mov rdx, qword [rcx + 0x8]"),
            x64::shellcode::callconv_win64!(@arg 1)
        )
    },
    (@arg 3) => {
        concat!(
            x64::shellcode::encode!("movsd xmm2, qword [rcx + 0x10]"),
            x64::shellcode::encode!("mov r8, qword [rcx + 0x10]"),
            x64::shellcode::callconv_win64!(@arg 2)
        )
    },
    (@arg 4) => {
        concat!(
            x64::shellcode::encode!("movsd xmm3, qword [rcx + 0x18]"),
            x64::shellcode::encode!("mov r9, qword [rcx + 0x18]"),
            x64::shellcode::callconv_win64!(@arg 3)
        )
    },
    (@arg 5) => {
        concat!(
            x64::shellcode::encode!("mov rax, qword [rcx + 0x20]"),
            x64::shellcode::encode!("mov qword [rsp + 0x20], rax"),
            x64::shellcode::callconv_win64!(@arg 4),
        )
    },
    (@arg 6) => {
        concat!(
            x64::shellcode::encode!("mov rax, qword [rcx + 0x28]"),
            x64::shellcode::encode!("mov qword [rsp + 0x28], rax"),
            x64::shellcode::callconv_win64!(@arg 5),
        )
    },
    (@arg 7) => {
        concat!(
            x64::shellcode::encode!("mov rax, qword [rcx + 0x30]"),
            x64::shellcode::encode!("mov qword [rsp + 0x30], rax"),
            x64::shellcode::callconv_win64!(@arg 6),
        )
    },
    (@arg 8) => {
        concat!(
            x64::shellcode::encode!("mov rax, qword [rcx + 0x38]"),
            x64::shellcode::encode!("mov qword [rsp + 0x38], rax"),
            x64::shellcode::callconv_win64!(@arg 7),
        )
    },
    (@arg 9) => {
        concat!(
            x64::shellcode::encode!("mov rax, qword [rcx + 0x40]"),
            x64::shellcode::encode!("mov qword [rsp + 0x40], rax"),
            x64::shellcode::callconv_win64!(@arg 8),
        )
    },
    (@arg 10) => {
        concat!(
            x64::shellcode::encode!("mov rax, qword [rcx + 0x48]"),
            x64::shellcode::encode!("mov qword [rsp + 0x48], rax"),
            x64::shellcode::callconv_win64!(@arg 9),
        )
    },
    (@arg 11) => {
        concat!(
            x64::shellcode::encode!("mov rax, qword [rcx + 0x50]"),
            x64::shellcode::encode!("mov qword [rsp + 0x50], rax"),
            x64::shellcode::callconv_win64!(@arg 10),
        )
    },
    (@arg 12) => {
        concat!(
            x64::shellcode::encode!("mov rax, qword [rcx + 0x58]"),
            x64::shellcode::encode!("mov qword [rsp + 0x58], rax"),
            x64::shellcode::callconv_win64!(@arg 11),
        )
    },
    (@arg 13) => {
        concat!(
            x64::shellcode::encode!("mov rax, qword [rcx + 0x60]"),
            x64::shellcode::encode!("mov qword [rsp + 0x60], rax"),
            x64::shellcode::callconv_win64!(@arg 12),
        )
    },
}
