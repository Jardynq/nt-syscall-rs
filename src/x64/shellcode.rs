#![allow(unused_imports)]
pub use super::callconv::*;
pub use super::encode::encode;
pub use crate::shared::args64 as args;
pub use crate::shared::decode;
use crate::{shared, x64, x86};

pub macro next_args {
    (0) => { "" },
    (1) => { x64::shellcode::encode!("add rcx, 0x8") },
    (2) => { x64::shellcode::encode!("add rcx, 0x10") },
    (3) => { x64::shellcode::encode!("add rcx, 0x18") },
    (4) => { x64::shellcode::encode!("add rcx, 0x20") },
    (5) => { x64::shellcode::encode!("add rcx, 0x28") },
    (6) => { x64::shellcode::encode!("add rcx, 0x30") },
    (7) => { x64::shellcode::encode!("add rcx, 0x38") },
    (8) => { x64::shellcode::encode!("add rcx, 0x40") },
    (9) => { x64::shellcode::encode!("add rcx, 0x48") },
    (10) => { x64::shellcode::encode!("add rcx, 0x50") },
    (11) => { x64::shellcode::encode!("add rcx, 0x58") },
    (12) => { x64::shellcode::encode!("add rcx, 0x60") },
    (13) => { x64::shellcode::encode!("add rcx, 0x68") },
}

pub macro prologue() {
    concat!(
        x64::shellcode::encode!("push rbp"),
        x64::shellcode::encode!("push rcx"),
        x64::shellcode::encode!("mov rbp, rsp"),
        x64::shellcode::encode!("and rsp, -0x10")
    )
}
pub macro epilogue() {
    concat!(
        x64::shellcode::encode!("mov rsp, rbp"),
        x64::shellcode::encode!("pop rcx"),
        x64::shellcode::encode!("pop rbp")
    )
}

pub macro syscall {
    ($count:tt) => {
		concat!(
            x64::shellcode::prologue!(),
            x64::shellcode::next_args!(2),
            x64::shellcode::callconv_syscall!(@stack $count),
            x64::shellcode::callconv_syscall!(@arg $count),
            x64::shellcode::encode!("mov rax, qword [rcx - 0x10]"),
            x64::shellcode::encode!("syscall"),
            x64::shellcode::epilogue!(),
            x64::shellcode::next_args!(1),
            x64::shellcode::callconv_syscall!(@ret),
            x64::shellcode::next_args!(1),
            x64::shellcode::next_args!($count)
		)
	},
    ($($args:tt)*) => {
		shared::count_args_helper!((x64::shellcode::syscall), (), $($args)* )
	},
}

pub macro enter_x86() {
    x64::shellcode::encode!(
        // Push x86 cpu mode to be written to cs by retf
        "call $0",
        "mov dword [rsp + 0x4], 0x23",
        "add dword [rsp], 0xd",
        "retf",
        // Solution for AMD specific race condition
        "mov ax, ds",
        "mov ss, ax"
    )
}

pub macro get_cpu_mode() {
    concat!(
        x64::shellcode::encode!("mov ax, cs"),
        x64::shellcode::encode!("mov rdx, qword [rcx]"),
        x64::shellcode::encode!("mov qword [rdx], rax"),
        next_args!(1)
    )
}

pub macro peb_ptr() {
    concat!(
        x64::shellcode::encode!(
            "mov rax, gs:[0x60]",
            "mov rdx, qword [rcx]",
            "mov qword [rdx], rax"
        ),
        next_args!(1)
    )
}

pub macro teb_ptr() {
    concat!(
        x64::shellcode::encode!(
            "mov rax, gs:[0x30]",
            "mov rdx, qword [rcx]",
            "mov qword [rdx], rax"
        ),
        next_args!(1)
    )
}

// TODO
pub macro naked_call_x86_ret_x64 {
    () => {
        concat!(
            x64::shellcode::naked_call_x86_ret_x64!(@start),
            // Handle call convention here
            x64::shellcode::naked_call_x86_ret_x64!(@middle),
            // Handle return convention here
            x64::shellcode::naked_call_x86_ret_x64!(@end)
        )
    },

    (@start) => {
        concat!(
            x64::shellcode::encode!("mov r11, rsp"),
            x64::shellcode::encode!("mov rsp, rdx"),
        )
    },
    (@middle) => {
        concat!(
            x64::shellcode::enter_x86!(),
            x86::shellcode::encode!("call ecx"),
            x86::shellcode::enter_x64!(),
            x64::shellcode::encode!("mov rsp, r11"),
        )
    },
    (@end) => {
        concat!(
            x64::shellcode::encode!("ret"),
        )
    },
}

// TODO
pub macro naked_x64_jump_x86() {
    core::arch::naked_asm!(
        shellcode::enter_x86!(),
        shellcode::encode!("mov esp, edx"),
        shellcode::encode!("jmp ecx"),
        shellcode::encode!("naked"),
    )
}
