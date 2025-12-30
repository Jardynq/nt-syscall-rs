#![allow(unused_imports)]
pub use super::encode::encode;
pub use super::syscall::syscall;
pub use crate::shared::args32 as args;
use crate::{shared, x64, x86};

pub macro next_args {
    (0) => { "" },
    (1) => { x86::shellcode::encode!("add ecx, 0x4") },
    (2) => { x86::shellcode::encode!("add ecx, 0x8") },
    (3) => { x86::shellcode::encode!("add ecx, 0xc") },
    (4) => { x86::shellcode::encode!("add ecx, 0x10") },
    (5) => { x86::shellcode::encode!("add ecx, 0x14") },
    (6) => { x86::shellcode::encode!("add ecx, 0x18") },
    (7) => { x86::shellcode::encode!("add ecx, 0x1c") },
    (8) => { x86::shellcode::encode!("add ecx, 0x20") },
    (9) => { x86::shellcode::encode!("add ecx, 0x24") },
    (10) => { x86::shellcode::encode!("add ecx, 0x28") },
    (11) => { x86::shellcode::encode!("add ecx, 0x2c") },
    (12) => { x86::shellcode::encode!("add ecx, 0x30") },
    (13) => { x86::shellcode::encode!("add ecx, 0x34") },
}

pub macro prologue() {
    concat!(
        x64::shellcode::encode!("push ebp"),
        x64::shellcode::encode!("push ecx"),
        x64::shellcode::encode!("mov ebp, esp")
    )
}
pub macro epilogue() {
    concat!(
        x64::shellcode::encode!("mov esp, ebp"),
        x64::shellcode::encode!("pop ecx"),
        x64::shellcode::encode!("pop ebp")
    )
}

pub macro enter_x64() {
    x86::shellcode::encode!(
        // Push x64 cpu mode to be written to cs by retf
        "push 0x33",
        "call $0",
        "add dword [esp], 0x5",
        "retf"
    )
}

pub macro get_cpu_mode() {
    concat!(
        x86::shellcode::encode!("mov ax, cs"),
        x86::shellcode::encode!("mov edx, dword [ecx]"),
        x86::shellcode::encode!("mov dword [edx], eax"),
        x86::shellcode::next_args!(1)
    )
}

pub macro peb_ptr() {
    concat!(
        x86::shellcode::encode!("mov eax, fs:[0x30]"),
        x86::shellcode::encode!("mov edx, dword [ecx]"),
        x86::shellcode::encode!("mov dword [edx], eax"),
        x86::shellcode::next_args!(1)
    )
}

pub macro teb_ptr() {
    concat!(
        x86::shellcode::encode!("mov eax, fs:[0x18]"),
        x86::shellcode::encode!("mov edx, dword [ecx]"),
        x86::shellcode::encode!("mov dword [edx], eax"),
        x86::shellcode::next_args!(1)
    )
}

pub macro memcopy_x64() {
    concat!(
        x86::shellcode::enter_x64!(),
        x64::shellcode::encode!(
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
        x64::shellcode::enter_x86!(),
        x86::shellcode::next_args!(3)
    )
}
pub macro memset_x64() {
    concat!(
        x86::shellcode::enter_x64!(),
        x64::shellcode::encode!(
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
        x64::shellcode::enter_x86!(),
        x86::shellcode::next_args!(3)
    )
}

pub macro jump_x64() {
    concat!(
        x86::shellcode::enter_x64!(),
        x64::shellcode::encode!("mov rax, qword [rcx]"),
        x64::shellcode::next_args!(1),
        x64::shellcode::encode!("jmp rax")
    )
}

// TODO: Careful with the count args helper since a single argument might be caught as a count
macro call_x64_inner {
    ($conv:tt, $ret:tt, $count:tt) => {
        concat!(
            x86::shellcode::enter_x64!(),
            x64::shellcode::prologue!(),
            x64::shellcode::encode!("mov r11, qword [rcx]"),
            x64::shellcode::next_args!(2),
            x64::shellcode::$conv!(@arg $count),
            x64::shellcode::encode!("call r11"),
            x64::shellcode::epilogue!(),
            x64::shellcode::next_args!(1),
            x64::shellcode::$conv!(@ret $ret),
            x64::shellcode::next_args!(1),
            x64::shellcode::next_args!($count),
            x64::shellcode::enter_x86!(),
        )
    },
    ($conv:tt, $ret:tt $(, $($args:tt)* )?) => {
		shared::count_args_helper!((x86::shellcode::call_x64_inner), ($conv, $ret,), $($($args)*)? )
	},
}

/*
    Calls an x64 function from x86 and returns in x86.
    The target function must follow Windows x64 calling convention,
    and return a floating point value in xmm0.
    Does not support vector (128+ bit) arguments or return values.
*/
pub macro call_x64_win64_float{
    ($($args:tt)*) => {
		x86::shellcode::call_x64_inner!(callconv_win64, float, $($args)*)
	},
}

/*
    Calls an x64 function from x86 and returns in x86.
    The target function must follow Windows x64 calling convention,
    and return an integer value in rax.
    Does not support vector (128+ bit) arguments or return values.
*/
pub macro call_x64_win64{
    ($($args:tt)*) => {
		x86::shellcode::call_x64_inner!(callconv_win64, int, $($args)*)
	},
}
