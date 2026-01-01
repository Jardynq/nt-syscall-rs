#[allow(unused_imports)]
use crate::x64;

pub macro callconv_syscall {
    (@ret) => {
        concat!(
            x64::assemble!("mov rdx, qword ptr [rcx]"),
            x64::assemble!("mov qword ptr [rdx], rax")
        )
    },

    (@stack 0) => { x64::assemble!("sub rsp, 0x30") },
    (@stack 1) => { x64::assemble!("sub rsp, 0x30") },
    (@stack 2) => { x64::assemble!("sub rsp, 0x30") },
    (@stack 3) => { x64::assemble!("sub rsp, 0x30") },
    (@stack $count:tt) => { x64::assemble!("sub rsp, 0x30 + 0x10 * ((" $count "- 4) / 2)") },

    (@arg 0) => { "" },
    (@arg 1) => {
        x64::assemble!("mov r10, qword ptr [rcx]")
    },
    (@arg 2) => {
        concat!(
            x64::assemble!("mov rdx, qword ptr [rcx + 0x8]"),
            x64::callconv_syscall!(@arg 1)
        )
    },
    (@arg 3) => {
        concat!(
            x64::assemble!("mov r8, qword ptr [rcx + 0x10]"),
            x64::callconv_syscall!(@arg 2)
        )
    },
    (@arg 4) => {
        concat!(
            x64::assemble!("mov r9, qword ptr [rcx + 0x18]"),
            x64::callconv_syscall!(@arg 3)
        )
    },
    // TODO either make a prev macro to do $count - 1 or use recursion with tt munching
    /*(@arg $count:tt) => {
        concat!(
            x64::assemble!("mov rax, qword ptr [rcx + 0x20 + 8 * (" $count "- 5)]"),
            x64::assemble!("mov qword ptr [rsp + 0x28 + 8 * (" $count "- 5)], rax"),
            x64::callconv_syscall!(@arg $count - 1),
        )
    },*/

    (@arg 5) => {
        concat!(
            x64::assemble!("mov rax, qword ptr [rcx + 0x20]"),
            x64::assemble!("mov qword ptr [rsp + 0x28], rax"),
            x64::callconv_syscall!(@arg 4),
        )
    },
    (@arg 6) => {
        concat!(
            x64::assemble!("mov rax, qword ptr [rcx + 0x28]"),
            x64::assemble!("mov qword ptr [rsp + 0x30], rax"),
            x64::callconv_syscall!(@arg 5),
        )
    },
    (@arg 7) => {
        concat!(
            x64::assemble!("mov rax, qword ptr [rcx + 0x30]"),
            x64::assemble!("mov qword ptr [rsp + 0x38], rax"),
            x64::callconv_syscall!(@arg 6),
        )
    },
    (@arg 8) => {
        concat!(
            x64::assemble!("mov rax, qword ptr [rcx + 0x38]"),
            x64::assemble!("mov qword ptr [rsp + 0x40], rax"),
            x64::callconv_syscall!(@arg 7),
        )
    },
    (@arg 9) => {
        concat!(
            x64::assemble!("mov rax, qword ptr [rcx + 0x40]"),
            x64::assemble!("mov qword ptr [rsp + 0x48], rax"),
            x64::callconv_syscall!(@arg 8),
        )
    },
    (@arg 10) => {
        concat!(
            x64::assemble!("mov rax, qword ptr [rcx + 0x48]"),
            x64::assemble!("mov qword ptr [rsp + 0x50], rax"),
            x64::callconv_syscall!(@arg 9),
        )
    },
    (@arg 11) => {
        concat!(
            x64::assemble!("mov rax, qword ptr [rcx + 0x50]"),
            x64::assemble!("mov qword ptr [rsp + 0x58], rax"),
            x64::callconv_syscall!(@arg 10),
        )
    },
    (@arg 12) => {
        concat!(
            x64::assemble!("mov rax, qword ptr [rcx + 0x58]"),
            x64::assemble!("mov qword ptr [rsp + 0x60], rax"),
            x64::callconv_syscall!(@arg 11),
        )
    },
    (@arg 13) => {
        concat!(
            x64::assemble!("mov rax, qword ptr [rcx + 0x60]"),
            x64::assemble!("mov qword ptr [rsp + 0x68], rax"),
            x64::callconv_syscall!(@arg 12),
        )
    },
}

pub macro callconv_win64 {
    (@arg ($($count:tt)+), &mut u8      $(, $($tail:tt)* )?) => { x64::callconv_win64!(@arg ($($count)+), u64 $(, $($tail)* )?) },
    (@arg ($($count:tt)+), &u8          $(, $($tail:tt)* )?) => { x64::callconv_win64!(@arg ($($count)+), u64 $(, $($tail)* )?) },
    (@arg ($($count:tt)+), *mut u8      $(, $($tail:tt)* )?) => { x64::callconv_win64!(@arg ($($count)+), u64 $(, $($tail)* )?) },
    (@arg ($($count:tt)+), *const u8    $(, $($tail:tt)* )?) => { x64::callconv_win64!(@arg ($($count)+), u64 $(, $($tail)* )?) },
    (@arg (0), u8  $(, $($tail:tt)* )?) => {
        concat!(
            x64::callconv_win64!(@arg (1) $(, $($tail)* )?),
            x64::assemble!("mov cl, byte ptr [rcx]"),
        )
    },
    (@arg (1), u8  $(, $($tail:tt)* )?) => {
        concat!(
            x64::callconv_win64!(@arg (2) $(, $($tail)* )?),
            x64::assemble!("mov dl, byte ptr [rcx + 0x8]"),
        )
    },
    (@arg (2), u8  $(, $($tail:tt)* )?) => {
        concat!(
            x64::callconv_win64!(@arg (3) $(, $($tail)* )?),
            x64::assemble!("mov r8b, byte ptr [rcx + 0x10]"),
        )
    },
    (@arg (3), u8  $(, $($tail:tt)* )?) => {
        concat!(
            x64::callconv_win64!(@arg (4) $(, $($tail)* )?),
            x64::assemble!("mov r9b, byte ptr [rcx + 0x18]"),
        )
    },
    (@arg ($($count:tt)+), u8  $(, $($tail:tt)* )?) => {
        concat!(
            x64::callconv_win64!(@arg ($($count)+ + 1) $(, $($tail)* )?),
            x64::assemble!("mov al, byte ptr [rcx + 0x20 + 8 * (" $($count)+ "- 4)]"),
            x64::assemble!("mov byte ptr [rsp + 0x20 + 8 * (" $($count)+ "- 4)], al"),
        )
    },

    (@arg ($($count:tt)+), &mut u16     $(, $($tail:tt)* )?) => { x64::callconv_win64!(@arg ($($count)+), u64 $(, $($tail)* )?) },
    (@arg ($($count:tt)+), &u16         $(, $($tail:tt)* )?) => { x64::callconv_win64!(@arg ($($count)+), u64 $(, $($tail)* )?) },
    (@arg ($($count:tt)+), *mut u16     $(, $($tail:tt)* )?) => { x64::callconv_win64!(@arg ($($count)+), u64 $(, $($tail)* )?) },
    (@arg ($($count:tt)+), *const u16   $(, $($tail:tt)* )?) => { x64::callconv_win64!(@arg ($($count)+), u64 $(, $($tail)* )?) },
    (@arg (0), u16  $(, $($tail:tt)* )?) => {
        concat!(
            x64::callconv_win64!(@arg (1) $(, $($tail)* )?),
            x64::assemble!("mov cx, word ptr [rcx]"),
        )
    },
    (@arg (1), u16  $(, $($tail:tt)* )?) => {
        concat!(
            x64::callconv_win64!(@arg (2) $(, $($tail)* )?),
            x64::assemble!("mov dx, word ptr [rcx + 0x8]"),
        )
    },
    (@arg (2), u16  $(, $($tail:tt)* )?) => {
        concat!(
            x64::callconv_win64!(@arg (3) $(, $($tail)* )?),
            x64::assemble!("mov r8w, word ptr [rcx + 0x10]"),
        )
    },
    (@arg (3), u16  $(, $($tail:tt)* )?) => {
        concat!(
            x64::callconv_win64!(@arg (4) $(, $($tail)* )?),
            x64::assemble!("mov r9w, word ptr [rcx + 0x18]"),
        )
    },
    (@arg ($($count:tt)+), u16  $(, $($tail:tt)* )?) => {
        concat!(
            x64::callconv_win64!(@arg ($($count)+ + 1) $(, $($tail)* )?),
            x64::assemble!("mov ax, word ptr [rcx + 0x20 + 8 * (" $($count)+ "- 4)]"),
            x64::assemble!("mov word ptr [rsp + 0x20 + 8 * (" $($count)+ "- 4)], ax"),
        )
    },

    (@arg ($($count:tt)+), &mut u32     $(, $($tail:tt)* )?) => { x64::callconv_win64!(@arg ($($count)+), u64 $(, $($tail)* )?) },
    (@arg ($($count:tt)+), &u32         $(, $($tail:tt)* )?) => { x64::callconv_win64!(@arg ($($count)+), u64 $(, $($tail)* )?) },
    (@arg ($($count:tt)+), *mut u32     $(, $($tail:tt)* )?) => { x64::callconv_win64!(@arg ($($count)+), u64 $(, $($tail)* )?) },
    (@arg ($($count:tt)+), *const u32   $(, $($tail:tt)* )?) => { x64::callconv_win64!(@arg ($($count)+), u64 $(, $($tail)* )?) },
    (@arg (0), u32  $(, $($tail:tt)* )?) => {
        concat!(
            x64::callconv_win64!(@arg (1) $(, $($tail)* )?),
            x64::assemble!("mov ecx, dword ptr [rcx]"),
        )
    },
    (@arg (1), u32  $(, $($tail:tt)* )?) => {
        concat!(
            x64::callconv_win64!(@arg (2) $(, $($tail)* )?),
            x64::assemble!("mov edx, dword ptr [rcx + 0x8]"),
        )
    },
    (@arg (2), u32  $(, $($tail:tt)* )?) => {
        concat!(
            x64::callconv_win64!(@arg (3) $(, $($tail)* )?),
            x64::assemble!("mov r8d, dword ptr [rcx + 0x10]"),
        )
    },
    (@arg (3), u32  $(, $($tail:tt)* )?) => {
        concat!(
            x64::callconv_win64!(@arg (4) $(, $($tail)* )?),
            x64::assemble!("mov r9d, dword ptr [rcx + 0x18]"),
        )
    },
    (@arg ($($count:tt)+), u32  $(, $($tail:tt)* )?) => {
        concat!(
            x64::callconv_win64!(@arg ($($count)+ + 1) $(, $($tail)* )?),
            x64::assemble!("mov eax, dword ptr [rcx + 0x20 + 8 * (" $($count)+ "- 4)]"),
            x64::assemble!("mov dword ptr [rsp + 0x20 + 8 * (" $($count)+ "- 4)], eax"),
        )
    },

    (@arg ($($count:tt)+), &mut u64     $(, $($tail:tt)* )?) => { x64::callconv_win64!(@arg ($($count)+), u64 $(, $($tail)* )?) },
    (@arg ($($count:tt)+), &u64         $(, $($tail:tt)* )?) => { x64::callconv_win64!(@arg ($($count)+), u64 $(, $($tail)* )?) },
    (@arg ($($count:tt)+), *mut u64     $(, $($tail:tt)* )?) => { x64::callconv_win64!(@arg ($($count)+), u64 $(, $($tail)* )?) },
    (@arg ($($count:tt)+), *const u64   $(, $($tail:tt)* )?) => { x64::callconv_win64!(@arg ($($count)+), u64 $(, $($tail)* )?) },
    (@arg (0), u64  $(, $($tail:tt)* )?) => {
        concat!(
            x64::callconv_win64!(@arg (1) $(, $($tail)* )?),
            x64::assemble!("mov rcx, qword ptr [rcx]"),
        )
    },
    (@arg (1), u64  $(, $($tail:tt)* )?) => {
        concat!(
            x64::callconv_win64!(@arg (2) $(, $($tail)* )?),
            x64::assemble!("mov rdx, qword ptr [rcx + 0x8]"),
        )
    },
    (@arg (2), u64  $(, $($tail:tt)* )?) => {
        concat!(
            x64::callconv_win64!(@arg (3) $(, $($tail)* )?),
            x64::assemble!("mov r8, qword ptr [rcx + 0x10]"),
        )
    },
    (@arg (3), u64  $(, $($tail:tt)* )?) => {
        concat!(
            x64::callconv_win64!(@arg (4) $(, $($tail)* )?),
            x64::assemble!("mov r9, qword ptr [rcx + 0x18]"),
        )
    },
    (@arg ($($count:tt)+), u64  $(, $($tail:tt)* )?) => {
        concat!(
            x64::callconv_win64!(@arg ($($count)+ + 1) $(, $($tail)* )?),
            x64::assemble!("mov rax, qword ptr [rcx + 0x20 + 8 * (" $($count)+ "- 4)]"),
            x64::assemble!("mov qword ptr [rsp + 0x20 + 8 * (" $($count)+ "- 4)], rax"),
        )
    },

    (@arg ($($count:tt)+), &mut f32     $(, $($tail:tt)* )?) => { x64::callconv_win64!(@arg ($($count)+), u64 $(, $($tail)* )?) },
    (@arg ($($count:tt)+), &f32         $(, $($tail:tt)* )?) => { x64::callconv_win64!(@arg ($($count)+), u64 $(, $($tail)* )?) },
    (@arg ($($count:tt)+), *mut f32     $(, $($tail:tt)* )?) => { x64::callconv_win64!(@arg ($($count)+), u64 $(, $($tail)* )?) },
    (@arg ($($count:tt)+), *const f32   $(, $($tail:tt)* )?) => { x64::callconv_win64!(@arg ($($count)+), u64 $(, $($tail)* )?) },
    (@arg (0), f32  $(, $($tail:tt)* )?) => {
        concat!(
            x64::callconv_win64!(@arg (1) $(, $($tail)* )?),
            x64::assemble!("movss xmm0, dword ptr [rcx]"),
        )
    },
    (@arg (1), f32  $(, $($tail:tt)* )?) => {
        concat!(
            x64::callconv_win64!(@arg (2) $(, $($tail)* )?),
            x64::assemble!("movss xmm1, dword ptr [rcx + 0x8]"),
        )
    },
    (@arg (2), f32  $(, $($tail:tt)* )?) => {
        concat!(
            x64::callconv_win64!(@arg (3) $(, $($tail)* )?),
            x64::assemble!("movss xmm2, dword ptr [rcx + 0x10]"),
        )
    },
    (@arg (3), f32  $(, $($tail:tt)* )?) => {
        concat!(
            x64::callconv_win64!(@arg (4) $(, $($tail)* )?),
            x64::assemble!("movss xmm3, dword ptr [rcx + 0x18]"),
        )
    },
    (@arg ($($count:tt)+), f32  $(, $($tail:tt)* )?) => {
        concat!(
            x64::callconv_win64!(@arg ($($count)+ + 1) $(, $($tail)* )?),
            x64::assemble!("mov eax, dword ptr [rcx + 0x20 + 8 * (" $($count)+ "- 4)]"),
            x64::assemble!("mov dword ptr [rsp + 0x20 + 8 * (" $($count)+ "- 4)], eax"),
        )
    },

    (@arg ($($count:tt)+), &mut f64     $(, $($tail:tt)* )?) => { x64::callconv_win64!(@arg ($($count)+), u64 $(, $($tail)* )?) },
    (@arg ($($count:tt)+), &f64         $(, $($tail:tt)* )?) => { x64::callconv_win64!(@arg ($($count)+), u64 $(, $($tail)* )?) },
    (@arg ($($count:tt)+), *mut f64     $(, $($tail:tt)* )?) => { x64::callconv_win64!(@arg ($($count)+), u64 $(, $($tail)* )?) },
    (@arg ($($count:tt)+), *const f64   $(, $($tail:tt)* )?) => { x64::callconv_win64!(@arg ($($count)+), u64 $(, $($tail)* )?) },
    (@arg (0), f64  $(, $($tail:tt)* )?) => {
        concat!(
            x64::callconv_win64!(@arg (1) $(, $($tail)* )?),
            x64::assemble!("movsd xmm0, qword ptr [rcx]"),
        )
    },
    (@arg (1), f64  $(, $($tail:tt)* )?) => {
        concat!(
            x64::callconv_win64!(@arg (2) $(, $($tail)* )?),
            x64::assemble!("movsd xmm1, qword ptr [rcx + 0x8]"),
        )
    },
    (@arg (2), f64  $(, $($tail:tt)* )?) => {
        concat!(
            x64::callconv_win64!(@arg (3) $(, $($tail)* )?),
            x64::assemble!("movsd xmm2, qword ptr [rcx + 0x10]"),
        )
    },
    (@arg (3), f64  $(, $($tail:tt)* )?) => {
        concat!(
            x64::callconv_win64!(@arg (4) $(, $($tail)* )?),
            x64::assemble!("movsd xmm3, qword ptr [rcx + 0x18]"),
        )
    },
    (@arg ($($count:tt)+), f64  $(, $($tail:tt)* )?) => {
        concat!(
            x64::callconv_win64!(@arg ($($count)+ + 1) $(, $($tail)* )?),
            x64::assemble!("mov rax, qword ptr [rcx + 0x20 + 8 * (" $($count)+ "- 4)]"),
            x64::assemble!("mov qword ptr [rsp + 0x20 + 8 * (" $($count)+ "- 4)], rax"),
        )
    },

    (@ret u8) => {
        concat!(
            x64::assemble!("mov rdx, qword ptr [rcx]"),
            x64::assemble!("mov byte ptr [rdx], al")
        )
    },
    (@ret u16) => {
        concat!(
            x64::assemble!("mov rdx, qword ptr [rcx]"),
            x64::assemble!("mov word ptr [rdx], ax")
        )
    },
    (@ret u32) => {
        concat!(
            x64::assemble!("mov rdx, qword ptr [rcx]"),
            x64::assemble!("mov dword ptr [rdx], eax")
        )
    },
    (@ret u64) => {
        concat!(
            x64::assemble!("mov rdx, qword ptr [rcx]"),
            x64::assemble!("mov qword ptr [rdx], rax")
        )
    },
    (@ret f32) => {
        concat!(
            x64::assemble!("mov rdx, qword ptr [rcx]"),
            x64::assemble!("movss dword ptr [rdx], xmm0")
        )
    },
    (@ret f64) => {
        concat!(
            x64::assemble!("mov rdx, qword ptr [rcx]"),
            x64::assemble!("movsd qword ptr [rdx], xmm0")
        )
    },

    (@arg ($($count:tt)+), &mut ()      $(, $($tail:tt)* )?) => { x64::callconv_win64!(@arg ($($count)+), u64 $(, $($tail)* )?) },
    (@arg ($($count:tt)+), &()          $(, $($tail:tt)* )?) => { x64::callconv_win64!(@arg ($($count)+), u64 $(, $($tail)* )?) },
    (@arg ($($count:tt)+), *mut ()      $(, $($tail:tt)* )?) => { x64::callconv_win64!(@arg ($($count)+), u64 $(, $($tail)* )?) },
    (@arg ($($count:tt)+), *const ()    $(, $($tail:tt)* )?) => { x64::callconv_win64!(@arg ($($count)+), u64 $(, $($tail)* )?) },

    (@arg ($($count:tt)+) $(,)?) => { x64::callconv_win64!(@stack ($($count)+)) },
    (@stack (0)) => { x64::assemble!("sub rsp, 0x30") },
    (@stack (1)) => { x64::assemble!("sub rsp, 0x30") },
    (@stack (2)) => { x64::assemble!("sub rsp, 0x30") },
    (@stack (3)) => { x64::assemble!("sub rsp, 0x30") },
    (@stack ($($count:tt)+)) => { x64::assemble!("sub rsp, 0x30 + 0x10 * ((" $($count)+ "- 4) / 2)") },

    (@arg ($($count:tt)+) $($tail:tt)*) => { compile_error!(concat!("Unexpected tokens: ", stringify!($($tail)*))) },
    ($($args:tt)*) => {
        x64::callconv_win64!(@arg (0), $($args)*)
    },
}
