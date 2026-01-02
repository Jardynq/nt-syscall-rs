#![allow(unused_imports)]
use crate::x86;

// TODO test this on native x86 system
pub macro callconv_syscall {
    (@arg 0)  => { "" },
    (@arg 1)  => { x86::callconv_syscall!(@inner 1, 0) },
    (@arg 2)  => { x86::callconv_syscall!(@inner 2, 1) },
    (@arg 3)  => { x86::callconv_syscall!(@inner 3, 2) },
    (@arg 4)  => { x86::callconv_syscall!(@inner 4, 3) },
    (@arg 5)  => { x86::callconv_syscall!(@inner 5, 4) },
    (@arg 6)  => { x86::callconv_syscall!(@inner 6, 5) },
    (@arg 7)  => { x86::callconv_syscall!(@inner 7, 6) },
    (@arg 8)  => { x86::callconv_syscall!(@inner 8, 7) },
    (@arg 9)  => { x86::callconv_syscall!(@inner 9, 8) },
    (@arg 10) => { x86::callconv_syscall!(@inner 10, 9) },
    (@arg 11) => { x86::callconv_syscall!(@inner 11, 10) },
    (@arg 12) => { x86::callconv_syscall!(@inner 12, 11) },
    (@arg 13) => { x86::callconv_syscall!(@inner 13, 12) },
    (@arg 14) => { x86::callconv_syscall!(@inner 14, 13) },
    (@arg 15) => { x86::callconv_syscall!(@inner 15, 14) },
    (@arg 16) => { x86::callconv_syscall!(@inner 16, 15) },
    (@arg 17) => { x86::callconv_syscall!(@inner 17, 16) },
    (@arg 18) => { x86::callconv_syscall!(@inner 18, 17) },
    (@arg 19) => { x86::callconv_syscall!(@inner 19, 18) },
    (@arg 20) => { x86::callconv_syscall!(@inner 20, 19) },
    (@inner $count:tt, $prev:tt) => {
        concat!(
			x86::assemble!("mov eax, dword ptr [ecx + 0x4 * (" $count " - 1)]"),
			x86::assemble!("mov [esp + 0x4 * (" $count " - 1)], eax"),
            x86::callconv_syscall!(@arg $prev),
        )
    },

    (@ret) => {
        concat!(
            x86::assemble!("mov edx, dword ptr [ecx]"),
            x86::assemble!("mov dword ptr [edx], eax"),
        )
    },

    (@stack 0) => { "" },
    (@stack $count:tt) => { x86::assemble!("sub esp, 0x4 * " $count) },
}
