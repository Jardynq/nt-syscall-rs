#[allow(unused_imports)]
use crate::x64;

pub macro callconv_win64 {
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

    (@ret ()) => { "" },
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

    (@arg ($($count:tt)+), ptr $(, $($tail:tt)* )?) => { x64::callconv_win64!(@arg ($($count)+), u64 $(, $($tail)* )?) },

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
