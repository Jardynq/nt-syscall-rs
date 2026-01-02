#![allow(unused_imports)]
use crate::{shared, x64, x86};

pub macro enter_x86() {
    x64::assemble!(
        "call +5",
        "mov dword ptr [rsp + 0x4], 0x23",
        "add dword ptr [rsp], 0xd",
        "retf",
        "mov ax, ds",
        "mov ss, ax"
    )
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
        x64::assemble!("mov rax, qword ptr [rcx - 0x10]"),
        x64::assemble!("syscall"),
        x64::epilogue!(),
        x64::next_args!(1),
        x64::callconv_syscall!(@ret),
        x64::next_args!(@ 1 + $count)
    )
}

pub macro get_cpu_mode() {
    concat!(
        x64::assemble!("mov ax, cs"),
        x64::assemble!("mov rdx, qword ptr [rcx]"),
        x64::assemble!("mov qword ptr [rdx], rax"),
        x64::next_args!(1)
    )
}

pub macro peb_ptr() {
    concat!(
        x64::assemble!(
            "mov rax, gs:[0x60]",
            "mov rdx, qword ptr [rcx]",
            "mov qword ptr [rdx], rax"
        ),
        x64::next_args!(1)
    )
}
pub macro teb_ptr() {
    concat!(
        x64::assemble!(
            "mov rax, gs:[0x30]",
            "mov rdx, qword ptr [rcx]",
            "mov qword ptr [rdx], rax"
        ),
        x64::next_args!(1)
    )
}

pub macro memcopy() {
    concat!(
        x64::assemble!(
            "mov r8, rdi",
            "mov r9, rsi",
            "mov r10, rcx",
            "mov rdi, qword ptr [rcx]",
            "mov rsi, qword ptr [rcx + 0x8]",
            "mov rcx, qword ptr [rcx + 0x10]",
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
            "mov rdi, qword ptr [rcx]",
            "mov al, byte ptr [rcx + 0x8]",
            "mov rcx, qword ptr [rcx + 0x10]",
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
            "mov rax, qword ptr [rcx]",
            "mov rdx, qword ptr [rcx + 0x8]",
            "mov rdx, qword ptr [rdx]",
            "mov qword ptr [rax], rdx",
        ),
        x64::next_args!(2),
    )
}
pub macro memread_u32() {
    concat!(
        x64::assemble!(
            "mov rax, qword ptr [rcx]",
            "mov rdx, qword ptr [rcx + 0x8]",
            "mov rdx, dword ptr [rdx]",
            "mov dword ptr [rax], rdx",
        ),
        x64::next_args!(2),
    )
}
pub macro memread_u16() {
    concat!(
        x64::assemble!(
            "mov rax, qword ptr [rcx]",
            "mov rdx, qword ptr [rcx + 0x8]",
            "mov rdx, word ptr [rdx]",
            "mov word ptr [rax], rdx",
        ),
        x64::next_args!(2),
    )
}
pub macro memread_u8() {
    concat!(
        x64::assemble!(
            "mov rax, qword ptr [rcx]",
            "mov rdx, qword ptr [rcx + 0x8]",
            "mov rdx, byte ptr [rdx]",
            "mov byte ptr [rax], rdx",
        ),
        x64::next_args!(2),
    )
}

pub macro jump() {
    concat!(
        x64::assemble!("mov rax, qword ptr [rcx]"),
        x64::next_args!(1),
        x64::assemble!("jmp rax"),
    )
}

macro call_inner($conv:tt, $ret:tt : $($arg:tt)*) {
    concat!(
        x64::prologue!(),
        x64::assemble!("mov r11, qword ptr [rcx]"),
        x64::next_args!(2),
        x64::$conv!($($arg)*),
        x64::assemble!("call r11"),
        x64::epilogue!(),
        x64::next_args!(1),
        x64::$conv!(@ret $ret),
        x64::next_args!(1 + $($arg)*),
    )
}
pub macro call_varargs($ret:tt : $($arg:tt)*) {
    call_inner!(callconv_varargs, $ret : $($arg)*)
}
pub macro call_vectorcall($ret:tt : $($arg:tt)*) {
    call_inner!(callconv_vectorcall, $ret : $($arg)*)
}
pub macro call_win64($ret:tt : $($arg:tt)*) {
    call_inner!(callconv_win64, $ret : $($arg)*)
}
