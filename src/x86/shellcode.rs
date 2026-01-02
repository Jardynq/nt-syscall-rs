#![allow(unused_imports)]
use crate::{shared, x64, x86};

pub macro enter_x64() {
    concat!(
        x86::assemble!("push 0x33"),
        x86::assemble!("call +5"),
        x86::assemble!("add dword ptr [esp], 0x5"),
        x86::assemble!("retf"),
    )
}

pub macro prologue() {
    concat!(
        x86::assemble!("push ebp"),
        x86::assemble!("push ecx"),
        x86::assemble!("mov ebp, esp"),
    )
}
pub macro epilogue() {
    concat!(
        x86::assemble!("mov esp, ebp"),
        x86::assemble!("pop ecx"),
        x86::assemble!("pop ebp"),
    )
}

pub macro syscall($count:tt) {
    concat!(
        x86::prologue!(),
        x86::callconv_syscall!(@arg $count),
        x86::assemble!(
            "mov eax, dword ptr [ecx]",
            "mov edx, esp",
            "sub edx, 0x8",
            "call $0",
            "add dword ptr [esp - 0x4], 0x6",
            "sysenter"),
        x86::epilogue!(),
        x86::assemble!(
            "mov edx, dword ptr [ecx + 0x4]",
            "mov dword ptr [edx], eax",
        ),
        x86::next_args!(@ 2 + $count)
    )
}

pub macro get_cpu_mode() {
    concat!(
        x86::assemble!("mov ax, cs"),
        x86::assemble!("mov edx, dword ptr [ecx]"),
        x86::assemble!("mov dword ptr [edx], eax"),
        x86::next_args!(1),
    )
}

pub macro peb_ptr() {
    concat!(
        x86::assemble!("mov eax, fs:[0x30]"),
        x86::assemble!("mov edx, dword ptr [ecx]"),
        x86::assemble!("mov dword ptr [edx], eax"),
        x86::next_args!(1),
    )
}
pub macro teb_ptr() {
    concat!(
        x86::assemble!("mov eax, fs:[0x18]"),
        x86::assemble!("mov edx, dword ptr [ecx]"),
        x86::assemble!("mov dword ptr [edx], eax"),
        x86::next_args!(1),
    )
}

pub macro jump() {
    concat!(
        x86::assemble!("mov eax, dword ptr [ecx]"),
        x86::next_args!(1),
        x86::assemble!("jmp eax"),
    )
}

macro call_inner($conv:tt, $ret:tt : $($arg:tt)*) {
    concat!(
        x86::prologue!(),
        x86::assemble!("mov edx, dword ptr [ecx ]"),
        x86::next_args!(2),
        x86::$conv!($($arg)*),
        x86::assemble!("call edx"),
        x86::epilogue!(),
        x86::next_args!(1),
        x86::$conv!(@ret $ret),
        x86::next_args!(1 + $($arg)*),
    )
}
pub macro call_cdecl($ret:tt : $($arg:tt)*) {
    call_inner!(callconv_cdecl, $ret : $($arg)*)
}
pub macro call_fastcall($ret:tt : $($arg:tt)*) {
    call_inner!(callconv_fastcall, $ret : $($arg)*)
}
pub macro call_stdcall($ret:tt : $($arg:tt)*) {
    call_inner!(callconv_stdcall, $ret : $($arg)*)
}
pub macro call_thiscall($ret:tt : $($arg:tt)*) {
    call_inner!(callconv_thiscall, $ret : $($arg)*)
}
pub macro call_varargs($ret:tt : $($arg:tt)*) {
    call_inner!(callconv_varargs, $ret : $($arg)*)
}
pub macro call_vectorcall($ret:tt : $($arg:tt)*) {
    call_inner!(callconv_vectorcall, $ret : $($arg)*)
}
