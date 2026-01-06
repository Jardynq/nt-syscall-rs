use crate::tests::*;
use crate::*;
use core::ptr;

static TRAMPOLINE_CPU_MODE: &[u8] = &encode!(
    x64::enter_x86_with_stack!(),
    x86::get_cpu_mode!(),
    x86::enter_x64_restore_stack!(),
    x64::assemble!("ret")
);

#[test]
fn cpu_mode() {
    unsafe {
        const SIZE: u64 = 0x1000;
        let target = alloc_low(SIZE);
        let stack = alloc_low(SIZE);
        ptr::copy(
            TRAMPOLINE_CPU_MODE.as_ptr(),
            target as *mut u8,
            TRAMPOLINE_CPU_MODE.len(),
        );

        let mode_ptr = stack as *const u16;

        let x86_args_ptr = alloc_low(SIZE);
        let x86_args = core::slice::from_raw_parts_mut(x86_args_ptr as *mut u8, SIZE as usize);
        x86::args_in!(x86_args, ptr: mode_ptr);

        let x64_args = x64::args!(target, stack + SIZE, x86_args_ptr);

        asm!(
            x64_args,
            x64::assemble!("mov rax, qword ptr [rcx]"),
            x64::arg_next!(1),
            x64::assemble!("call rax")
        );

        let mode = *mode_ptr;
        let mode = CpuMode::from(mode);
        assert_eq!(mode.host, CpuType::X64);
        assert_eq!(mode.user, CpuType::X86);

        free(target);
        free(stack);
        free(x86_args_ptr);
    }
}

static TRAMPOLINE_JUMP_LOW: &[u8] = &encode!(x64::enter_x86_with_stack!(), x86::jump!(),);
static TRAMPOLINE_JUMP_LOW_TARGET: &[u8] =
    &encode!(x86::enter_x64_restore_stack!(), x64::assemble!("ret"));

#[test]
fn jump_low() {
    unsafe {
        const SIZE: u64 = 0x1000;
        let target = alloc_low(SIZE);
        let stack = alloc_low(SIZE);
        let jump_target = alloc_low(SIZE);

        ptr::copy(
            TRAMPOLINE_JUMP_LOW.as_ptr(),
            target as *mut u8,
            TRAMPOLINE_JUMP_LOW.len(),
        );
        ptr::copy(
            TRAMPOLINE_JUMP_LOW_TARGET.as_ptr(),
            jump_target as *mut u8,
            TRAMPOLINE_JUMP_LOW_TARGET.len(),
        );

        let x86_args_ptr = alloc_low(SIZE);
        let x86_args = core::slice::from_raw_parts_mut(x86_args_ptr as *mut u8, SIZE as usize);
        x86::args_in!(x86_args, ptr: jump_target);

        let x64_args = x64::args!(target, stack + SIZE, x86_args_ptr);
        asm!(
            x64_args,
            x64::assemble!("mov rax, qword ptr [rcx]"),
            x64::arg_next!(1),
            x64::assemble!("call rax")
        );

        free(target);
        free(stack);
        free(jump_target);
        free(x86_args_ptr);
    }
}
