pub mod varargs;
pub mod vectorcall;
pub mod win64;

#[macro_export]
macro_rules! new_f64 {
    ($val:expr) => {{
        let result: u64;
        const BITS: u64 = ($val as f64).to_bits();
        unsafe {
            core::arch::asm!(
                "",
                inout("rax") BITS => result,
            );
        }
        f64::from_bits(result)
    }};
}

#[macro_export]
macro_rules! cast_f64 {
    ($val:expr) => {{
        let mut result: f64;
        let val = $val as u64;
        unsafe {
            core::arch::asm!(
                "mov [rsp - 8], {input}",
                "fild qword ptr [rsp - 8]",
                "fstp qword ptr [rsp - 8]",
                "movsd {output}, [rsp - 8]",
                input = in(reg) val,
                output = out(xmm_reg) result,
            );
            result
        }
    }};
}
