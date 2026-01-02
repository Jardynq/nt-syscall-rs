use crate::tests::*;
use crate::*;
use std::vec;

#[test]
fn cpu_mode() {
    emu_only!();

    let mut mode: u16 = 0;
    unsafe {
        let args = x64::args!(ptr: &mut mode);
        asm!(
            args,
            x86::enter_x64!(),
            x64::get_cpu_mode!(),
            x64::enter_x86!()
        );
    };
    let mode = CpuMode::from(mode);
    assert_eq!(mode.host, CpuType::X64);
    assert_eq!(mode.user, CpuType::X64);
}

#[test]
fn peb_teb() {
    emu_only!();

    unsafe {
        let mut peb: u64 = 0;
        let mut teb: u64 = 0;
        let args = x64::args!(
            ptr: &mut peb,
            ptr: &mut teb
        );
        asm!(
            args,
            x86::enter_x64!(),
            x64::peb_ptr!(),
            x64::teb_ptr!(),
            x64::enter_x86!()
        );

        let mut peb_from_teb: u64 = 0;
        let mut teb_from_teb: u64 = 0;
        let args = x64::args!(
            ptr: &mut peb_from_teb,
            teb + 0x60,
            ptr: &mut teb_from_teb,
            teb + 0x30
        );
        asm!(
            args,
            x86::enter_x64!(),
            x64::memread_u64!(),
            x64::memread_u64!(),
            x64::enter_x86!()
        );

        assert_eq!(peb_from_teb, peb);
        assert_eq!(teb_from_teb, teb);
    }
}

#[test]
fn memset_low() {
    emu_only!();

    unsafe {
        let mut buffer = [0u8; 16];
        let args = x64::args!(buffer.as_mut_ptr(), 0x42, 16);
        asm!(args, x86::enter_x64!(), x64::memset!(), x64::enter_x86!());
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
        let args = x64::args!(target, 0x42, SIZE);
        asm!(args, x86::enter_x64!(), x64::memset!(), x64::enter_x86!());

        let mut buffer = vec![0u8; SIZE as usize];
        let args = x64::args!(buffer.as_mut_ptr(), target, SIZE);
        asm!(args, x86::enter_x64!(), x64::memcopy!(), x64::enter_x86!());
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
        let args = x64::args!(buffer.as_mut_ptr(), data.as_ptr(), 16);
        asm!(args, x86::enter_x64!(), x64::memcopy!(), x64::enter_x86!());
        assert_eq!(EXPECTED, buffer);
    }
}

#[test]
fn memcopy_high() {
    emu_only!();

    const THIS: &[u8] = "n\0t\0_\0s\0y\0s\0c\0a\0l\0l\0".as_bytes();
    const OTHER: &[u8] = "a\0b\0c\0d\0e\0f\0g\0h\0i\0j\0".as_bytes();
    unsafe {
        let mut peb: u64 = 0;
        let args = x64::args!(ptr: &mut peb);
        asm!(args, x86::enter_x64!(), x64::peb_ptr!(), x64::enter_x86!());

        let mut ldr_ptr: u64 = 0;
        let args = x64::args!(ptr: &mut ldr_ptr, peb + 0x18);
        asm!(
            args,
            x86::enter_x64!(),
            x64::memread_u64!(),
            x64::enter_x86!()
        );

        let mut entry_ptr: u64 = 0;
        let args = x64::args!(ptr: &mut entry_ptr, ldr_ptr + 0x10);
        asm!(
            args,
            x86::enter_x64!(),
            x64::memread_u64!(),
            x64::enter_x86!()
        );

        let mut name_ptr: u64 = 0;
        let args = x64::args!(ptr: &mut name_ptr, entry_ptr + 0x60);
        asm!(
            args,
            x86::enter_x64!(),
            x64::memread_u64!(),
            x64::enter_x86!()
        );

        // read name of first entry
        let mut name = [0u8; THIS.len().max(OTHER.len())];
        let args = x64::args!(name.as_mut_ptr(), name_ptr, THIS.len());
        asm!(args, x86::enter_x64!(), x64::memcopy!(), x64::enter_x86!());

        assert_eq!(&name, THIS);

        // write name and then read name of first entry

        let args = x64::args!(name_ptr, OTHER.as_ptr(), OTHER.len());
        asm!(args, x86::enter_x64!(), x64::memcopy!(), x64::enter_x86!());

        let args = x64::args!(name.as_mut_ptr(), name_ptr, OTHER.len());
        asm!(args, x86::enter_x64!(), x64::memcopy!(), x64::enter_x86!());

        assert_eq!(&name, OTHER);
    }
}

#[test]
#[should_panic]
fn jump_low() {
    emu_only!();

    fn panic2() {
        unsafe {
            let args = x64::args!();
            asm!(args, x64::enter_x86!());
        };
        panic!()
    }
    unsafe {
        let args = x64::args!(ptr: panic2 as *const ());
        asm!(args, x86::enter_x64!(), x64::jump!());
    }
}

#[test]
fn syscall_bad_id() {
    emu_only!();

    unsafe {
        let mut status: u64 = 0;
        let args = x64::args!(0xfff, ptr: &mut status);
        asm!(args, x86::enter_x64!(), x64::syscall!(0), x64::enter_x86!());
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
        let mut retval: u32 = 0;
        let args = x64::args!(target, ptr: &mut retval);
        asm!(
            args,
            x86::enter_x64!(),
            x64::call_x64_win64!(u32:),
            x64::enter_x86!(),
        );
        assert_eq!(retval, 0x123);
    }
}

#[test]
fn call_high_win64_simple() {
    emu_only!();

    unsafe {
        const SIZE: u64 = dummy_x64::WIN64_SIMPLE_U32.len() as u64;
        let target = alloc_high(SIZE);

        let args = x64::args!(target, dummy_x64::WIN64_SIMPLE_U32.as_ptr(), SIZE);
        asm!(args, x86::enter_x64!(), x64::memcopy!(), x64::enter_x86!());

        let mut retval: u64 = 0;
        let args = x64::args!(target, ptr: &mut retval);
        asm!(
            args,
            x86::enter_x64!(),
            x64::call_x64_win64!(u64:),
            x64::enter_x86!()
        );
        assert_eq!(retval, 0x123);
        free(target);
    }
}

#[test]
fn call_high_win64_simple_float() {
    emu_only!();

    unsafe {
        const SIZE: u64 = dummy_x64::WIN64_SIMPLE_F32.len() as u64;
        let target = alloc_high(SIZE);

        let args = x64::args!(target, dummy_x64::WIN64_SIMPLE_F32.as_ptr(), SIZE);
        asm!(args, x86::enter_x64!(), x64::memcopy!(), x64::enter_x86!());

        let mut retval: f64 = 0.0;
        let args = x64::args!(target, ptr: &mut retval);
        asm!(
            args,
            x86::enter_x64!(),
            x64::call_x64_win64!(f64:),
            x64::enter_x86!()
        );
        assert_eq!(retval, 123.123);
        free(target);
    }
}

#[test]
fn call_high_win64_complex() {
    emu_only!();

    unsafe {
        const SIZE: u64 = dummy_x64::WIN64_COMPLEX.len() as u64;
        let target = alloc_high(SIZE);
        let args = x64::args!(target, dummy_x64::WIN64_COMPLEX.as_ptr(), SIZE);
        asm!(args, x86::enter_x64!(), x64::memcopy!(), x64::enter_x86!());

        let a1 = 1;
        let a2 = 2;
        let a3 = 3;
        let a4 = 4;
        let a5 = 5.0f32;
        let a6 = 6.0f64;
        let a7: u32 = 7;
        let mut a8: u32 = 0xdeadbeef;
        let mut retval: f32 = 0.0;
        let args = x64::args!(
            target,
            ptr: &mut retval,
            a1,
            a2,
            a3,
            a4,
            f32: a5,
            f64: a6,
            ptr: &a7,
            ptr: &mut a8,
        );

        asm!(
            args,
            x86::enter_x64!(),
            x64::call_x64_win64!(f32: u8, u16, u32, u64, f32, f64, ptr, ptr,),
            x64::enter_x86!()
        );

        assert_eq!(a8, 0x123);
        assert_eq!(retval.round(), 28.0f32);
        free(target);
    }
}

#[test]
fn call_high_win64_complex2() {
    emu_only!();

    unsafe {
        const SIZE: u64 = dummy_x64::WIN64_COMPLEX2.len() as u64;
        let target = alloc_high(SIZE);
        let args = x64::args!(target, dummy_x64::WIN64_COMPLEX2.as_ptr(), SIZE);
        asm!(args, x86::enter_x64!(), x64::memcopy!(), x64::enter_x86!());

        let a1 = 10.0f32;
        let a2 = 20.0f32;
        let a3 = 30.0f64;
        let a4 = 40.0f32;
        let mut out: f32 = 123.123;
        let args = x64::args!(
            target,
            0,
            f32: a1,
            f32: a2,
            f64: a3,
            f32: a4,
            ptr: &mut out,
        );

        asm!(
            args,
            x86::enter_x64!(),
            x64::call_x64_win64!((): f32, f32, f64, f32, ptr),
            x64::enter_x86!()
        );

        assert_eq!(out.round(), 100.0f32);
        free(target);
    }
}
