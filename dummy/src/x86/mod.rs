pub mod cdecl;
pub mod fastcall;
pub mod stdcall;
pub mod syscall;
pub mod thiscall;
pub mod varargs;
pub mod vectorcall;

#[macro_export]
macro_rules! new_f32 {
    ($val:expr) => {{
        let result: u32;
        const BITS: u32 = ($val as f32).to_bits();
        unsafe {
            core::arch::asm!(
                "",
                inout("eax") BITS => result,
            );
        }
        f32::from_bits(result)
    }};
}

#[macro_export]
macro_rules! cast_f32 {
    ($val:expr) => {{
        let mut result: f32;
        let val = $val as i64;
        unsafe {
            core::arch::asm!(
                "push {hi}",
                "push {lo}",
                "fild qword ptr [esp]",
                "fstp dword ptr [esp]",
                "movss {output}, [esp]",
                "add esp, 8",
                hi = in(reg) (val >> 32) as u32,
                lo = in(reg) (val & 0xFFFFFFFF) as u32,
                output = out(xmm_reg) result,
            );
            result
        }
    }};
}
