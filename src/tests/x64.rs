use crate::*;

#[test]
fn cpu_mode() {
    let mode = CpuMode::default();
    assert_eq!(mode.host, CpuType::X64);
    assert_eq!(mode.user, CpuType::X64);
}

#[test]
fn peb_teb() {
    unsafe {
        let peb = x64::inline::peb_ptr!();
        let teb = x64::inline::teb_ptr!();
        let (peb2, teb2) = x64::inline::peb_teb_ptr!();

        let peb_from_teb = *((teb + 0x60) as *const u64);
        let teb_from_teb = *((teb + 0x30) as *const u64);

        assert_eq!(peb, peb2);
        assert_eq!(teb, teb2);
        assert_eq!(peb_from_teb, peb);
        assert_eq!(teb_from_teb, teb);
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

    let args = x64::shellcode::args!(arg0, arg1, fn panic2, arg2);
    unsafe {
        asm!(
            args,
            "mov rdx, [rcx + 0x8]\n",
            "mov rax, [rcx + 0x10]\n",
            "mov r8, [rcx + 0x18]\n",
            "mov rcx, [rcx]\n",
            "call rax\n",
            x86::shellcode::next_args!(5),
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

    let args = x64::shellcode::args!(arg0, arg1, arg2, &ptr3, &mut ptr4, &mut ptr5, fn panic2);
    unsafe {
        asm!(
            args,
            "mov rdx, [rcx + 0x30]\n",
            "mov rax, [rcx]\n",
            "cmp rax, 0x1111\n",
            "je 2f\n",
            "call rdx\n",
            "2:\n",
            "mov rax, [rcx + 0x8]\n",
            "cmp rax, 0x2222\n",
            "je 3f\n",
            "call rdx\n",
            "3:\n",
            "mov rax, [rcx + 0x10]\n",
            "cmp rax, 0x3333\n",
            "je 4f\n",
            "call rdx\n",
            "4:\n",
            x64::shellcode::next_args!(3),
            "mov rax, [rcx]\n",
            "mov rax, [rax]\n",
            "cmp rax, 0x4444\n",
            "je 5f\n",
            "call rdx\n",
            "5:\n",
            "mov rax, [rcx + 0x8]\n",
            "mov qword ptr [rax], 0x1234\n",
            "mov rax, [rcx + 0x10]\n",
            "mov qword ptr [rax], 0x5678\n",
            x64::shellcode::next_args!(3),
        )
    };

    assert_eq!(arg0, 0x1111);
    assert_eq!(arg1, 0x2222);
    assert_eq!(arg2, 0x3333);
    assert_eq!(ptr3, 0x4444);
    assert_eq!(ptr4, 0x1234);
    assert_eq!(ptr5, 0x5678);
}

#[test]
fn syscall_bad_id() {
    unsafe {
        let status = x64::inline::syscall!((0xfff));
        assert_eq!(status, 0xc000001c);
    }
}
