#![allow(unused_imports)]
use crate::{args, x64, x86};

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

/*
    Enter x86 mode on an x64 thread.
    The caller must ensure that:
        - The stack pointer is < u32::MAX, e.g. by allocating a low address buffer.
        - The instruction pointer is < u32::MAX, e.g. by using a low address trampoline.
        - The arguments ptr in rcx is < u32::MAX, e.g. by allocating a low address buffer.

    Return to x64 mode via x86::enter_x64().
*/

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

/*
    Enter x86 mode on an x64 thread.
    The caller must ensure that:
        - The instruction pointer is < u32::MAX, e.g. by using a low address trampoline.

    This handles setting up the stack and arguments for x86 mode.

    Return to x64 mode via x86::enter_x64_restore_stack().

    Arguments:
    0 [rcx + 0x00] - new stack pointer for x86 mode
    1 [rcx + 0x08] - new args pointer for x86 mode
*/
pub macro enter_x86_with_stack() {
    concat!(
        x64::prologue!(),
        x64::assemble!("mov rsp, qword ptr [rcx]"),
        x64::arg_next!(1),
        x64::arg_pop!(),
        x64::enter_x86!(),
    )
}

/*
    Arguments:
    0 [rcx + 0x00] - syscall id
    1 [rcx + 0x08] - return value pointer
    + Syscall arguments
*/
pub macro syscall($count:tt) {
    concat!(
        x64::prologue!(),
        x64::arg_next!(2),
        x64::callconv_syscall!(@stack $count),
        x64::callconv_syscall!(@arg $count),
        x64::assemble!("mov rax, qword ptr [rcx - 0x10]"),
        x64::assemble!("syscall"),
        x64::epilogue!(),
        x64::arg_next!(1),
        x64::callconv_syscall!(@ret),
        x64::arg_next!(@ 1 + $count)
    )
}

/*
    Arguments:
    0 [rcx + 0x00] - out pointer
*/
pub macro get_cpu_mode() {
    concat!(
        x64::assemble!("mov ax, cs"),
        x64::assemble!("mov rdx, qword ptr [rcx]"),
        x64::assemble!("mov word ptr [rdx], ax"),
        x64::arg_next!(1)
    )
}

/*
    Arguments:
    0 [rcx + 0x00] - out pointer
*/
pub macro peb_ptr() {
    concat!(
        x64::assemble!(
            "mov rax, qword ptr gs:[0x60]",
            "mov rdx, qword ptr [rcx]",
            "mov qword ptr [rdx], rax"
        ),
        x64::arg_next!(1)
    )
}

/*
    Arguments:
    0 [rcx + 0x00] - out pointer
*/
pub macro teb_ptr() {
    concat!(
        x64::assemble!(
            "mov rax, qword ptr gs:[0x30]",
            "mov rdx, qword ptr [rcx]",
            "mov qword ptr [rdx], rax"
        ),
        x64::arg_next!(1)
    )
}

/*
    Arguments:
    0 [rcx + 0x00] - destination pointer
    1 [rcx + 0x08] - source pointer
    2 [rcx + 0x10] - size in bytes
*/
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
        x64::arg_next!(3),
    )
}

/*
    Arguments:
    0 [rcx + 0x00] - destination pointer
    1 [rcx + 0x08] - value byte
    2 [rcx + 0x10] - size in bytes
*/
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
        x64::arg_next!(3),
    )
}

/*
    Arguments:
    0 [rcx + 0x00] - destination pointer
    1 [rcx + 0x08] - source pointer
*/
pub macro memread_u64() {
    concat!(
        x64::assemble!(
            "mov rax, qword ptr [rcx]",
            "mov rdx, qword ptr [rcx + 0x8]",
            "mov rdx, qword ptr [rdx]",
            "mov qword ptr [rax], rdx",
        ),
        x64::arg_next!(2),
    )
}

/*
    Arguments:
    0 [rcx + 0x00] - destination pointer
    1 [rcx + 0x08] - source pointer
*/
pub macro memread_u32() {
    concat!(
        x64::assemble!(
            "mov rax, qword ptr [rcx]",
            "mov rdx, qword ptr [rcx + 0x8]",
            "mov rdx, dword ptr [rdx]",
            "mov dword ptr [rax], rdx",
        ),
        x64::arg_next!(2),
    )
}

/*
    Arguments:
    0 [rcx + 0x00] - destination pointer
    1 [rcx + 0x08] - source pointer
*/
pub macro memread_u16() {
    concat!(
        x64::assemble!(
            "mov rax, qword ptr [rcx]",
            "mov rdx, qword ptr [rcx + 0x8]",
            "mov rdx, word ptr [rdx]",
            "mov word ptr [rax], rdx",
        ),
        x64::arg_next!(2),
    )
}

/*
    Arguments:
    0 [rcx + 0x00] - destination pointer
    1 [rcx + 0x08] - source pointer
*/
pub macro memread_u8() {
    concat!(
        x64::assemble!(
            "mov rax, qword ptr [rcx]",
            "mov rdx, qword ptr [rcx + 0x8]",
            "mov rdx, byte ptr [rdx]",
            "mov byte ptr [rax], rdx",
        ),
        x64::arg_next!(2),
    )
}

/*
    Arguments:
    0 [rcx + 0x00] - destination address
*/
pub macro jump() {
    concat!(
        x64::assemble!("mov rax, qword ptr [rcx]"),
        x64::arg_next!(1),
        x64::assemble!("jmp rax"),
    )
}

/*
    Arguments:
    0 [rcx + 0x00] - destination address
    1 [rcx + 0x08] - return value pointer
    + Function arguments
*/
macro call_inner($conv:tt, $ret:tt : $($arg:tt)*) {
    concat!(
        x64::prologue!(),
        x64::assemble!("mov r11, qword ptr [rcx]"),
        x64::arg_next!(2),
        x64::$conv!($($arg)*),
        x64::assemble!("call r11"),
        x64::epilogue!(),
        x64::arg_next!(1),
        x64::$conv!(@ret $ret),
        x64::arg_next!(1 + $($arg)*),
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
