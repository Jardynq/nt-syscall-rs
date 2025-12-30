#![allow(unused_imports)]
use crate::{shared, x86};

pub macro syscall {
    (@arg 0) => { "" },
    (@arg 1) => {
		concat!(
			x86::shellcode::encode!("mov eax, dword [ecx + 0x8]"),
			x86::shellcode::encode!("push eax"),
		)
    },
    (@arg 2) => {
		concat!(
			x86::shellcode::encode!("mov eax, dword [ecx + 0xc]"),
			x86::shellcode::encode!("push eax"),
			x86::shellcode::syscall!(@arg 1)
		)
    },
    (@arg 3) => {
		concat!(
			x86::shellcode::encode!("mov eax, dword [ecx + 0x10]"),
			x86::shellcode::encode!("push eax"),
			x86::shellcode::syscall!(@arg 2)
		)
    },
    (@arg 4) => {
		concat!(
			x86::shellcode::encode!("mov eax, dword [ecx + 0x14]"),
			x86::shellcode::encode!("push eax"),
			x86::shellcode::syscall!(@arg 3)
		)
    },
	(@arg 5) => {
		concat!(
			x86::shellcode::encode!("mov eax, dword [ecx + 0x18]"),
			x86::shellcode::encode!("push eax"),
			x86::shellcode::syscall!(@arg 4)
		)
	},
	(@arg 6) => {
		concat!(
			x86::shellcode::encode!("mov eax, dword [ecx + 0x1c]"),
			x86::shellcode::encode!("push eax"),
			x86::shellcode::syscall!(@arg 5)
		)
	},
	(@arg 7) => {
		concat!(
			x86::shellcode::encode!("mov eax, dword [ecx + 0x20]"),
			x86::shellcode::encode!("push eax"),
			x86::shellcode::syscall!(@arg 6)
		)
	},
	(@arg 8) => {
		concat!(
			x86::shellcode::encode!("mov eax, dword [ecx + 0x24]"),
			x86::shellcode::encode!("push eax"),
			x86::shellcode::syscall!(@arg 7)
		)
	},
	(@arg 9) => {
		concat!(
			x86::shellcode::encode!("mov eax, dword [ecx + 0x28]"),
			x86::shellcode::encode!("push eax"),
			x86::shellcode::syscall!(@arg 8)
		)
	},
	(@arg 10) => {
		concat!(
			x86::shellcode::encode!("mov eax, dword [ecx + 0x2c]"),
			x86::shellcode::encode!("push eax"),
			x86::shellcode::syscall!(@arg 9)
		)
	},
	(@arg 11) => {
		concat!(
			x86::shellcode::encode!("mov eax, dword [ecx + 0x30]"),
			x86::shellcode::encode!("push eax"),
			x86::shellcode::syscall!(@arg 10)
		)
	},
	(@arg 12) => {
		concat!(
			x86::shellcode::encode!("mov eax, dword [ecx + 0x34]"),
			x86::shellcode::encode!("push eax"),
			x86::shellcode::syscall!(@arg 11)
		)
	},
	(@arg 13) => {
		concat!(
			x86::shellcode::encode!("mov eax, dword [ecx + 0x38]"),
			x86::shellcode::encode!("push eax"),
			x86::shellcode::syscall!(@arg 12)
		)
	},

    ($count:tt) => {
		concat!(
            x86::shellcode::prologue(),
            x86::shellcode::syscall!(@arg $count),
            x86::shellcode::encode!(
				"mov eax, dword [ecx]",
				"mov edx, esp",
				"sub edx, 0x8",
				"call $0",
				"add dword [esp - 0x4], 0x6",
                "sysenter"),
			x86::shellcode::epilogue(),
			x86::shellcode::encode!(
                "mov edx, dword [ecx + 0x4]",
                "mov dword [edx], eax",
            ),
            x86::shellcode::next_args!(2),
            x86::shellcode::next_args!($count)
		)
	},
	($($args:tt)*) => {
		shared::count_args_helper!((x86::shellcode::syscall), (), $($args)* )
	},
}
