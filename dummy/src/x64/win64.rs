use crate::*;

#[unsafe(no_mangle)]
#[unsafe(link_section = ".payload")]
pub extern "win64" fn win64_simple_u32() -> u32 {
    0x123
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".payload")]
pub extern "win64" fn win64_simple_f32() -> f64 {
    new_f64!(123.123)
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".payload")]
pub unsafe extern "win64" fn win64_complex(
    a1: u8,
    a2: u16,
    a3: u32,
    a4: u64,
    a5: f32,
    a6: f64,
    a7: &u32,
    a8: *mut u32,
) -> f32 {
    unsafe { *a8 = 0x123 };
    let a1 = cast_f64!(a1);
    let a2 = cast_f64!(a2);
    let a3 = cast_f64!(a3);
    let a4 = cast_f64!(a4);
    let a7 = cast_f64!(*a7);
    (a1 + a2 + a3 + a4 + a5 as f64 + a6 + a7) as f32
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".payload")]
pub unsafe extern "win64" fn win64_complex2(a1: f32, a2: f32, a3: f64, a4: f32, out: &mut f32) {
    *out = a1 + a2 + a3 as f32 + a4;
}
