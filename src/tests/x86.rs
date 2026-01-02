use crate::*;

#[test]
fn cpu_mode() {
    let mode = CpuMode::default();

    let mut mode2: u16 = 0;
    let args = x86::args!(ptr: &mut mode2);
    unsafe {
        asm!(args, x86::get_cpu_mode!());
    };

    assert_eq!(mode.value(), mode2);
    assert_eq!(mode.user, CpuType::X86);
    assert!(mode.host == CpuType::X64 || mode.host == CpuType::X86);
}

#[test]
fn peb_teb() {
    unsafe {
        let mut peb: u32 = 0;
        let mut teb: u32 = 0;
        let args = x86::args!(
            ptr: &mut peb,
            ptr: &mut teb
        );
        asm!(args, x86::peb_ptr!(), x86::teb_ptr!());

        let peb_from_teb = *((teb + 0x30) as *const u32);
        let teb_from_teb = *((teb + 0x18) as *const u32);
        assert_eq!(peb_from_teb, peb);
        assert_eq!(teb_from_teb, teb);
    }
}

#[test]
fn syscall_bad_id() {
    native_only!();

    unsafe {
        let mut status: u32 = 0;
        let args = x86::args!(0xfff, ptr: &mut status);
        asm!(args, x86::syscall!(0));
        assert_eq!(status, 0xc000001c);
    }
}

#[test]
#[should_panic]
fn args_fn() {
    let arg0: u32 = 0x1111;
    let arg1: u32 = 0x2222;
    let arg2: u32 = 0x3333;

    fn panic2(arg0: u32, arg1: u32, arg2: u32) {
        assert!(arg0 != 0x1111 && arg1 != 0x2222 && arg2 != 0x3333);
    }

    let args = x86::args!(
        arg0,
        arg1,
        ptr: panic2 as *const (),
        arg2
    );
    unsafe {
        asm!(
            args,
            "mov eax, [ecx]\n",
            "push eax\n",
            "mov eax, [ecx + 0x4]\n",
            "push eax\n",
            "mov eax, [ecx + 0xc]\n",
            "push eax\n",
            "mov edx, [ecx + 0x8]\n",
            "call edx\n",
            x86::next_args!(4),
        )
    };
}

#[test]
fn args() {
    let arg0: u32 = 0x1111;
    let arg1: u32 = 0x2222;
    let arg2: u32 = 0x3333;
    let ptr3: u32 = 0x4444;
    let mut ptr4: u32 = 0x5555;
    let mut ptr5: u32 = 0x6666;

    fn panic2() -> ! {
        panic!("Failed asm comparison.")
    }

    let args = x86::args!(
        arg0,
        arg1,
        arg2,
        ptr: &ptr3,
        ptr: &mut ptr4,
        ptr: &mut ptr5,
        ptr: panic2 as *const ()
    );
    unsafe {
        asm!(
            args,
            "mov edx, [ecx + 0x18]\n",
            "mov eax, [ecx]\n",
            "cmp eax, 0x1111\n",
            "je 2f\n",
            "call edx\n",
            "2:\n",
            x86::next_args!(@ 0 + 1 + 0),
            "mov eax, [ecx]\n",
            "cmp eax, 0x2222\n",
            "je 3f\n",
            "call edx\n",
            "3:\n",
            "mov eax, [ecx + 4]\n",
            "cmp eax, 0x3333\n",
            "je 4f\n",
            "call edx\n",
            "4:\n",
            x86::next_args!(1 + u32),
            "mov eax, [ecx]\n",
            "mov eax, [eax]\n",
            "cmp eax, 0x4444\n",
            "je 5f\n",
            "call edx\n",
            "5:\n",
            "mov eax, [ecx + 4]\n",
            "mov dword ptr [eax], 0x1234\n",
            "mov eax, [ecx + 8]\n",
            "mov dword ptr [eax], 0x5678\n",
            x86::next_args!(1 + u32, u32),
        )
    };

    assert_eq!(arg0, 0x1111);
    assert_eq!(arg1, 0x2222);
    assert_eq!(arg2, 0x3333);
    assert_eq!(ptr3, 0x4444);
    assert_eq!(ptr4, 0x1234);
    assert_eq!(ptr5, 0x5678);
}
