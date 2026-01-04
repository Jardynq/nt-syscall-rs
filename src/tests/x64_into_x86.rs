use crate::tests::*;
use crate::*;

#[unsafe(naked)]
pub unsafe extern "win64" fn x64_jump_x86(func: *const u8, stack: *mut u8) {
    //crate::host_x64::naked_x64_call_x86_ret_x64!(stdcall 0 u32)
    core::arch::naked_asm!("")
}

/*
#[test]
fn test_cpu_mode() {
    const SIZE: u64 = 0x10000;
    let target = alloc_low(SIZE);
    let trampoline = alloc_low(SIZE);
    let stack = alloc_low(SIZE);

    unsafe {
        core::ptr::copy(
            dummy_x86::STDCALL_0_U32.as_ptr(),
            target,
            dummy_x86::STDCALL_0_U32.len(),
        );
        core::ptr::copy(x64_jump_x86 as *const _, trampoline, SIZE);
    }

    wait();

    let mode = unsafe {
        let targ: extern "stdcall" fn() -> u32 = core::mem::transmute(target);
        let tramp: extern "win64" fn(*const u8, *mut u8) -> u32 = core::mem::transmute(trampoline);
        let res: u32 = tramp(targ as _, stack + SIZE / 2);
        panic!("panic: {}", res);
    };

    //assert_eq!(mode.host, CpuType::X64);
    //assert_eq!(mode.user, CpuType::X86);

    free(target);
    free(trampoline);
    free(stack);
}
 */
