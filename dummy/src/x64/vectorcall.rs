use std::arch::x86_64::*;

#[unsafe(no_mangle)]
#[unsafe(link_section = ".payload")]
#[target_feature(enable = "avx2")]
pub unsafe extern "vectorcall" fn vectorcall_simple(a: __m256, b: __m256) -> __m256 {
    _mm256_add_ps(a, b)
}
