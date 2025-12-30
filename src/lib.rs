#![no_std]
#![cfg(windows)]
#![allow(unused_macros, unused_unsafe)]
#![feature(macro_metavar_expr, decl_macro, const_cmp, const_trait_impl)]
#![recursion_limit = "1024"]

#[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
compile_error!("Unsupported architecture");

#[cfg(test)]
mod tests;

mod shared;
pub mod x64;
pub mod x86;
pub use aligned;

pub macro asm($args:expr, $($tok:tt)+) {{
    #[cfg(target_arch = "x86_64")]
    core::arch::asm!(
        concat!($($tok)+),
        inout("rcx") ($args).as_ptr() as u64 => _,
        clobber_abi("C"),
        //out("rdx") _, // rbx cannot be clobbered as it is used by llvm. So avoid using it.
    );
    #[cfg(target_arch = "x86")]
    core::arch::asm!(
        concat!($($tok)+),
        inout("ecx") ($args).as_ptr() as u32 => _,
        clobber_abi("C"),
        //out("edx") _, // ebx cannot be clobbered as it is used by llvm. So avoid using it.
    );
}}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CpuType {
    X64,
    X86,
    Arm64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CpuMode {
    host: CpuType,
    user: CpuType,
}
impl Default for CpuMode {
    fn default() -> Self {
        let mut mode: u16;
        unsafe {
            core::arch::asm!(
                "mov ax, cs",
                out("ax") mode,
            );
        }
        Self::from(mode)
    }
}
impl CpuMode {
    pub fn from(mode: u16) -> Self {
        match mode {
            0x33 => Self {
                host: CpuType::X64,
                user: CpuType::X64,
            },
            0x23 => Self {
                host: CpuType::X64,
                user: CpuType::X86,
            },
            0x1b => Self {
                host: CpuType::X86,
                user: CpuType::X86,
            },
            _ => panic!("Unsupported CPU type: {:#x}", mode),
        }
    }
    pub fn value(&self) -> u16 {
        match (self.host, self.user) {
            (CpuType::X64, CpuType::X64) => 0x33,
            (CpuType::X64, CpuType::X86) => 0x23,
            (CpuType::X86, CpuType::X86) => 0x1b,
            _ => panic!("Unsupported CPU type: {:?}, {:?}", self.host, self.user),
        }
    }
    pub fn is_emu(&self) -> bool {
        self.host != self.user
    }
}

pub macro encode($($lit:tt)+) {
    const_str::hex!(
        const_str::split!(
            const_str::trim_ascii!(
                const_str::replace!(
                    const_str::replace!(
                        const_str::replace!(
                            const_str::replace!(
                                $($lit)+,
                                "0x",
                                ""
                            ),
                            "\n.byte ",
                            ", "
                        ),
                        ".byte ",
                        ""
                    ),
                    "\n",
                    ""
                )
            ),
            ", "
        )
    )
}
