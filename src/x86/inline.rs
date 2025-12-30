#![allow(unused_imports)]
use crate::{asm, shared, x64, x86};

// TODO: need to make a better inline api, where the first aguments follow the fancy pattern matching
// Proc macro time?

pub macro enter_x64() {{
    let args = x86::shellcode::args!();
    asm!(args, x86::shellcode::enter_x64!());
}}

pub macro get_cpu_mode() {{
    let mut mode: u32 = 0;
    let args = x86::shellcode::args!(&mut mode);
    asm!(args, x86::shellcode::get_cpu_mode!());
    mode as u16
}}

pub macro syscall($($args:tt)*) {{
	if crate::CpuMode::default().host != crate::CpuType::X86 {
		syscall_emulated!($($args)*)
	} else {
		// TODO
		0
		//syscall_native!( $($args)* ) as u64
	}
}}

pub macro syscall_native($index:tt $(, $($args:tt)*)?) {{
	let mut status: u32 = 0;
	let args = x86::shellcode::args!($index, &mut status, $($($args)*)? );
	asm!(
		args,
		x86::shellcode::syscall!($($($args)*)?),
	);
	status
}}
pub macro syscall_emulated($index:tt $(, $($args:tt)*)?) {{
	let mut status: u64 = 0;
	let args = x64::shellcode::args!($index, &mut status, $($($args)*)? );
	asm!(
		args,
		x86::shellcode::enter_x64!(),
		x64::shellcode::syscall!($($($args)*)?),
		x64::shellcode::enter_x86!()
	);
	status as u32
}}

pub macro peb_ptr() {{
    let mut ptr: u32 = 0;
    let args = x86::shellcode::args!(&mut ptr);
    asm!(args, x86::shellcode::peb_ptr!());
    ptr
}}
pub macro teb_ptr() {{
    let mut ptr: u32 = 0;
    let args = x86::shellcode::args!(&mut ptr);
    asm!(args, x86::shellcode::teb_ptr!());
    ptr
}}
pub macro peb_teb_ptr() {{
    let (mut peb, mut teb): (u32, u32) = (0, 0);
    let args = x86::shellcode::args!(&mut peb, &mut teb);
    asm!(args, x86::shellcode::peb_ptr!(), x86::shellcode::teb_ptr!());
    (peb, teb)
}}

pub macro peb_ptr_emulated() {{
    let mut ptr: u64 = 0;
    let args = x64::shellcode::args!(&mut ptr);
    asm!(
        args,
        x86::shellcode::enter_x64!(),
        x64::shellcode::peb_ptr!(),
        x64::shellcode::enter_x86!()
    );
    ptr
}}
pub macro teb_ptr_emulated() {{
    let mut ptr: u64 = 0;
    let args = x64::shellcode::args!(&mut ptr);
    asm!(
        args,
        x86::shellcode::enter_x64!(),
        x64::shellcode::teb_ptr!(),
        x64::shellcode::enter_x86!()
    );
    ptr
}}
pub macro peb_teb_ptr_emulated() {{
    let (mut peb, mut teb): (u64, u64) = (0, 0);
    let args = x64::shellcode::args!(&mut peb, &mut teb);
    asm!(
        args,
        x86::shellcode::enter_x64!(),
        x64::shellcode::peb_ptr!(),
        x64::shellcode::teb_ptr!(),
        x64::shellcode::enter_x86!()
    );
    (peb, teb)
}}

pub macro memcopy_x64($($args:tt)*) {{
	let args = x64::shellcode::args!($($args)*);
	asm!(
		args,
		x86::shellcode::memcopy_x64!()
	);
}}
pub macro read_x64_u64($($src:tt)*) {{
	let mut dst = [0u8; 8];
	memcopy_x64!(dst.as_mut_ptr(), $($src)*, 8);
	u64::from_le_bytes(dst)
}}
pub macro read_x64_u32($($src:tt)*) {{
	let mut dst = [0u8; 4];
	memcopy_x64!(dst.as_mut_ptr(), $($src)*, 4);
	u32::from_le_bytes(dst)
}}

pub macro memset_x64($($args:tt)*) {{
	let args = x64::shellcode::args!($($args)*);
	asm!(
		args,
		x86::shellcode::memset_x64!()
	);
}}

pub macro jump_x64($($dst:tt)*) {{
	let args = x64::shellcode::args!($($dst)*);
    asm!(args, x86::shellcode::jump_x64!());
}}

pub macro call_x64_win64_float($target:tt $(, $($args:tt)* )?) {{
	let mut retval: u64 = 0;
	let args = x64::shellcode::args!($target, &mut retval, $($($args)*)?);
	asm!(
		args,
		x86::shellcode::call_x64_win64_float!($($($args)*)?)
	);
	retval
}}

pub macro call_x64_win64($target:tt $(, $($args:tt)* )?) {{
	let mut retval: u64 = 0;
	let args = x64::shellcode::args!($target, &mut retval, $($($args)*)?);
	asm!(
		args,
		x86::shellcode::call_x64_win64!($($($args)*)?)
	);
	retval
}}
