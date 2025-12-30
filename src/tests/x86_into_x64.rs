use crate::tests::*;
use crate::*;

#[test]
fn cpu_mode() {
    emu_only!();
    let mut mode: u64 = 0;
    unsafe {
        let args = x64::shellcode::args!(&mut mode);
        asm!(
            args,
            x86::shellcode::enter_x64!(),
            x64::shellcode::get_cpu_mode!(),
            x64::shellcode::enter_x86!()
        );
    };
    let mode = CpuMode::from(mode as u16);
    assert_eq!(mode.host, CpuType::X64);
    assert_eq!(mode.user, CpuType::X64);
}

#[test]
fn peb_teb() {
    emu_only!();
    unsafe {
        let (peb, teb) = x86::inline::peb_teb_ptr_emulated!();
        let teb_from_teb = x86::inline::read_x64_u64!(teb + 0x30);
        let peb_from_teb = x86::inline::read_x64_u64!(teb + 0x60);

        assert_eq!(peb_from_teb, peb);
        assert_eq!(teb_from_teb, teb);
    }
}

#[test]
fn memset_low() {
    emu_only!();
    unsafe {
        let mut buffer = [0u8; 16];
        x86::inline::memset_x64!(buffer.as_mut_ptr(), 0x42, 16);
        for byte in buffer {
            assert_eq!(byte, 0x42);
        }
    }
}
#[test]
fn memset_high() {
    emu_only!();
    unsafe {
        const SIZE: u64 = 0x1000 as u64;
        let target = alloc_high(SIZE);
        x86::inline::memset_x64!(target, 0x42, SIZE);

        let mut buffer = vec![0u8; SIZE as usize];
        x86::inline::memcopy_x64!(buffer.as_mut_ptr(), target, SIZE);
        for byte in buffer {
            assert_eq!(byte, 0x42);
        }
    }
}

#[test]
fn memcopy_low() {
    emu_only!();
    unsafe {
        const EXPECTED: [u8; 16] = [0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let data = [0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let mut buffer = [0u8; 16];
        x86::inline::memcopy_x64!(buffer.as_mut_ptr(), data.as_ptr(), 16);
        assert_eq!(EXPECTED, buffer);
    }
}

#[test]
fn memcopy_high() {
    emu_only!();
    const THIS: &[u8] = "n\0t\0_\0s\0y\0s\0c\0a\0l\0l\0".as_bytes();
    const OTHER: &[u8] = "a\0b\0c\0d\0e\0f\0g\0h\0i\0j\0".as_bytes();
    unsafe {
        let peb = x86::inline::peb_ptr_emulated!();
        let ldr_ptr = x86::inline::read_x64_u64!(peb + 0x18);
        let entry_ptr = x86::inline::read_x64_u64!(ldr_ptr + 0x10);
        let name_ptr = x86::inline::read_x64_u64!(entry_ptr + 0x60);

        // read name of first entry
        let mut name = [0u8; THIS.len().max(OTHER.len())];
        x86::inline::memcopy_x64!(name.as_mut_ptr(), name_ptr, THIS.len());

        assert_eq!(&name, THIS);

        // write name and then read name of first entry
        x86::inline::memcopy_x64!(name_ptr, OTHER.as_ptr(), OTHER.len());
        x86::inline::memcopy_x64!(name.as_mut_ptr(), name_ptr, OTHER.len());

        assert_eq!(&name, OTHER);
    }
}

#[test]
#[should_panic]
fn jump_low() {
    emu_only!();
    fn panic2() {
        unsafe {
            x64::inline::enter_x86!();
        };
        panic!()
    }
    unsafe {
        x86::inline::jump_x64!(fn panic2);
    }
}

#[test]
fn syscall_bad_id() {
    emu_only!();
    unsafe {
        let status = x86::inline::syscall_emulated!((0xfff));
        assert_eq!(status, 0xc000001c);
    }
}

#[test]
fn call_low() {
    emu_only!();

    #[unsafe(naked)]
    extern "C" fn target() {
        core::arch::naked_asm!("mov eax, 0x123", "ret");
    }
    unsafe {
        let target = target as *const ();
        let retval = x86::inline::call_x64_win64!(target);
        assert_eq!(retval, 0x123);
    }
}

#[test]
fn call_high_win64_simple() {
    emu_only!();
    unsafe {
        const SIZE: u64 = dummy_x64::WIN64_SIMPLE_U32.len() as u64;
        let target = alloc_high(SIZE);
        x86::inline::memcopy_x64!(target, dummy_x64::WIN64_SIMPLE_U32.as_ptr(), SIZE);
        let retval = x86::inline::call_x64_win64!(target);
        assert_eq!(retval, 0x123);
        free(target);

        const SIZE2: u64 = dummy_x64::WIN64_SIMPLE_F32.len() as u64;
        let target = alloc_high(SIZE2);
        x86::inline::memcopy_x64!(target, dummy_x64::WIN64_SIMPLE_F32.as_ptr(), SIZE2);
        let retval: f64 = core::mem::transmute(x86::inline::call_x64_win64_float!(target));
        assert_eq!(retval, 123.123f64);
        free(target);
    }
}

#[test]
fn call_high_win64_complex() {
    emu_only!();
    unsafe {
        const SIZE: u64 = dummy_x64::WIN64_COMPLEX.len() as u64;
        let target = alloc_high(SIZE);
        x86::inline::memcopy_x64!(target, dummy_x64::WIN64_COMPLEX.as_ptr(), SIZE);
        let a1 = 1;
        let a2 = 2;
        let a3 = 3;
        let a4 = 4;
        let a5 = 5.0f32;
        let a6 = 6.0f64;
        let a7: u64 = 7;
        let mut a8: u64 = 0;
        let retval: f64 = core::mem::transmute(x86::inline::call_x64_win64_float!(
            target, a1, a2, a3, a4, float a5, float a6, &a7, &mut a8
        ));
        assert_eq!(a8 as u32, 0x123);
        assert_eq!(retval, 28.0f64);
        free(target);
    }
}
