#[cfg(target_arch = "x86_64")]
pub macro new_f64($val:expr) {{
    const BITS: u64 = ($val as f64).to_bits();
    let result: u64;
    unsafe {
        core::arch::asm!(
            "",
            inout("rax") BITS => result,
        );
    }
    f64::from_bits(result)
}}
#[cfg(target_arch = "x86_64")]
pub macro cast_f64($val:expr) {{
    let mut result: f64;
    let val = $val as u64;
    unsafe {
        core::arch::asm!(
            "cvtsi2sd {output}, {input}",
            input = in(reg) val,
            output = out(xmm_reg) result,
        );
        result
    }
}}
#[cfg(target_arch = "x86_64")]
pub macro cast_i64($val:expr) {{
    let mut result: i64;
    let val = $val as f64;
    unsafe {
        core::arch::asm!(
            "cvttsd2si {output}, {input}",
            input = in(xmm_reg) val,
            output = out(reg) result,
        );
        result
    }
}}

pub macro new_f32($val:expr) {{
    let result: u32;
    const BITS: u32 = ($val as f32).to_bits();
    unsafe {
        core::arch::asm!(
            "",
            inout("eax") BITS => result,
        );
    }
    f32::from_bits(result)
}}
pub macro cast_f32($val:expr) {{
    let mut result: f32;
    let val = $val as u32;
    unsafe {
        core::arch::asm!(
            "cvtsi2ss {output}, {input:e}",
            input = in(reg) val,
            output = out(xmm_reg) result,
        );
        result
    }
}}
pub macro cast_i32($val:expr) {{
    let mut result: i32;
    let val = $val as f32;
    unsafe {
        core::arch::asm!(
            "cvttss2si {output:e}, {input}",
            input = in(xmm_reg) val,
            output = out(reg) result,
        );
        result
    }
}}
