extern crate std;

#[allow(unused_imports)]
use crate::{asm, encode, x64, x86};
use winapi::um::winnt::{MEM_COMMIT, MEM_RELEASE, MEM_RESERVE, PAGE_EXECUTE_READWRITE};

#[cfg(target_arch = "x86_64")]
#[path = "x64.rs"]
mod test_x64;
#[cfg(target_arch = "x86_64")]
#[path = "x64_into_x86.rs"]
mod test_x64_into_x86;

#[cfg(target_arch = "x86")]
#[path = "x86.rs"]
mod test_x86;
#[cfg(target_arch = "x86")]
#[path = "x86_into_x64.rs"]
mod test_x86_into_x64;

mod dummy_x64 {
    #![allow(dead_code)]
    include!("../../target/x86_64-pc-windows-msvc/dummy/dummy.rs");
}
mod dummy_x86 {
    #![allow(dead_code)]
    include!("../../target/i686-pc-windows-msvc/dummy/dummy.rs");
}

#[macro_export]
macro_rules! emu_only {
    () => {
        if !CpuMode::default().is_emu() {
            return;
        }
    };
}
#[macro_export]
macro_rules! native_only {
    () => {
        if CpuMode::default().is_emu() {
            return;
        }
    };
}

static mut NT_VIRTUAL_MEMORY_IDS: std::sync::LazyLock<(u32, u32)> =
    std::sync::LazyLock::new(|| {
        let ids = nt_sysdump::dump(
            nt_sysdump::LoadFile::Ntdll,
            nt_sysdump::LoadMethod::Sorting,
            nt_sysdump::LoadSource::Memory,
            None,
            None,
        )
        .expect("Failed to dump syscalls");

        let (_, alloc_id) = ids
            .iter()
            .find(|(name, _)| name == "NtAllocateVirtualMemory")
            .expect("NtAllocateVirtualMemory not found");
        let (_, free_id) = ids
            .iter()
            .find(|(name, _)| name == "NtFreeVirtualMemory")
            .expect("NtFreeVirtualMemory not found");
        (*alloc_id, *free_id)
    });

#[allow(dead_code)]
fn wait() {
    std::io::stdin()
        .read_line(&mut std::string::String::new())
        .unwrap();
}

fn alloc(mut address: u64, mut size: u64) -> Result<u64, u32> {
    unsafe {
        let id = NT_VIRTUAL_MEMORY_IDS.0;
        let mut status: u64 = 0;
        let args = x64::args!(
            id,
            &mut status,
            -1i64,
            &mut address,
            0,
            &mut size,
            MEM_RESERVE | MEM_COMMIT,
            PAGE_EXECUTE_READWRITE
        );

        #[cfg(target_arch = "x86")]
        asm!(args, x86::enter_x64!(), x64::syscall!(6), x64::enter_x86!());
        #[cfg(target_arch = "x86_64")]
        asm!(args, x64::syscall!(6));

        if status != 0 {
            Err(status as u32)
        } else {
            Ok(address)
        }
    }
}

fn alloc_low(size: u64) -> u64 {
    let mut addr = 0x10000u64;
    let end = 0x7fff0000u64;

    while addr < end {
        if let Ok(ptr) = alloc(addr, size) {
            return ptr;
        }
        addr += 0x1000;
    }
    panic!("Failed to allocate low address memory");
}

fn alloc_high(size: u64) -> u64 {
    let mut addr = 0x0000001000000000u64;
    let end = 0x7fffffff00000000u64;

    while addr < end {
        if let Ok(ptr) = alloc(addr, size) {
            return ptr;
        }
        addr += 0x1000;
    }
    panic!("Failed to allocate high address memory");
}

fn free(mut address: u64) {
    unsafe {
        let mut size: u64 = 0;
        let mut status: u64 = 0;
        let id = NT_VIRTUAL_MEMORY_IDS.1;
        let args = x64::args!(
            id,
            &mut status,
            -1isize,
            &mut address,
            &mut size,
            MEM_RELEASE
        );

        #[cfg(target_arch = "x86")]
        asm!(args, x86::enter_x64!(), x64::syscall!(5), x64::enter_x86!());

        #[cfg(target_arch = "x86_64")]
        asm!(args, x64::syscall!(5));

        if status != 0 {
            panic!("NtFreeVirtualMemory failed: {:#x}", status);
        }
    }
}

#[test]
fn test_alloc_low() {
    const SIZE: u64 = 0x1000;
    let addr = alloc_low(SIZE);
    assert!(addr < u32::MAX as u64);
    free(addr);
}

#[test]
fn test_alloc_high() {
    const SIZE: u64 = 0x1000;
    let addr = alloc_high(SIZE);
    assert!(addr > u32::MAX as u64);
    free(addr);
}

#[test]
fn test_encode() {
    assert_eq!(
        encode!(x64::enter_x86!()),
        [
            232, 0, 0, 0, 0, 199, 68, 36, 4, 35, 0, 0, 0, 131, 4, 36, 13, 203, 102, 140, 216, 102,
            142, 208
        ]
    );
}
