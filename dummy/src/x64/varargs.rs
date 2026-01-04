use crate::*;

#[unsafe(no_mangle)]
#[unsafe(link_section = ".payload")]
pub unsafe extern "C" fn varargs(a: f32, mut args: ...) -> i32 {
    unsafe {
        let b = args.arg::<u32>();
        let c = args.arg::<i32>();
        let d = cast_i64!(args.arg::<f64>());
        let e = args.arg::<u64>();
        let f = args.arg::<u64>();
        let sum = cast_i32!(a) as u32 + b + c as u32 + d as u32 + e as u32;
        *(f as *mut u32) = sum * 2;
        sum as i32
    }
}
