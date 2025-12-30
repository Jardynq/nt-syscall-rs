macro_rules! new_float {
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

macro_rules! cast_float {
    ($val:expr) => {{
        let mut result: f64;
        unsafe {
            core::arch::asm!(
                "mov [rsp - 8], {input}",
                "fild qword ptr [rsp - 8]",
                "fstp qword ptr [rsp - 8]",
                "movsd {output}, [rsp - 8]",
                input = in(reg) $val as u64,
                output = out(xmm_reg) result,
            );
            result
        }
    }};
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".payload")]
pub extern "win64" fn win64_simple_u32() -> u32 {
    0x123
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".payload")]
pub extern "win64" fn win64_simple_f32() -> f64 {
    new_float!(123.123)
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".payload")]
pub unsafe extern "win64" fn win64_complex(
    a1: u8,
    a2: u16,
    a3: u32,
    a4: u64,
    a5: f64, // TODO change back to f32 when support is added
    a6: f64,
    a7: &u32,
    a8: *mut u32,
) -> f64 {
    unsafe { *a8 = 0x123 };
    let a1 = cast_float!(a1);
    let a2 = cast_float!(a2);
    let a3 = cast_float!(a3);
    let a4 = cast_float!(a4);
    let a7 = cast_float!(*a7);
    a1 + a2 + a3 + a4 + a5 + a6 + a7
}
