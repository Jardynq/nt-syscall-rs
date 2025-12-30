#![allow(unused_imports)]
use crate::{asm, shared, x64, x86};

pub macro enter_x86() {{
    let args = x64::shellcode::args!();
    asm!(args, x64::shellcode::enter_x86!());
}}

pub macro get_cpu_mode() {{
    let mut mode: u64 = 0;
    let args = x64::shellcode::args!(&mut mode);
    asm!(args, x64::shellcode::get_cpu_mode!());
    mode as u16
}}

pub macro syscall($index:tt $(, $($args:tt)* )?) {{
	let mut status: u64 = 0;
	let args = x64::shellcode::args!($index, &mut status, $($($args)*)? );
	asm!(
		args,
		shared::count_args_helper!((x64::shellcode::syscall), (), $($($args)*)? ),
	);
	status as u32
}}

pub macro peb_ptr() {{
    let mut ptr: u64 = 0;
    let args = x64::shellcode::args!(&mut ptr);
    asm!(args, x64::shellcode::peb_ptr!());
    ptr
}}
pub macro teb_ptr() {{
    let mut ptr: u64 = 0;
    let args = x64::shellcode::args!(&mut ptr);
    asm!(args, x64::shellcode::teb_ptr!());
    ptr
}}
pub macro peb_teb_ptr() {{
    let (mut peb, mut teb): (u64, u64) = (0, 0);
    let args = x64::shellcode::args!(&mut peb, &mut teb);
    asm!(args, x64::shellcode::peb_ptr!(), x64::shellcode::teb_ptr!());
    (peb, teb)
}}
