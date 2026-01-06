use crate::*;

#[test]
fn cpu_mode() {
    let mode = CpuMode::default();

    let mut mode2: u16 = 0;
    let args = x64::args!(ptr: &mut mode2);
    unsafe {
        asm!(args, x64::get_cpu_mode!());
    };

    assert_eq!(mode.value(), mode2);
    assert_eq!(mode.user, CpuType::X64);
    assert_eq!(mode.host, CpuType::X64);
}

#[test]
fn peb_teb() {
    unsafe {
        let mut peb: u64 = 0;
        let mut teb: u64 = 0;
        let args = x64::args!(
            ptr: &mut peb,
            ptr: &mut teb
        );
        asm!(args, x64::peb_ptr!(), x64::teb_ptr!());
        let peb_from_teb = *((teb + 0x60) as *const u64);
        let teb_from_teb = *((teb + 0x30) as *const u64);
        assert_eq!(peb_from_teb, peb);
        assert_eq!(teb_from_teb, teb);
    }
}

#[test]
fn syscall_bad_id() {
    native_only!();

    unsafe {
        let mut status: u32 = 0;
        let args = x64::args!(0xfff, ptr: &mut status);
        asm!(args, x64::syscall!(0));
        assert_eq!(status, 0xc000001c);
    }
}

#[test]
#[should_panic]
fn args_fn() {
    let arg0: u64 = 0x1111;
    let arg1: u64 = 0x2222;
    let arg2: u64 = 0x3333;

    fn panic2(arg0: u64, arg1: u64, arg2: u64) {
        assert!(arg0 != 0x1111 && arg1 != 0x2222 && arg2 != 0x3333);
    }

    let args = x64::args!(
        arg0,
        arg1,
        ptr: panic2 as *const (),
        arg2
    );
    unsafe {
        asm!(
            args,
            "mov rax, [rcx + 0x10]\n",
            "mov rdx, [rcx + 0x8]\n",
            "mov r8, [rcx + 0x18]\n",
            "mov rcx, [rcx]\n",
            "call rax\n",
            x64::arg_next!(4),
        )
    };
}

#[test]
fn args() {
    let arg0: u64 = 0x1111;
    let arg1: u64 = 0x2222;
    let arg2: u64 = 0x3333;
    let ptr3: u64 = 0x4444;
    let mut ptr4: u64 = 0x5555;
    let mut ptr5: u64 = 0x6666;

    fn panic2() -> ! {
        panic!("Failed asm comparison.")
    }

    let args = x64::args!(
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
            "mov rdx, [rcx + 0x30]\n",
            "mov rax, [rcx]\n",
            "cmp rax, 0x1111\n",
            "je 2f\n",
            "call rdx\n",
            "2:\n",
            x64::arg_next!(@ 0 + 1 + 0),
            "mov rax, [rcx]\n",
            "cmp rax, 0x2222\n",
            "je 3f\n",
            "call rdx\n",
            "3:\n",
            "mov rax, [rcx + 8]\n",
            "cmp rax, 0x3333\n",
            "je 4f\n",
            "call rdx\n",
            "4:\n",
            x64::arg_next!(1 + u64),
            "mov rax, [rcx]\n",
            "mov rax, [rax]\n",
            "cmp rax, 0x4444\n",
            "je 5f\n",
            "call rdx\n",
            "5:\n",
            "mov rax, [rcx + 8]\n",
            "mov qword ptr [rax], 0x1234\n",
            "mov rax, [rcx + 0x10]\n",
            "mov qword ptr [rax], 0x5678\n",
            x64::arg_next!(1 + u64, u64),
        )
    };

    assert_eq!(arg0, 0x1111);
    assert_eq!(arg1, 0x2222);
    assert_eq!(arg2, 0x3333);
    assert_eq!(ptr3, 0x4444);
    assert_eq!(ptr4, 0x1234);
    assert_eq!(ptr5, 0x5678);
}
