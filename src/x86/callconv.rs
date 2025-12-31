#![allow(unused_imports)]
use crate::{shared, x86};

pub macro callconv_syscall {
    (@ret) => {
        concat!(
            x86::encode!("mov edx, dword ptr [ecx]"),
            x86::encode!("mov dword ptr [edx], eax"),
        )
    },

    (@arg 0) => { "" },
    (@arg 1) => {
		concat!(
			x86::encode!("mov eax, dword ptr [ecx]"),
			x86::encode!("push eax"),
		)
    },
    (@arg 2) => {
		concat!(
			x86::encode!("mov eax, dword ptr [ecx + 0x4]"),
			x86::encode!("push eax"),
			x86::callconv_syscall!(@arg 1)
		)
    },
    (@arg 3) => {
		concat!(
			x86::encode!("mov eax, dword ptr [ecx + 0x8]"),
			x86::encode!("push eax"),
			x86::callconv_syscall!(@arg 2)
		)
    },
    (@arg 4) => {
		concat!(
			x86::encode!("mov eax, dword ptr [ecx + 0xc]"),
			x86::encode!("push eax"),
			x86::callconv_syscall!(@arg 3)
		)
    },
	(@arg 5) => {
		concat!(
			x86::encode!("mov eax, dword ptr [ecx + 0x10]"),
			x86::encode!("push eax"),
			x86::callconv_syscall!(@arg 4)
		)
	},
	(@arg 6) => {
		concat!(
			x86::encode!("mov eax, dword ptr [ecx + 0x14]"),
			x86::encode!("push eax"),
			x86::callconv_syscall!(@arg 5)
		)
	},
	(@arg 7) => {
		concat!(
			x86::encode!("mov eax, dword ptr [ecx + 0x18]"),
			x86::encode!("push eax"),
			x86::callconv_syscall!(@arg 6)
		)
	},
	(@arg 8) => {
		concat!(
			x86::encode!("mov eax, dword ptr [ecx + 0x1c]"),
			x86::encode!("push eax"),
			x86::callconv_syscall!(@arg 7)
		)
	},
	(@arg 9) => {
		concat!(
			x86::encode!("mov eax, dword ptr [ecx + 0x20]"),
			x86::encode!("push eax"),
			x86::callconv_syscall!(@arg 8)
		)
	},
	(@arg 10) => {
		concat!(
			x86::encode!("mov eax, dword ptr [ecx + 0x24]"),
			x86::encode!("push eax"),
			x86::callconv_syscall!(@arg 9)
		)
	},
	(@arg 11) => {
		concat!(
			x86::encode!("mov eax, dword ptr [ecx + 0x28]"),
			x86::encode!("push eax"),
			x86::callconv_syscall!(@arg 10)
		)
	},
	(@arg 12) => {
		concat!(
			x86::encode!("mov eax, dword ptr [ecx + 0x2c]"),
			x86::encode!("push eax"),
			x86::callconv_syscall!(@arg 11)
		)
	},
	(@arg 13) => {
		concat!(
			x86::encode!("mov eax, dword ptr [ecx + 0x30]"),
			x86::encode!("push eax"),
			x86::callconv_syscall!(@arg 12)
		)
	},
}
