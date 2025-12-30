#![allow(unused_imports)]
use crate::{shared, x64, x86};

pub macro enter_x86() {
    x64::assemble!(
        "call $0",
        "mov dword [rsp + 0x4], 0x23",
        "add dword [rsp], 0xd",
        "retf",
        "mov ax, ds",
        "mov ss, ax"
    )
}

pub macro next_args {
    (0) => { "" },
    (1) => { x64::assemble!("add rcx, 0x8") },
    (2) => { x64::assemble!("add rcx, 0x10") },
    (3) => { x64::assemble!("add rcx, 0x18") },
    (4) => { x64::assemble!("add rcx, 0x20") },
    (5) => { x64::assemble!("add rcx, 0x28") },
    (6) => { x64::assemble!("add rcx, 0x30") },
    (7) => { x64::assemble!("add rcx, 0x38") },
    (8) => { x64::assemble!("add rcx, 0x40") },
    (9) => { x64::assemble!("add rcx, 0x48") },
    (10) => { x64::assemble!("add rcx, 0x50") },
    (11) => { x64::assemble!("add rcx, 0x58") },
    (12) => { x64::assemble!("add rcx, 0x60") },
    (13) => { x64::assemble!("add rcx, 0x68") },
}

pub macro prologue() {
    concat!(
        x64::assemble!("push rbp"),
        x64::assemble!("push rcx"),
        x64::assemble!("mov rbp, rsp"),
        x64::assemble!("and rsp, -0x10")
    )
}
pub macro epilogue() {
    concat!(
        x64::assemble!("mov rsp, rbp"),
        x64::assemble!("pop rcx"),
        x64::assemble!("pop rbp")
    )
}

pub macro syscall($count:tt) {
    concat!(
        x64::prologue!(),
        x64::next_args!(2),
        x64::callconv_syscall!(@stack $count),
        x64::callconv_syscall!(@arg $count),
        x64::assemble!("mov rax, qword [rcx - 0x10]"),
        x64::assemble!("syscall"),
        x64::epilogue!(),
        x64::next_args!(1),
        x64::callconv_syscall!(@ret),
        x64::next_args!(1),
        x64::next_args!($count)
    )
}

pub macro get_cpu_mode() {
    concat!(
        x64::assemble!("mov ax, cs"),
        x64::assemble!("mov rdx, qword [rcx]"),
        x64::assemble!("mov qword [rdx], rax"),
        next_args!(1)
    )
}

pub macro peb_ptr() {
    concat!(
        x64::assemble!(
            "mov rax, gs:[0x60]",
            "mov rdx, qword [rcx]",
            "mov qword [rdx], rax"
        ),
        next_args!(1)
    )
}

pub macro teb_ptr() {
    concat!(
        x64::assemble!(
            "mov rax, gs:[0x30]",
            "mov rdx, qword [rcx]",
            "mov qword [rdx], rax"
        ),
        next_args!(1)
    )
}

pub macro memcopy() {
    concat!(
        x64::assemble!(
            "mov r8, rdi",
            "mov r9, rsi",
            "mov r10, rcx",
            "mov rdi, qword [rcx]",
            "mov rsi, qword [rcx + 0x8]",
            "mov rcx, qword [rcx + 0x10]",
            "cld",
            "rep movsb",
            "mov rcx, r10",
            "mov rsi, r9",
            "mov rdi, r8",
        ),
        x64::next_args!(3),
    )
}
pub macro memset() {
    concat!(
        x64::assemble!(
            "mov r8, rdi",
            "mov r9, rsi",
            "mov r10, rcx",
            "mov rdi, qword [rcx]",
            "mov al, byte [rcx + 0x8]",
            "mov rcx, qword [rcx + 0x10]",
            "cld",
            "rep stosb",
            "mov rcx, r10",
            "mov rsi, r9",
            "mov rdi, r8",
        ),
        x64::next_args!(3),
    )
}
pub macro memread_u64() {
    concat!(
        x64::assemble!(
            "mov rax, qword [rcx]",
            "mov rdx, qword [rcx + 0x8]",
            "mov rdx, qword [rdx]",
            "mov qword [rax], rdx",
        ),
        x64::next_args!(2),
    )
}
pub macro memread_u32() {
    concat!(
        x64::assemble!(
            "mov rax, qword [rcx]",
            "mov rdx, qword [rcx + 0x8]",
            "mov rdx, dword [rdx]",
            "mov dword [rax], rdx",
        ),
        x64::next_args!(2),
    )
}
pub macro memread_u16() {
    concat!(
        x64::assemble!(
            "mov rax, qword [rcx]",
            "mov rdx, qword [rcx + 0x8]",
            "mov rdx, word [rdx]",
            "mov word [rax], rdx",
        ),
        x64::next_args!(2),
    )
}
pub macro memread_u8() {
    concat!(
        x64::assemble!(
            "mov rax, qword [rcx]",
            "mov rdx, qword [rcx + 0x8]",
            "mov rdx, byte [rdx]",
            "mov byte [rax], rdx",
        ),
        x64::next_args!(2),
    )
}

pub macro jump() {
    concat!(
        x64::assemble!("mov rax, qword [rcx]"),
        x64::next_args!(1),
        x64::assemble!("jmp rax"),
    )
}

macro call_inner($conv:tt, $ret:tt, $count:tt) {
    concat!(
        x64::prologue!(),
        x64::assemble!("mov r11, qword [rcx]"),
        x64::next_args!(2),
        x64::$conv!(@arg $count),
        x64::assemble!("call r11"),
        x64::epilogue!(),
        x64::next_args!(1),
        x64::$conv!(@ret $ret),
        x64::next_args!(1),
        x64::next_args!($count),
    )
}
pub macro call_x64_win64_float($count:tt) {
    call_inner!(callconv_win64, float, $count)
}
pub macro call_x64_win64($count:tt) {
    call_inner!(callconv_win64, int, $count)
}
