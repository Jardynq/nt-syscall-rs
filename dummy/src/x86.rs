macro_rules! float {
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

use std::hint::black_box;

macro_rules! work {
    () => {{
        // fibonacci sequence
        let mut a = black_box(0);
        let mut b = black_box(1);
        for _ in black_box(0..1000) {
            let c = black_box(a + b);
            a = black_box(b);
            b = black_box(c);
        }
    }};
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".payload")]
pub extern "stdcall" fn stdcall_0_u32() -> u32 {
    work!();
    42
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".payload")]
pub fn stdcall_0_f32() -> f32 {
    work!();
    0.42
}
