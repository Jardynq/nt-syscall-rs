#![no_std]
#![allow(unused_macros)]
#![feature(macro_metavar_expr)]


pub mod indices;

mod version;
pub use version::*;




#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CpuMode {
	Unknown(u16),
	NativeX64,
	EmulatedX86,
	NativeX86,
}
#[macro_export]
macro_rules! cpu_mode {
	() => { unsafe {
		let mut cs: u16;
		asm!(
			"mov ax, cs",
			out("ax") cs,
			options(pure, nomem, nostack, preserves_flags),
		);
		match cs {
			0x33 => ::nt_syscall::CpuMode::NativeX64,
			0x23 => ::nt_syscall::CpuMode::EmulatedX86,
			0x1b => ::nt_syscall::CpuMode::NativeX86,
			_ => ::nt_syscall::CpuMode::Unknown(cs),
		}
	}}
}


// TODO make a x64/x86 jmp/call


// Only call from x86 thread
#[macro_export]
macro_rules! enter_x64 {
	() => {
		asm!(
			"push 0x33",
			// call $0
			".byte 0xe8", ".byte 0x00", ".byte 0x00", ".byte 0x00", ".byte 0x00",
			"add dword ptr [esp], 0x5",
			"retf",

			options(nomem, preserves_flags),
		);
	}
}


// Only call from x64 thread, where the return addres is < u32::MAX
#[macro_export]
macro_rules! enter_x86 {
	() => {
		asm!(
			// call $0
			".byte 0xe8", ".byte 0x00", ".byte 0x00", ".byte 0x00", ".byte 0x00",
			// mov dword ptr [rsp + 0x4], 0x23
			".byte 0xc7", ".byte 0x44", ".byte 0x24", ".byte 0x04", ".byte 0x23", ".byte 0x00", ".byte 0x00", ".byte 0x00",
			// add dword ptr [rsp], 0xd
			".byte 0x83", ".byte 0x04", ".byte 0x24", ".byte 0x0d", 
			"retf",
			// Solution for AMD specific race condition
			"mov ax, ds",
			"mov ss, ax",

			out("ax") _,
			options(nomem, preserves_flags),
		);
	}
}




#[macro_export]
macro_rules! lo {
	($($value:expr)*) => {
		((($($value)*) as u64) & 0xffff_ffff) as u32
	}
}
#[macro_export]
macro_rules! hi {
	($($value:expr)*) => {
		::nt_syscall::lo!((($($value)*) as u64) >> 0x20) as u32
	}
}



#[macro_export]
#[cfg(target_arch = "x86_64")]
macro_rules! syscall {
	( $( $($params:expr)* ),* ) => {
		::nt_syscall::syscall_x64!( $( $($params)* ),* )
	};
}
#[macro_export]
#[cfg(target_arch = "x86")]
macro_rules! syscall {
	( $( $($params:expr)* ),* ) => {
		if ::nt_syscall::cpu_mode!() == ::nt_syscall::CpuMode::EmulatedX86 {
			::nt_syscall::syscall_x86_emulated!( $( $($params)* ),* )
		} else {
			::nt_syscall::syscall_x86!( $( $($params)* ),* )
		}
	};
}




#[macro_export]
macro_rules! syscall_x64 {
	($($index:expr)*) => {{
		let mut status: u64;
		asm!(
			"sub rsp, 0x30",
			"syscall",
			"add rsp, 0x30",

			in("rax") ($($index)*) as u64,

			lateout("r10") _,
			lateout("rdx") _,
			lateout("r8") _,
			lateout("r9") _,
			lateout("r11") _,
			lateout("rcx") _,
			lateout("rax") status,
		);
		(status & 0xffff_ffff) as u32
	}};
	($($index:expr)*, $($p1:expr)*) => {{
		let mut status: u64;
		asm!(
			"sub rsp, 0x30",
			"syscall",
			"add rsp, 0x30",

			in("rax") ($($index)*) as u64,
			in("r10") ($($p1)*) as u64,
			
			lateout("r10") _,
			lateout("rdx") _,
			lateout("r8") _,
			lateout("r9") _,
			lateout("r11") _,
			lateout("rcx") _,
			lateout("rax") status,
		);
		(status & 0xffff_ffff) as u32
	}};
	($($index:expr)*, $($p1:expr)*, $($p2:expr)*) => {{
		let mut status: u64;
		asm!(
			"sub rsp, 0x30",
			"syscall",
			"add rsp, 0x30",

			in("rax") ($($index)*) as u64,
			in("r10") ($($p1)*) as u64,
			in("rdx") ($($p2)*) as u64,

			lateout("r10") _,
			lateout("rdx") _,
			lateout("r8") _,
			lateout("r9") _,
			lateout("r11") _,
			lateout("rcx") _,
			lateout("rax") status,
		);
		(status & 0xffff_ffff) as u32
	}};
	($($index:expr)*, $($p1:expr)*, $($p2:expr)*, $($p3:expr)*) => {{
		let mut status: u64;
		asm!(
			"sub rsp, 0x30",
			"syscall",
			"add rsp, 0x30",

			in("rax") ($($index)*) as u64,
			in("r10") ($($p1)*) as u64,
			in("rdx") ($($p2)*) as u64,
			in("r8")  ($($p3)*) as u64,

			lateout("r10") _,
			lateout("rdx") _,
			lateout("r8") _,
			lateout("r9") _,
			lateout("r11") _,
			lateout("rcx") _,
			lateout("rax") status,
		);
		(status & 0xffff_ffff) as u32
	}};
	($($index:expr)*, $($p1:expr)*, $($p2:expr)*, $($p3:expr)*, $($p4:expr)*) => {{
		let mut status: u64;
		asm!(
			"sub rsp, 0x30",
			"syscall",
			"add rsp, 0x30",

			in("rax") ($($index)*) as u64,
			in("r10") ($($p1)*) as u64,
			in("rdx") ($($p2)*) as u64,
			in("r8") ($($p3)*) as u64,
			in("r9") ($($p4)*) as u64,

			lateout("r10") _,
			lateout("rdx") _,
			lateout("r8") _,
			lateout("r9") _,
			lateout("r11") _,
			lateout("rcx") _,
			lateout("rax") status,
		);
		(status & 0xffff_ffff) as u32
	}};
	($($index:expr)*, $($p1:expr)*, $($p2:expr)*, $($p3:expr)*, $($p4:expr)*, $($p5:expr)*) => {{
		let mut status: u64;
		asm!(
			"sub rsp, 0x30",
			"mov qword ptr [rsp + 0x28], {p5}",

			"syscall",
			"add rsp, 0x30",

			p5 = in(reg) ($($p5)*) as u64,

			in("rax") ($($index)*) as u64,
			in("r10") ($($p1)*) as u64,
			in("rdx") ($($p2)*) as u64,
			in("r8") ($($p3)*) as u64,
			in("r9") ($($p4)*) as u64,
			
			lateout("r10") _,
			lateout("rdx") _,
			lateout("r8") _,
			lateout("r9") _,
			lateout("r11") _,
			lateout("rcx") _,
			lateout("rax") status,
		);
		(status & 0xffff_ffff) as u32
	}};
	($($index:expr)*, $($p1:expr)*, $($p2:expr)*, $($p3:expr)*, $($p4:expr)*, $($p5:expr)*, $($p6:expr)*) => {{
		let mut status: u64;
		asm!(
			"sub rsp, 0x40",
			"mov qword ptr [rsp + 0x28], {p5}",
			"mov qword ptr [rsp + 0x30], {p6}",

			"syscall",
			"add rsp, 0x40",

			p5 = in(reg) ($($p5)*) as u64,
			p6 = in(reg) ($($p6)*) as u64,

			in("rax") ($($index)*) as u64,
			in("r10") ($($p1)*) as u64,
			in("rdx") ($($p2)*) as u64,
			in("r8") ($($p3)*) as u64,
			in("r9") ($($p4)*) as u64,
			
			lateout("r10") _,
			lateout("rdx") _,
			lateout("r8") _,
			lateout("r9") _,
			lateout("r11") _,
			lateout("rcx") _,
			lateout("rax") status,
		);
		(status & 0xffff_ffff) as u32
	}};
	($($index:expr)*, $($p1:expr)*, $($p2:expr)*, $($p3:expr)*, $($p4:expr)*, $($p5:expr)*, $($p6:expr)*, $($p7:expr)*) => {{
		let mut status: u64;
		asm!(
			"sub rsp, 0x40",
			"mov qword ptr [rsp + 0x28], {p5}",
			"mov qword ptr [rsp + 0x30], {p6}",
			"mov qword ptr [rsp + 0x38], {p7}",

			"syscall",
			"add rsp, 0x40",

			p5 = in(reg) ($($p5)*) as u64,
			p6 = in(reg) ($($p6)*) as u64,
			p7 = in(reg) ($($p7)*) as u64,

			in("rax") ($($index)*) as u64,
			in("r10") ($($p1)*) as u64,
			in("rdx") ($($p2)*) as u64,
			in("r8") ($($p3)*) as u64,
			in("r9") ($($p4)*) as u64,
			
			lateout("r10") _,
			lateout("rdx") _,
			lateout("r8") _,
			lateout("r9") _,
			lateout("r11") _,
			lateout("rcx") _,
			lateout("rax") status,
		);
		(status & 0xffff_ffff) as u32
	}};
	($($index:expr)*, $($p1:expr)*, $($p2:expr)*, $($p3:expr)*, $($p4:expr)*, $($p5:expr)*, $($p6:expr)*, $($p7:expr)*, $($p8:expr)*) => {{
		let mut status: u64;
		asm!(
			"sub rsp, 0x50",
			"mov qword ptr [rsp + 0x28], {p5}",
			"mov qword ptr [rsp + 0x30], {p6}",
			"mov qword ptr [rsp + 0x38], {p7}",
			"mov qword ptr [rsp + 0x40], {p8}",
			
			"syscall",
			"add rsp, 0x50",

			p5 = in(reg) ($($p5)*) as u64,
			p6 = in(reg) ($($p6)*) as u64,
			p7 = in(reg) ($($p7)*) as u64,
			p8 = in(reg) ($($p8)*) as u64,

			in("rax") ($($index)*) as u64,
			in("r10") ($($p1)*) as u64,
			in("rdx") ($($p2)*) as u64,
			in("r8") ($($p3)*) as u64,
			in("r9") ($($p4)*) as u64,
			
			lateout("r10") _,
			lateout("rdx") _,
			lateout("r8") _,
			lateout("r9") _,
			lateout("r11") _,
			lateout("rcx") _,
			lateout("rax") status,
		);
		(status & 0xffff_ffff) as u32
	}};
	($($index:expr)*, $($p1:expr)*, $($p2:expr)*, $($p3:expr)*, $($p4:expr)*, $($p5:expr)*, $($p6:expr)*, $($p7:expr)*, $($p8:expr)*, $($p9:expr)*) => {{
		let mut status: u64;
		asm!(
			"sub rsp, 0x50",
			"mov qword ptr [rsp + 0x28], {p5}",
			"mov qword ptr [rsp + 0x30], {p6}",
			"mov qword ptr [rsp + 0x38], {p7}",
			"mov qword ptr [rsp + 0x40], {p8}",
			"mov qword ptr [rsp + 0x48], {p9}",
			
			"syscall",
			"add rsp, 0x50",

			p5 = in(reg) ($($p5)*) as u64,
			p6 = in(reg) ($($p6)*) as u64,
			p7 = in(reg) ($($p7)*) as u64,
			p8 = in(reg) ($($p8)*) as u64,
			p9 = in(reg) ($($p9)*) as u64,

			in("rax") ($($index)*) as u64,
			in("r10") ($($p1)*) as u64,
			in("rdx") ($($p2)*) as u64,
			in("r8") ($($p3)*) as u64,
			in("r9") ($($p4)*) as u64,
			
			lateout("r10") _,
			lateout("rdx") _,
			lateout("r8") _,
			lateout("r9") _,
			lateout("r11") _,
			lateout("rcx") _,
			lateout("rax") status,
		);
		(status & 0xffff_ffff) as u32
	}};
	($($index:expr)*, $($p1:expr)*, $($p2:expr)*, $($p3:expr)*, $($p4:expr)*, $($p5:expr)*, $($p6:expr)*, $($p7:expr)*, $($p8:expr)*, $($p9:expr)*, $($p10:expr)*) => {{
		let mut status: u64;
		asm!(
			"sub rsp, 0x60",
			"mov qword ptr [rsp + 0x28], {p5}",
			"mov qword ptr [rsp + 0x30], {p6}",
			"mov qword ptr [rsp + 0x38], {p7}",
			"mov qword ptr [rsp + 0x40], {p8}",
			"mov qword ptr [rsp + 0x48], {p9}",
			"mov qword ptr [rsp + 0x50], {p10}",
			
			"syscall",
			"add rsp, 0x60",

			p5 = in(reg) ($($p5)*) as u64,
			p6 = in(reg) ($($p6)*) as u64,
			p7 = in(reg) ($($p7)*) as u64,
			p8 = in(reg) ($($p8)*) as u64,
			p9 = in(reg) ($($p9)*) as u64,
			p10 = in(reg) ($($p10)*) as u64,

			in("rax") ($($index)*) as u64,
			in("r10") ($($p1)*) as u64,
			in("rdx") ($($p2)*) as u64,
			in("r8") ($($p3)*) as u64,
			in("r9") ($($p4)*) as u64,
			
			lateout("r10") _,
			lateout("rdx") _,
			lateout("r8") _,
			lateout("r9") _,
			lateout("r11") _,
			lateout("rcx") _,
			lateout("rax") status,
		);
		(status & 0xffff_ffff) as u32
	}};
	($($index:expr)*, $($p1:expr)*, $($p2:expr)*, $($p3:expr)*, $($p4:expr)*, $($p5:expr)*, $($p6:expr)*, $($p7:expr)*, $($p8:expr)*, $($p9:expr)*, $($p10:expr)*, $($p11:expr)*) => {{
		let mut status: u64;
		asm!(
			"sub rsp, 0x60",
			"mov qword ptr [rsp + 0x28], {p5}",
			"mov qword ptr [rsp + 0x30], {p6}",
			"mov qword ptr [rsp + 0x38], {p7}",
			"mov qword ptr [rsp + 0x40], {p8}",
			"mov qword ptr [rsp + 0x48], {p9}",
			"mov qword ptr [rsp + 0x50], {p10}",
			"mov qword ptr [rsp + 0x58], {p11}",
			
			"syscall",
			"add rsp, 0x60h",

			p5 = in(reg) ($($p5)*) as u64,
			p6 = in(reg) ($($p6)*) as u64,
			p7 = in(reg) ($($p7)*) as u64,
			p8 = in(reg) ($($p8)*) as u64,
			p9 = in(reg) ($($p9)*) as u64,
			p10 = in(reg) ($($p10)*) as u64,
			p11 = in(reg) ($($p11)*) as u64,

			in("rax") ($($index)*) as u64,
			in("r10") ($($p1)*) as u64,
			in("rdx") ($($p2)*) as u64,
			in("r8") ($($p3)*) as u64,
			in("r9") ($($p4)*) as u64,
			
			lateout("r10") _,
			lateout("rdx") _,
			lateout("r8") _,
			lateout("r9") _,
			lateout("r11") _,
			lateout("rcx") _,
			lateout("rax") status,
		);
		(status & 0xffff_ffff) as u32
	}};
	($($index:expr)*, $($p1:expr)*, $($p2:expr)*, $($p3:expr)*, $($p4:expr)*, $($p5:expr)*, $($p6:expr)*, $($p7:expr)*, $($p8:expr)*, $($p9:expr)*, $($p10:expr)*, $($p11:expr)*, $($p12:expr)*) => {{
		let mut status: u64;
		asm!(
			"sub rsp, 0x70",
			"mov qword ptr [rsp + 0x28], {p5}",
			"mov qword ptr [rsp + 0x30], {p6}",
			"mov qword ptr [rsp + 0x38], {p7}",
			"mov qword ptr [rsp + 0x40], {p8}",
			"mov qword ptr [rsp + 0x48], {p9}",
			"mov qword ptr [rsp + 0x50], {p10}",
			"mov qword ptr [rsp + 0x58], {p11}",
			"mov qword ptr [rsp + 0x60], {p12}",
			
			"syscall",
			"add rsp, 0x70h",

			p5 = in(reg) ($($p5)*) as u64,
			p6 = in(reg) ($($p6)*) as u64,
			p7 = in(reg) ($($p7)*) as u64,
			p8 = in(reg) ($($p8)*) as u64,
			p9 = in(reg) ($($p9)*) as u64,
			p10 = in(reg) ($($p10)*) as u64,
			p11 = in(reg) ($($p11)*) as u64,
			p12 = in(reg) ($($p12)*) as u64,

			in("rax") ($($index)*) as u64,
			in("r10") ($($p1)*) as u64,
			in("rdx") ($($p2)*) as u64,
			in("r8") ($($p3)*) as u64,
			in("r9") ($($p4)*) as u64,
			
			lateout("r10") _,
			lateout("rdx") _,
			lateout("r8") _,
			lateout("r9") _,
			lateout("r11") _,
			lateout("rcx") _,
			lateout("rax") status,
		);
		(status & 0xffff_ffff) as u32
	}};
	($($index:expr)*, $($p1:expr)*, $($p2:expr)*, $($p3:expr)*, $($p4:expr)*, $($p5:expr)*, $($p6:expr)*, $($p7:expr)*, $($p8:expr)*, $($p9:expr)*, $($p10:expr)*, $($p11:expr)*, $($p12:expr)*, $($p13:expr)*) => {{
		let mut status: u64;
		asm!(
			"sub rsp, 0x70",
			"mov qword ptr [rsp + 0x28], {p5}",
			"mov qword ptr [rsp + 0x30], {p6}",
			"mov qword ptr [rsp + 0x38], {p7}",
			"mov qword ptr [rsp + 0x40], {p8}",
			"mov qword ptr [rsp + 0x48], {p9}",
			"mov qword ptr [rsp + 0x50], {p10}",
			"mov qword ptr [rsp + 0x58], {p11}",
			"mov qword ptr [rsp + 0x60], {p12}",
			"mov qword ptr [rsp + 0x60], {p13}",
			
			"syscall",
			"add rsp, 0x70h",

			p5 = in(reg) ($($p5)*) as u64,
			p6 = in(reg) ($($p6)*) as u64,
			p7 = in(reg) ($($p7)*) as u64,
			p8 = in(reg) ($($p8)*) as u64,
			p9 = in(reg) ($($p9)*) as u64,
			p10 = in(reg) ($($p10)*) as u64,
			p11 = in(reg) ($($p11)*) as u64,
			p12 = in(reg) ($($p12)*) as u64,
			p13 = in(reg) ($($p13)*) as u64,

			in("rax") ($($index)*) as u64,
			in("r10") ($($p1)*) as u64,
			in("rdx") ($($p2)*) as u64,
			in("r8") ($($p3)*) as u64,
			in("r9") ($($p4)*) as u64,
			
			lateout("r10") _,
			lateout("rdx") _,
			lateout("r8") _,
			lateout("r9") _,
			lateout("r11") _,
			lateout("rcx") _,
			lateout("rax") status,
		);
		(status & 0xffff_ffff) as u32
	}};
}




#[macro_export]
macro_rules! syscall_x86_emulated {
	($($index:expr)*) => {{
		let index: u64 = ($($index)*) as u64;

		let mut aligned: u32;
		let mut status: u32;
		unsafe {
			asm!(
				// Align copy stack
				"mov ecx, esp",
				"and ecx, 0xfffffff0",
				"sub ecx, 0x30",

				out("ecx") aligned,
				options(pure, nomem, nostack, preserves_flags),
			);
			asm!(
				// Prepare stack spill space
				"mov dword ptr [ecx + 0x00], {index_lo}",
				"mov dword ptr [ecx + 0x04], {index_hi}",

				index_lo = in(reg) ::nt_syscall::lo!(index),
				index_hi = in(reg) ::nt_syscall::hi!(index),
				in("ecx") aligned,
				options(preserves_flags),
			);
			::nt_syscall::enter_x64!();
			asm!(
				// Align real stack
				"mov ebx, esp",
				"mov esp, ecx",

				// Prepare registers
				".byte 0x48", "mov eax, [ecx + 0x00]", // rax
				".byte 0x4c", "mov edx, [ecx + 0x08]", // r10
				".byte 0x48", "mov edx, [ecx + 0x10]", // rdx
				".byte 0x4c", "mov eax, [ecx + 0x18]", // r8
				".byte 0x4c", "mov ecx, [ecx + 0x20]", // r9

				// Syscall and restore stack
				"syscall",
				"mov esp, ebx",
				
				in("ecx") aligned,
				lateout("ebx") _,
				lateout("ecx") _,
				lateout("edx") _,
				lateout("eax") status,
			);
			::nt_syscall::enter_x86!();
		}

		status
	}};
	($($index:expr)*, $($p1:expr)*) => {{
		let index: u64 = ($($index)*) as u64;
		let p1: u64 = ($($p1)*) as u64;

		let mut aligned: u32;
		let mut status: u32;
		unsafe {
			asm!(
				// Align copy stack
				"mov ecx, esp",
				"and ecx, 0xfffffff0",
				"sub ecx, 0x30",

				out("ecx") aligned,
				options(pure, nomem, nostack, preserves_flags),
			);
			asm!(
				// Prepare stack spill space
				"mov dword ptr [ecx + 0x00], {index_lo}",
				"mov dword ptr [ecx + 0x04], {index_hi}",
				"mov dword ptr [ecx + 0x08], {p1_lo}",
				"mov dword ptr [ecx + 0x0c], {p1_hi}",

				index_lo = in(reg) ::nt_syscall::lo!(index),
				index_hi = in(reg) ::nt_syscall::hi!(index),
				p1_lo = in(reg) ::nt_syscall::lo!(p1),
				p1_hi = in(reg) ::nt_syscall::hi!(p1),
				in("ecx") aligned,
				options(preserves_flags),
			);
			::nt_syscall::enter_x64!();
			asm!(
				// Align real stack
				"mov ebx, esp",
				"mov esp, ecx",

				// Prepare registers
				".byte 0x48", "mov eax, [ecx + 0x00]", // rax
				".byte 0x4c", "mov edx, [ecx + 0x08]", // r10
				".byte 0x48", "mov edx, [ecx + 0x10]", // rdx
				".byte 0x4c", "mov eax, [ecx + 0x18]", // r8
				".byte 0x4c", "mov ecx, [ecx + 0x20]", // r9

				// Syscall and restore stack
				"syscall",
				"mov esp, ebx",
				
				in("ecx") aligned,
				lateout("ebx") _,
				lateout("ecx") _,
				lateout("edx") _,
				lateout("eax") status,
			);
			::nt_syscall::enter_x86!();
		}

		status
	}};
	($($index:expr)*, $($p1:expr)*, $($p2:expr)*) => {{
		let index: u64 = ($($index)*) as u64;
		let p1: u64 = ($($p1)*) as u64;
		let p2: u64 = ($($p2)*) as u64;

		let mut aligned: u32;
		let mut status: u32;
		unsafe {
			asm!(
				// Align copy stack
				"mov ecx, esp",
				"and ecx, 0xfffffff0",
				"sub ecx, 0x30",

				out("ecx") aligned,
				options(pure, nomem, nostack, preserves_flags),
			);
			asm!(
				// Prepare stack spill space
				"mov dword ptr [ecx + 0x00], {index_lo}",
				"mov dword ptr [ecx + 0x04], {index_hi}",
				"mov dword ptr [ecx + 0x08], {p1_lo}",
				"mov dword ptr [ecx + 0x0c], {p1_hi}",

				index_lo = in(reg) ::nt_syscall::lo!(index),
				index_hi = in(reg) ::nt_syscall::hi!(index),
				p1_lo = in(reg) ::nt_syscall::lo!(p1),
				p1_hi = in(reg) ::nt_syscall::hi!(p1),
				in("ecx") aligned,
				options(preserves_flags),
			);
			asm!(
				"mov dword ptr [ecx + 0x10], {p2_lo}",
				"mov dword ptr [ecx + 0x14], {p2_hi}",

				p2_lo = in(reg) ::nt_syscall::lo!(p2),
				p2_hi = in(reg) ::nt_syscall::hi!(p2),
				in("ecx") aligned,
				options(preserves_flags),
			);
			::nt_syscall::enter_x64!();
			asm!(
				// Align real stack
				"mov ebx, esp",
				"mov esp, ecx",

				// Prepare registers
				".byte 0x48", "mov eax, [ecx + 0x00]", // rax
				".byte 0x4c", "mov edx, [ecx + 0x08]", // r10
				".byte 0x48", "mov edx, [ecx + 0x10]", // rdx
				".byte 0x4c", "mov eax, [ecx + 0x18]", // r8
				".byte 0x4c", "mov ecx, [ecx + 0x20]", // r9

				// Syscall and restore stack
				"syscall",
				"mov esp, ebx",
				
				in("ecx") aligned,
				lateout("ebx") _,
				lateout("ecx") _,
				lateout("edx") _,
				lateout("eax") status,
			);
			::nt_syscall::enter_x86!();
		}

		status
	}};
	($($index:expr)*, $($p1:expr)*, $($p2:expr)*, $($p3:expr)*) => {{
		let index: u64 = ($($index)*) as u64;
		let p1: u64 = ($($p1)*) as u64;
		let p2: u64 = ($($p2)*) as u64;
		let p3: u64 = ($($p3)*) as u64;

		let mut aligned: u32;
		let mut status: u32;
		unsafe {
			asm!(
				// Align copy stack
				"mov ecx, esp",
				"and ecx, 0xfffffff0",
				"sub ecx, 0x30",

				out("ecx") aligned,
				options(pure, nomem, nostack, preserves_flags),
			);
			asm!(
				// Prepare stack spill space
				"mov dword ptr [ecx + 0x00], {index_lo}",
				"mov dword ptr [ecx + 0x04], {index_hi}",
				"mov dword ptr [ecx + 0x08], {p1_lo}",
				"mov dword ptr [ecx + 0x0c], {p1_hi}",

				index_lo = in(reg) ::nt_syscall::lo!(index),
				index_hi = in(reg) ::nt_syscall::hi!(index),
				p1_lo = in(reg) ::nt_syscall::lo!(p1),
				p1_hi = in(reg) ::nt_syscall::hi!(p1),
				in("ecx") aligned,
				options(preserves_flags),
			);
			asm!(
				"mov dword ptr [ecx + 0x10], {p2_lo}",
				"mov dword ptr [ecx + 0x14], {p2_hi}",
				"mov dword ptr [ecx + 0x18], {p3_lo}",
				"mov dword ptr [ecx + 0x1c], {p3_hi}",

				p2_lo = in(reg) ::nt_syscall::lo!(p2),
				p2_hi = in(reg) ::nt_syscall::hi!(p2),
				p3_lo = in(reg) ::nt_syscall::lo!(p3),
				p3_hi = in(reg) ::nt_syscall::hi!(p3),
				in("ecx") aligned,
				options(preserves_flags),
			);
			::nt_syscall::enter_x64!();
			asm!(
				// Align real stack
				"mov ebx, esp",
				"mov esp, ecx",

				// Prepare registers
				".byte 0x48", "mov eax, [ecx + 0x00]", // rax
				".byte 0x4c", "mov edx, [ecx + 0x08]", // r10
				".byte 0x48", "mov edx, [ecx + 0x10]", // rdx
				".byte 0x4c", "mov eax, [ecx + 0x18]", // r8
				".byte 0x4c", "mov ecx, [ecx + 0x20]", // r9

				// Syscall and restore stack
				"syscall",
				"mov esp, ebx",
				
				in("ecx") aligned,
				lateout("ebx") _,
				lateout("ecx") _,
				lateout("edx") _,
				lateout("eax") status,
			);
			::nt_syscall::enter_x86!();
		}

		status
	}};
	($($index:expr)*, $($p1:expr)*, $($p2:expr)*, $($p3:expr)*, $($p4:expr)*) => {{
		let index: u64 = ($($index)*) as u64;
		let p1: u64 = ($($p1)*) as u64;
		let p2: u64 = ($($p2)*) as u64;
		let p3: u64 = ($($p3)*) as u64;
		let p4: u64 = ($($p4)*) as u64;

		let mut aligned: u32;
		let mut status: u32;
		unsafe {
			asm!(
				// Align copy stack
				"mov ecx, esp",
				"and ecx, 0xfffffff0",
				"sub ecx, 0x30",

				out("ecx") aligned,
				options(pure, nomem, nostack, preserves_flags),
			);
			asm!(
				// Prepare stack spill space
				"mov dword ptr [ecx + 0x00], {index_lo}",
				"mov dword ptr [ecx + 0x04], {index_hi}",
				"mov dword ptr [ecx + 0x08], {p1_lo}",
				"mov dword ptr [ecx + 0x0c], {p1_hi}",

				index_lo = in(reg) ::nt_syscall::lo!(index),
				index_hi = in(reg) ::nt_syscall::hi!(index),
				p1_lo = in(reg) ::nt_syscall::lo!(p1),
				p1_hi = in(reg) ::nt_syscall::hi!(p1),
				in("ecx") aligned,
				options(preserves_flags),
			);
			asm!(
				"mov dword ptr [ecx + 0x10], {p2_lo}",
				"mov dword ptr [ecx + 0x14], {p2_hi}",
				"mov dword ptr [ecx + 0x18], {p3_lo}",
				"mov dword ptr [ecx + 0x1c], {p3_hi}",

				p2_lo = in(reg) ::nt_syscall::lo!(p2),
				p2_hi = in(reg) ::nt_syscall::hi!(p2),
				p3_lo = in(reg) ::nt_syscall::lo!(p3),
				p3_hi = in(reg) ::nt_syscall::hi!(p3),
				in("ecx") aligned,
				options(preserves_flags),
			);
			asm!(
				"mov dword ptr [ecx + 0x20], {p4_lo}",
				"mov dword ptr [ecx + 0x24], {p4_hi}",

				p4_lo = in(reg) ::nt_syscall::lo!(p4),
				p4_hi = in(reg) ::nt_syscall::hi!(p4),
				in("ecx") aligned,
				options(preserves_flags),
			);
			::nt_syscall::enter_x64!();
			asm!(
				// Align real stack
				"mov ebx, esp",
				"mov esp, ecx",

				// Prepare registers
				".byte 0x48", "mov eax, [ecx + 0x00]", // rax
				".byte 0x4c", "mov edx, [ecx + 0x08]", // r10
				".byte 0x48", "mov edx, [ecx + 0x10]", // rdx
				".byte 0x4c", "mov eax, [ecx + 0x18]", // r8
				".byte 0x4c", "mov ecx, [ecx + 0x20]", // r9

				// Syscall and restore stack
				"syscall",
				"mov esp, ebx",
				
				in("ecx") aligned,
				lateout("ebx") _,
				lateout("ecx") _,
				lateout("edx") _,
				lateout("eax") status,
			);
			::nt_syscall::enter_x86!();
		}

		status
	}};
	($($index:expr)*, $($p1:expr)*, $($p2:expr)*, $($p3:expr)*, $($p4:expr)*, $($p5:expr)*) => {{
		let index: u64 = ($($index)*) as u64;
		let p1: u64 = ($($p1)*) as u64;
		let p2: u64 = ($($p2)*) as u64;
		let p3: u64 = ($($p3)*) as u64;
		let p4: u64 = ($($p4)*) as u64;
		let p5: u64 = ($($p5)*) as u64;

		let mut aligned: u32;
		let mut status: u32;
		unsafe {
			asm!(
				// Align copy stack
				"mov ecx, esp",
				"and ecx, 0xfffffff0",
				"sub ecx, 0x40", // Add extra space in case of down alignment of esp

				out("ecx") aligned,
				options(pure, nomem, nostack, preserves_flags),
			);
			asm!(
				// Prepare stack spill space
				"mov dword ptr [ecx + 0x00], {index_lo}",
				"mov dword ptr [ecx + 0x04], {index_hi}",
				"mov dword ptr [ecx + 0x08], {p1_lo}",
				"mov dword ptr [ecx + 0x0c], {p1_hi}",

				index_lo = in(reg) ::nt_syscall::lo!(index),
				index_hi = in(reg) ::nt_syscall::hi!(index),
				p1_lo = in(reg) ::nt_syscall::lo!(p1),
				p1_hi = in(reg) ::nt_syscall::hi!(p1),
				in("ecx") aligned,
				options(preserves_flags),
			);
			asm!(
				"mov dword ptr [ecx + 0x10], {p2_lo}",
				"mov dword ptr [ecx + 0x14], {p2_hi}",
				"mov dword ptr [ecx + 0x18], {p3_lo}",
				"mov dword ptr [ecx + 0x1c], {p3_hi}",

				p2_lo = in(reg) ::nt_syscall::lo!(p2),
				p2_hi = in(reg) ::nt_syscall::hi!(p2),
				p3_lo = in(reg) ::nt_syscall::lo!(p3),
				p3_hi = in(reg) ::nt_syscall::hi!(p3),
				in("ecx") aligned,
				options(preserves_flags),
			);
			asm!(
				"mov dword ptr [ecx + 0x20], {p4_lo}",
				"mov dword ptr [ecx + 0x24], {p4_hi}",

				p4_lo = in(reg) ::nt_syscall::lo!(p4),
				p4_hi = in(reg) ::nt_syscall::hi!(p4),
				in("ecx") aligned,
				options(preserves_flags),
			);
			asm!(
				// Prepare used part of the stack
				"mov dword ptr [ecx + 0x28], {p5_lo}",
				"mov dword ptr [ecx + 0x2c], {p5_hi}",

				p5_lo = in(reg) ::nt_syscall::lo!(p5),
				p5_hi = in(reg) ::nt_syscall::hi!(p5),
				in("ecx") aligned,
				options(preserves_flags),
			);
			::nt_syscall::enter_x64!();
			asm!(
				// Align real stack
				"mov ebx, esp",
				"mov esp, ecx",

				// Prepare registers
				".byte 0x48", "mov eax, [ecx + 0x00]", // rax
				".byte 0x4c", "mov edx, [ecx + 0x08]", // r10
				".byte 0x48", "mov edx, [ecx + 0x10]", // rdx
				".byte 0x4c", "mov eax, [ecx + 0x18]", // r8
				".byte 0x4c", "mov ecx, [ecx + 0x20]", // r9

				// Syscall and restore stack
				"syscall",
				"mov esp, ebx",
				
				in("ecx") aligned,
				lateout("ebx") _,
				lateout("ecx") _,
				lateout("edx") _,
				lateout("eax") status,
			);
			::nt_syscall::enter_x86!();
		}

		status
	}};
	($($index:expr)*, $($p1:expr)*, $($p2:expr)*, $($p3:expr)*, $($p4:expr)*, $($p5:expr)*, $($p6:expr)*) => {{
		let index: u64 = ($($index)*) as u64;
		let p1: u64 = ($($p1)*) as u64;
		let p2: u64 = ($($p2)*) as u64;
		let p3: u64 = ($($p3)*) as u64;
		let p4: u64 = ($($p4)*) as u64;
		let p5: u64 = ($($p5)*) as u64;
		let p6: u64 = ($($p6)*) as u64;

		let mut aligned: u32;
		let mut status: u32;
		unsafe {
			asm!(
				// Align copy stack
				"mov ecx, esp",
				"and ecx, 0xfffffff0",
				"sub ecx, 0x40",

				out("ecx") aligned,
				options(pure, nomem, nostack, preserves_flags),
			);
			asm!(
				// Prepare stack spill space
				"mov dword ptr [ecx + 0x00], {index_lo}",
				"mov dword ptr [ecx + 0x04], {index_hi}",
				"mov dword ptr [ecx + 0x08], {p1_lo}",
				"mov dword ptr [ecx + 0x0c], {p1_hi}",

				index_lo = in(reg) ::nt_syscall::lo!(index),
				index_hi = in(reg) ::nt_syscall::hi!(index),
				p1_lo = in(reg) ::nt_syscall::lo!(p1),
				p1_hi = in(reg) ::nt_syscall::hi!(p1),
				in("ecx") aligned,
				options(preserves_flags),
			);
			asm!(
				"mov dword ptr [ecx + 0x10], {p2_lo}",
				"mov dword ptr [ecx + 0x14], {p2_hi}",
				"mov dword ptr [ecx + 0x18], {p3_lo}",
				"mov dword ptr [ecx + 0x1c], {p3_hi}",

				p2_lo = in(reg) ::nt_syscall::lo!(p2),
				p2_hi = in(reg) ::nt_syscall::hi!(p2),
				p3_lo = in(reg) ::nt_syscall::lo!(p3),
				p3_hi = in(reg) ::nt_syscall::hi!(p3),
				in("ecx") aligned,
				options(preserves_flags),
			);
			asm!(
				"mov dword ptr [ecx + 0x20], {p4_lo}",
				"mov dword ptr [ecx + 0x24], {p4_hi}",

				p4_lo = in(reg) ::nt_syscall::lo!(p4),
				p4_hi = in(reg) ::nt_syscall::hi!(p4),
				in("ecx") aligned,
				options(preserves_flags),
			);
			asm!(
				// Prepare used part of the stack
				"mov dword ptr [ecx + 0x28], {p5_lo}",
				"mov dword ptr [ecx + 0x2c], {p5_hi}",
				"mov dword ptr [ecx + 0x30], {p6_lo}",
				"mov dword ptr [ecx + 0x34], {p6_hi}",

				p5_lo = in(reg) ::nt_syscall::lo!(p5),
				p5_hi = in(reg) ::nt_syscall::hi!(p5),
				p6_lo = in(reg) ::nt_syscall::lo!(p6),
				p6_hi = in(reg) ::nt_syscall::hi!(p6),
				in("ecx") aligned,
				options(preserves_flags),
			);
			::nt_syscall::enter_x64!();
			asm!(
				// Align real stack
				"mov ebx, esp",
				"mov esp, ecx",

				// Prepare registers
				".byte 0x48", "mov eax, [ecx + 0x00]", // rax
				".byte 0x4c", "mov edx, [ecx + 0x08]", // r10
				".byte 0x48", "mov edx, [ecx + 0x10]", // rdx
				".byte 0x4c", "mov eax, [ecx + 0x18]", // r8
				".byte 0x4c", "mov ecx, [ecx + 0x20]", // r9

				// Syscall and restore stack
				"syscall",
				"mov esp, ebx",
				
				in("ecx") aligned,
				lateout("ebx") _,
				lateout("ecx") _,
				lateout("edx") _,
				lateout("eax") status,
			);
			::nt_syscall::enter_x86!();
		}

		status
	}};
	($($index:expr)*, $($p1:expr)*, $($p2:expr)*, $($p3:expr)*, $($p4:expr)*, $($p5:expr)*, $($p6:expr)*, $($p7:expr)*) => {{
		let index: u64 = ($($index)*) as u64;
		let p1: u64 = ($($p1)*) as u64;
		let p2: u64 = ($($p2)*) as u64;
		let p3: u64 = ($($p3)*) as u64;
		let p4: u64 = ($($p4)*) as u64;
		let p5: u64 = ($($p5)*) as u64;
		let p6: u64 = ($($p6)*) as u64;
		let p7: u64 = ($($p7)*) as u64;

		let mut aligned: u32;
		let mut status: u32;
		unsafe {
			asm!(
				// Align copy stack
				"mov ecx, esp",
				"and ecx, 0xfffffff0",
				"sub ecx, 0x50", // Add extra space in case of down alignment of esp

				out("ecx") aligned,
				options(pure, nomem, nostack, preserves_flags),
			);
			asm!(
				// Prepare stack spill space
				"mov dword ptr [ecx + 0x00], {index_lo}",
				"mov dword ptr [ecx + 0x04], {index_hi}",
				"mov dword ptr [ecx + 0x08], {p1_lo}",
				"mov dword ptr [ecx + 0x0c], {p1_hi}",

				index_lo = in(reg) ::nt_syscall::lo!(index),
				index_hi = in(reg) ::nt_syscall::hi!(index),
				p1_lo = in(reg) ::nt_syscall::lo!(p1),
				p1_hi = in(reg) ::nt_syscall::hi!(p1),
				in("ecx") aligned,
				options(preserves_flags),
			);
			asm!(
				"mov dword ptr [ecx + 0x10], {p2_lo}",
				"mov dword ptr [ecx + 0x14], {p2_hi}",
				"mov dword ptr [ecx + 0x18], {p3_lo}",
				"mov dword ptr [ecx + 0x1c], {p3_hi}",

				p2_lo = in(reg) ::nt_syscall::lo!(p2),
				p2_hi = in(reg) ::nt_syscall::hi!(p2),
				p3_lo = in(reg) ::nt_syscall::lo!(p3),
				p3_hi = in(reg) ::nt_syscall::hi!(p3),
				in("ecx") aligned,
				options(preserves_flags),
			);
			asm!(
				"mov dword ptr [ecx + 0x20], {p4_lo}",
				"mov dword ptr [ecx + 0x24], {p4_hi}",

				p4_lo = in(reg) ::nt_syscall::lo!(p4),
				p4_hi = in(reg) ::nt_syscall::hi!(p4),
				in("ecx") aligned,
				options(preserves_flags),
			);
			asm!(
				// Prepare used part of the stack
				"mov dword ptr [ecx + 0x28], {p5_lo}",
				"mov dword ptr [ecx + 0x2c], {p5_hi}",
				"mov dword ptr [ecx + 0x30], {p6_lo}",
				"mov dword ptr [ecx + 0x34], {p6_hi}",

				p5_lo = in(reg) ::nt_syscall::lo!(p5),
				p5_hi = in(reg) ::nt_syscall::hi!(p5),
				p6_lo = in(reg) ::nt_syscall::lo!(p6),
				p6_hi = in(reg) ::nt_syscall::hi!(p6),
				in("ecx") aligned,
				options(preserves_flags),
			);
			asm!(
				"mov dword ptr [ecx + 0x38], {p7_lo}",
				"mov dword ptr [ecx + 0x3c], {p7_hi}",

				p7_lo = in(reg) ::nt_syscall::lo!(p7),
				p7_hi = in(reg) ::nt_syscall::hi!(p7),
				in("ecx") aligned,
				options(preserves_flags),
			);
			::nt_syscall::enter_x64!();
			asm!(
				// Align real stack
				"mov ebx, esp",
				"mov esp, ecx",

				// Prepare registers
				".byte 0x48", "mov eax, [ecx + 0x00]", // rax
				".byte 0x4c", "mov edx, [ecx + 0x08]", // r10
				".byte 0x48", "mov edx, [ecx + 0x10]", // rdx
				".byte 0x4c", "mov eax, [ecx + 0x18]", // r8
				".byte 0x4c", "mov ecx, [ecx + 0x20]", // r9

				// Syscall and restore stack
				"syscall",
				"mov esp, ebx",
				
				in("ecx") aligned,
				lateout("ebx") _,
				lateout("ecx") _,
				lateout("edx") _,
				lateout("eax") status,
			);
			::nt_syscall::enter_x86!();
		}

		status
	}};
	($($index:expr)*, $($p1:expr)*, $($p2:expr)*, $($p3:expr)*, $($p4:expr)*, $($p5:expr)*, $($p6:expr)*, $($p7:expr)*, $($p8:expr)*) => {{
		let index: u64 = ($($index)*) as u64;
		let p1: u64 = ($($p1)*) as u64;
		let p2: u64 = ($($p2)*) as u64;
		let p3: u64 = ($($p3)*) as u64;
		let p4: u64 = ($($p4)*) as u64;
		let p5: u64 = ($($p5)*) as u64;
		let p6: u64 = ($($p6)*) as u64;
		let p7: u64 = ($($p7)*) as u64;
		let p8: u64 = ($($p8)*) as u64;

		let mut aligned: u32;
		let mut status: u32;
		unsafe {
			asm!(
				// Align copy stack
				"mov ecx, esp",
				"and ecx, 0xfffffff0",
				"sub ecx, 0x50",

				out("ecx") aligned,
				options(pure, nomem, nostack, preserves_flags),
			);
			asm!(
				// Prepare stack spill space
				"mov dword ptr [ecx + 0x00], {index_lo}",
				"mov dword ptr [ecx + 0x04], {index_hi}",
				"mov dword ptr [ecx + 0x08], {p1_lo}",
				"mov dword ptr [ecx + 0x0c], {p1_hi}",

				index_lo = in(reg) ::nt_syscall::lo!(index),
				index_hi = in(reg) ::nt_syscall::hi!(index),
				p1_lo = in(reg) ::nt_syscall::lo!(p1),
				p1_hi = in(reg) ::nt_syscall::hi!(p1),
				in("ecx") aligned,
				options(preserves_flags),
			);
			asm!(
				"mov dword ptr [ecx + 0x10], {p2_lo}",
				"mov dword ptr [ecx + 0x14], {p2_hi}",
				"mov dword ptr [ecx + 0x18], {p3_lo}",
				"mov dword ptr [ecx + 0x1c], {p3_hi}",

				p2_lo = in(reg) ::nt_syscall::lo!(p2),
				p2_hi = in(reg) ::nt_syscall::hi!(p2),
				p3_lo = in(reg) ::nt_syscall::lo!(p3),
				p3_hi = in(reg) ::nt_syscall::hi!(p3),
				in("ecx") aligned,
				options(preserves_flags),
			);
			asm!(
				"mov dword ptr [ecx + 0x20], {p4_lo}",
				"mov dword ptr [ecx + 0x24], {p4_hi}",

				p4_lo = in(reg) ::nt_syscall::lo!(p4),
				p4_hi = in(reg) ::nt_syscall::hi!(p4),
				in("ecx") aligned,
				options(preserves_flags),
			);
			asm!(
				// Prepare used part of the stack
				"mov dword ptr [ecx + 0x28], {p5_lo}",
				"mov dword ptr [ecx + 0x2c], {p5_hi}",
				"mov dword ptr [ecx + 0x30], {p6_lo}",
				"mov dword ptr [ecx + 0x34], {p6_hi}",

				p5_lo = in(reg) ::nt_syscall::lo!(p5),
				p5_hi = in(reg) ::nt_syscall::hi!(p5),
				p6_lo = in(reg) ::nt_syscall::lo!(p6),
				p6_hi = in(reg) ::nt_syscall::hi!(p6),
				in("ecx") aligned,
				options(preserves_flags),
			);
			asm!(
				"mov dword ptr [ecx + 0x38], {p7_lo}",
				"mov dword ptr [ecx + 0x3c], {p7_hi}",
				"mov dword ptr [ecx + 0x40], {p8_lo}",
				"mov dword ptr [ecx + 0x44], {p8_hi}",

				p7_lo = in(reg) ::nt_syscall::lo!(p7),
				p7_hi = in(reg) ::nt_syscall::hi!(p7),
				p8_lo = in(reg) ::nt_syscall::lo!(p8),
				p8_hi = in(reg) ::nt_syscall::hi!(p8),
				in("ecx") aligned,
				options(preserves_flags),
			);
			::nt_syscall::enter_x64!();
			asm!(
				// Align real stack
				"mov ebx, esp",
				"mov esp, ecx",

				// Prepare registers
				".byte 0x48", "mov eax, [ecx + 0x00]", // rax
				".byte 0x4c", "mov edx, [ecx + 0x08]", // r10
				".byte 0x48", "mov edx, [ecx + 0x10]", // rdx
				".byte 0x4c", "mov eax, [ecx + 0x18]", // r8
				".byte 0x4c", "mov ecx, [ecx + 0x20]", // r9

				// Syscall and restore stack
				"syscall",
				"mov esp, ebx",
				
				in("ecx") aligned,
				lateout("ebx") _,
				lateout("ecx") _,
				lateout("edx") _,
				lateout("eax") status,
			);
			::nt_syscall::enter_x86!();
		}

		status
	}};
	($($index:expr)*, $($p1:expr)*, $($p2:expr)*, $($p3:expr)*, $($p4:expr)*, $($p5:expr)*, $($p6:expr)*, $($p7:expr)*, $($p8:expr)*, $($p9:expr)*) => {{
		let index: u64 = ($($index)*) as u64;
		let p1: u64 = ($($p1)*) as u64;
		let p2: u64 = ($($p2)*) as u64;
		let p3: u64 = ($($p3)*) as u64;
		let p4: u64 = ($($p4)*) as u64;
		let p5: u64 = ($($p5)*) as u64;
		let p6: u64 = ($($p6)*) as u64;
		let p7: u64 = ($($p7)*) as u64;
		let p8: u64 = ($($p8)*) as u64;
		let p9: u64 = ($($p9)*) as u64;

		let mut aligned: u32;
		let mut status: u32;
		unsafe {
			asm!(
				// Align copy stack
				"mov ecx, esp",
				"and ecx, 0xfffffff0",
				"sub ecx, 0x60", // Add extra space in case of down alignment of esp

				out("ecx") aligned,
				options(pure, nomem, nostack, preserves_flags),
			);
			asm!(
				// Prepare stack spill space
				"mov dword ptr [ecx + 0x00], {index_lo}",
				"mov dword ptr [ecx + 0x04], {index_hi}",
				"mov dword ptr [ecx + 0x08], {p1_lo}",
				"mov dword ptr [ecx + 0x0c], {p1_hi}",

				index_lo = in(reg) ::nt_syscall::lo!(index),
				index_hi = in(reg) ::nt_syscall::hi!(index),
				p1_lo = in(reg) ::nt_syscall::lo!(p1),
				p1_hi = in(reg) ::nt_syscall::hi!(p1),
				in("ecx") aligned,
				options(preserves_flags),
			);
			asm!(
				"mov dword ptr [ecx + 0x10], {p2_lo}",
				"mov dword ptr [ecx + 0x14], {p2_hi}",
				"mov dword ptr [ecx + 0x18], {p3_lo}",
				"mov dword ptr [ecx + 0x1c], {p3_hi}",

				p2_lo = in(reg) ::nt_syscall::lo!(p2),
				p2_hi = in(reg) ::nt_syscall::hi!(p2),
				p3_lo = in(reg) ::nt_syscall::lo!(p3),
				p3_hi = in(reg) ::nt_syscall::hi!(p3),
				in("ecx") aligned,
				options(preserves_flags),
			);
			asm!(
				"mov dword ptr [ecx + 0x20], {p4_lo}",
				"mov dword ptr [ecx + 0x24], {p4_hi}",

				p4_lo = in(reg) ::nt_syscall::lo!(p4),
				p4_hi = in(reg) ::nt_syscall::hi!(p4),
				in("ecx") aligned,
				options(preserves_flags),
			);
			asm!(
				// Prepare used part of the stack
				"mov dword ptr [ecx + 0x28], {p5_lo}",
				"mov dword ptr [ecx + 0x2c], {p5_hi}",
				"mov dword ptr [ecx + 0x30], {p6_lo}",
				"mov dword ptr [ecx + 0x34], {p6_hi}",

				p5_lo = in(reg) ::nt_syscall::lo!(p5),
				p5_hi = in(reg) ::nt_syscall::hi!(p5),
				p6_lo = in(reg) ::nt_syscall::lo!(p6),
				p6_hi = in(reg) ::nt_syscall::hi!(p6),
				in("ecx") aligned,
				options(preserves_flags),
			);
			asm!(
				"mov dword ptr [ecx + 0x38], {p7_lo}",
				"mov dword ptr [ecx + 0x3c], {p7_hi}",
				"mov dword ptr [ecx + 0x40], {p8_lo}",
				"mov dword ptr [ecx + 0x44], {p8_hi}",

				p7_lo = in(reg) ::nt_syscall::lo!(p7),
				p7_hi = in(reg) ::nt_syscall::hi!(p7),
				p8_lo = in(reg) ::nt_syscall::lo!(p8),
				p8_hi = in(reg) ::nt_syscall::hi!(p8),
				in("ecx") aligned,
				options(preserves_flags),
			);
			asm!(
				"mov dword ptr [ecx + 0x48], {p9_lo}",
				"mov dword ptr [ecx + 0x4c], {p9_hi}",

				p9_lo = in(reg) ::nt_syscall::lo!(p9),
				p9_hi = in(reg) ::nt_syscall::hi!(p9),
				in("ecx") aligned,
				options(preserves_flags),
			);
			::nt_syscall::enter_x64!();
			asm!(
				// Align real stack
				"mov ebx, esp",
				"mov esp, ecx",

				// Prepare registers
				".byte 0x48", "mov eax, [ecx + 0x00]", // rax
				".byte 0x4c", "mov edx, [ecx + 0x08]", // r10
				".byte 0x48", "mov edx, [ecx + 0x10]", // rdx
				".byte 0x4c", "mov eax, [ecx + 0x18]", // r8
				".byte 0x4c", "mov ecx, [ecx + 0x20]", // r9

				// Syscall and restore stack
				"syscall",
				"mov esp, ebx",
				
				in("ecx") aligned,
				lateout("ebx") _,
				lateout("ecx") _,
				lateout("edx") _,
				lateout("eax") status,
			);
			::nt_syscall::enter_x86!();
		}

		status
	}};
	($($index:expr)*, $($p1:expr)*, $($p2:expr)*, $($p3:expr)*, $($p4:expr)*, $($p5:expr)*, $($p6:expr)*, $($p7:expr)*, $($p8:expr)*, $($p9:expr)*, $($p10:expr)*) => {{
		let index: u64 = ($($index)*) as u64;
		let p1: u64 = ($($p1)*) as u64;
		let p2: u64 = ($($p2)*) as u64;
		let p3: u64 = ($($p3)*) as u64;
		let p4: u64 = ($($p4)*) as u64;
		let p5: u64 = ($($p5)*) as u64;
		let p6: u64 = ($($p6)*) as u64;
		let p7: u64 = ($($p7)*) as u64;
		let p8: u64 = ($($p8)*) as u64;
		let p9: u64 = ($($p9)*) as u64;
		let p10: u64 = ($($p10)*) as u64;

		let mut aligned: u32;
		let mut status: u32;
		unsafe {
			asm!(
				// Align copy stack
				"mov ecx, esp",
				"and ecx, 0xfffffff0",
				"sub ecx, 0x60", 

				out("ecx") aligned,
				options(pure, nomem, nostack, preserves_flags),
			);
			asm!(
				// Prepare stack spill space
				"mov dword ptr [ecx + 0x00], {index_lo}",
				"mov dword ptr [ecx + 0x04], {index_hi}",
				"mov dword ptr [ecx + 0x08], {p1_lo}",
				"mov dword ptr [ecx + 0x0c], {p1_hi}",

				index_lo = in(reg) ::nt_syscall::lo!(index),
				index_hi = in(reg) ::nt_syscall::hi!(index),
				p1_lo = in(reg) ::nt_syscall::lo!(p1),
				p1_hi = in(reg) ::nt_syscall::hi!(p1),
				in("ecx") aligned,
				options(preserves_flags),
			);
			asm!(
				"mov dword ptr [ecx + 0x10], {p2_lo}",
				"mov dword ptr [ecx + 0x14], {p2_hi}",
				"mov dword ptr [ecx + 0x18], {p3_lo}",
				"mov dword ptr [ecx + 0x1c], {p3_hi}",

				p2_lo = in(reg) ::nt_syscall::lo!(p2),
				p2_hi = in(reg) ::nt_syscall::hi!(p2),
				p3_lo = in(reg) ::nt_syscall::lo!(p3),
				p3_hi = in(reg) ::nt_syscall::hi!(p3),
				in("ecx") aligned,
				options(preserves_flags),
			);
			asm!(
				"mov dword ptr [ecx + 0x20], {p4_lo}",
				"mov dword ptr [ecx + 0x24], {p4_hi}",

				p4_lo = in(reg) ::nt_syscall::lo!(p4),
				p4_hi = in(reg) ::nt_syscall::hi!(p4),
				in("ecx") aligned,
				options(preserves_flags),
			);
			asm!(
				// Prepare used part of the stack
				"mov dword ptr [ecx + 0x28], {p5_lo}",
				"mov dword ptr [ecx + 0x2c], {p5_hi}",
				"mov dword ptr [ecx + 0x30], {p6_lo}",
				"mov dword ptr [ecx + 0x34], {p6_hi}",

				p5_lo = in(reg) ::nt_syscall::lo!(p5),
				p5_hi = in(reg) ::nt_syscall::hi!(p5),
				p6_lo = in(reg) ::nt_syscall::lo!(p6),
				p6_hi = in(reg) ::nt_syscall::hi!(p6),
				in("ecx") aligned,
				options(preserves_flags),
			);
			asm!(
				"mov dword ptr [ecx + 0x38], {p7_lo}",
				"mov dword ptr [ecx + 0x3c], {p7_hi}",
				"mov dword ptr [ecx + 0x40], {p8_lo}",
				"mov dword ptr [ecx + 0x44], {p8_hi}",

				p7_lo = in(reg) ::nt_syscall::lo!(p7),
				p7_hi = in(reg) ::nt_syscall::hi!(p7),
				p8_lo = in(reg) ::nt_syscall::lo!(p8),
				p8_hi = in(reg) ::nt_syscall::hi!(p8),
				in("ecx") aligned,
				options(preserves_flags),
			);
			asm!(
				"mov dword ptr [ecx + 0x48], {p9_lo}",
				"mov dword ptr [ecx + 0x4c], {p9_hi}",
				"mov dword ptr [ecx + 0x50], {p10_lo}",
				"mov dword ptr [ecx + 0x54], {p10_hi}",

				p9_lo = in(reg) ::nt_syscall::lo!(p9),
				p9_hi = in(reg) ::nt_syscall::hi!(p9),
				p10_lo = in(reg) ::nt_syscall::lo!(p10),
				p10_hi = in(reg) ::nt_syscall::hi!(p10),
				in("ecx") aligned,
				options(preserves_flags),
			);
			::nt_syscall::enter_x64!();
			asm!(
				// Align real stack
				"mov ebx, esp",
				"mov esp, ecx",

				// Prepare registers
				".byte 0x48", "mov eax, [ecx + 0x00]", // rax
				".byte 0x4c", "mov edx, [ecx + 0x08]", // r10
				".byte 0x48", "mov edx, [ecx + 0x10]", // rdx
				".byte 0x4c", "mov eax, [ecx + 0x18]", // r8
				".byte 0x4c", "mov ecx, [ecx + 0x20]", // r9

				// Syscall and restore stack
				"syscall",
				"mov esp, ebx",
				
				in("ecx") aligned,
				lateout("ebx") _,
				lateout("ecx") _,
				lateout("edx") _,
				lateout("eax") status,
			);
			::nt_syscall::enter_x86!();
		}

		status
	}};
	($($index:expr)*, $($p1:expr)*, $($p2:expr)*, $($p3:expr)*, $($p4:expr)*, $($p5:expr)*, $($p6:expr)*, $($p7:expr)*, $($p8:expr)*, $($p9:expr)*, $($p10:expr)*, $($p11:expr)*) => {{
		let index: u64 = ($($index)*) as u64;
		let p1: u64 = ($($p1)*) as u64;
		let p2: u64 = ($($p2)*) as u64;
		let p3: u64 = ($($p3)*) as u64;
		let p4: u64 = ($($p4)*) as u64;
		let p5: u64 = ($($p5)*) as u64;
		let p6: u64 = ($($p6)*) as u64;
		let p7: u64 = ($($p7)*) as u64;
		let p8: u64 = ($($p8)*) as u64;
		let p9: u64 = ($($p9)*) as u64;
		let p10: u64 = ($($p10)*) as u64;
		let p11: u64 = ($($p11)*) as u64;

		let mut aligned: u32;
		let mut status: u32;
		unsafe {
			asm!(
				// Align copy stack
				"mov ecx, esp",
				"and ecx, 0xfffffff0",
				"sub ecx, 0x70", // Add extra space in case of down alignment of esp

				out("ecx") aligned,
				options(pure, nomem, nostack, preserves_flags),
			);
			asm!(
				// Prepare stack spill space
				"mov dword ptr [ecx + 0x00], {index_lo}",
				"mov dword ptr [ecx + 0x04], {index_hi}",
				"mov dword ptr [ecx + 0x08], {p1_lo}",
				"mov dword ptr [ecx + 0x0c], {p1_hi}",

				index_lo = in(reg) ::nt_syscall::lo!(index),
				index_hi = in(reg) ::nt_syscall::hi!(index),
				p1_lo = in(reg) ::nt_syscall::lo!(p1),
				p1_hi = in(reg) ::nt_syscall::hi!(p1),
				in("ecx") aligned,
				options(preserves_flags),
			);
			asm!(
				"mov dword ptr [ecx + 0x10], {p2_lo}",
				"mov dword ptr [ecx + 0x14], {p2_hi}",
				"mov dword ptr [ecx + 0x18], {p3_lo}",
				"mov dword ptr [ecx + 0x1c], {p3_hi}",

				p2_lo = in(reg) ::nt_syscall::lo!(p2),
				p2_hi = in(reg) ::nt_syscall::hi!(p2),
				p3_lo = in(reg) ::nt_syscall::lo!(p3),
				p3_hi = in(reg) ::nt_syscall::hi!(p3),
				in("ecx") aligned,
				options(preserves_flags),
			);
			asm!(
				"mov dword ptr [ecx + 0x20], {p4_lo}",
				"mov dword ptr [ecx + 0x24], {p4_hi}",

				p4_lo = in(reg) ::nt_syscall::lo!(p4),
				p4_hi = in(reg) ::nt_syscall::hi!(p4),
				in("ecx") aligned,
				options(preserves_flags),
			);
			asm!(
				// Prepare used part of the stack
				"mov dword ptr [ecx + 0x28], {p5_lo}",
				"mov dword ptr [ecx + 0x2c], {p5_hi}",
				"mov dword ptr [ecx + 0x30], {p6_lo}",
				"mov dword ptr [ecx + 0x34], {p6_hi}",

				p5_lo = in(reg) ::nt_syscall::lo!(p5),
				p5_hi = in(reg) ::nt_syscall::hi!(p5),
				p6_lo = in(reg) ::nt_syscall::lo!(p6),
				p6_hi = in(reg) ::nt_syscall::hi!(p6),
				in("ecx") aligned,
				options(preserves_flags),
			);
			asm!(
				"mov dword ptr [ecx + 0x38], {p7_lo}",
				"mov dword ptr [ecx + 0x3c], {p7_hi}",
				"mov dword ptr [ecx + 0x40], {p8_lo}",
				"mov dword ptr [ecx + 0x44], {p8_hi}",

				p7_lo = in(reg) ::nt_syscall::lo!(p7),
				p7_hi = in(reg) ::nt_syscall::hi!(p7),
				p8_lo = in(reg) ::nt_syscall::lo!(p8),
				p8_hi = in(reg) ::nt_syscall::hi!(p8),
				in("ecx") aligned,
				options(preserves_flags),
			);
			asm!(
				"mov dword ptr [ecx + 0x48], {p9_lo}",
				"mov dword ptr [ecx + 0x4c], {p9_hi}",
				"mov dword ptr [ecx + 0x50], {p10_lo}",
				"mov dword ptr [ecx + 0x54], {p10_hi}",

				p9_lo = in(reg) ::nt_syscall::lo!(p9),
				p9_hi = in(reg) ::nt_syscall::hi!(p9),
				p10_lo = in(reg) ::nt_syscall::lo!(p10),
				p10_hi = in(reg) ::nt_syscall::hi!(p10),
				in("ecx") aligned,
				options(preserves_flags),
			);
			asm!(
				"mov dword ptr [ecx + 0x58], {p11_lo}",
				"mov dword ptr [ecx + 0x5c], {p11_hi}",

				p11_lo = in(reg) ::nt_syscall::lo!(p11),
				p11_hi = in(reg) ::nt_syscall::hi!(p11),
				in("ecx") aligned,
				options(preserves_flags),
			);
			::nt_syscall::enter_x64!();
			asm!(
				// Align real stack
				"mov ebx, esp",
				"mov esp, ecx",

				// Prepare registers
				".byte 0x48", "mov eax, [ecx + 0x00]", // rax
				".byte 0x4c", "mov edx, [ecx + 0x08]", // r10
				".byte 0x48", "mov edx, [ecx + 0x10]", // rdx
				".byte 0x4c", "mov eax, [ecx + 0x18]", // r8
				".byte 0x4c", "mov ecx, [ecx + 0x20]", // r9

				// Syscall and restore stack
				"syscall",
				"mov esp, ebx",
				
				in("ecx") aligned,
				lateout("ebx") _,
				lateout("ecx") _,
				lateout("edx") _,
				lateout("eax") status,
			);
			::nt_syscall::enter_x86!();
		}

		status
	}};
	($($index:expr)*, $($p1:expr)*, $($p2:expr)*, $($p3:expr)*, $($p4:expr)*, $($p5:expr)*, $($p6:expr)*, $($p7:expr)*, $($p8:expr)*, $($p9:expr)*, $($p10:expr)*, $($p11:expr)*, $($p12:expr)*) => {{
		let index: u64 = ($($index)*) as u64;
		let p1: u64 = ($($p1)*) as u64;
		let p2: u64 = ($($p2)*) as u64;
		let p3: u64 = ($($p3)*) as u64;
		let p4: u64 = ($($p4)*) as u64;
		let p5: u64 = ($($p5)*) as u64;
		let p6: u64 = ($($p6)*) as u64;
		let p7: u64 = ($($p7)*) as u64;
		let p8: u64 = ($($p8)*) as u64;
		let p9: u64 = ($($p9)*) as u64;
		let p10: u64 = ($($p10)*) as u64;
		let p11: u64 = ($($p11)*) as u64;
		let p12: u64 = ($($p12)*) as u64;

		let mut aligned: u32;
		let mut status: u32;
		unsafe {
			asm!(
				// Align copy stack
				"mov ecx, esp",
				"and ecx, 0xfffffff0",
				"sub ecx, 0x70",

				out("ecx") aligned,
				options(pure, nomem, nostack, preserves_flags),
			);
			asm!(
				// Prepare stack spill space
				"mov dword ptr [ecx + 0x00], {index_lo}",
				"mov dword ptr [ecx + 0x04], {index_hi}",
				"mov dword ptr [ecx + 0x08], {p1_lo}",
				"mov dword ptr [ecx + 0x0c], {p1_hi}",

				index_lo = in(reg) ::nt_syscall::lo!(index),
				index_hi = in(reg) ::nt_syscall::hi!(index),
				p1_lo = in(reg) ::nt_syscall::lo!(p1),
				p1_hi = in(reg) ::nt_syscall::hi!(p1),
				in("ecx") aligned,
				options(preserves_flags),
			);
			asm!(
				"mov dword ptr [ecx + 0x10], {p2_lo}",
				"mov dword ptr [ecx + 0x14], {p2_hi}",
				"mov dword ptr [ecx + 0x18], {p3_lo}",
				"mov dword ptr [ecx + 0x1c], {p3_hi}",

				p2_lo = in(reg) ::nt_syscall::lo!(p2),
				p2_hi = in(reg) ::nt_syscall::hi!(p2),
				p3_lo = in(reg) ::nt_syscall::lo!(p3),
				p3_hi = in(reg) ::nt_syscall::hi!(p3),
				in("ecx") aligned,
				options(preserves_flags),
			);
			asm!(
				"mov dword ptr [ecx + 0x20], {p4_lo}",
				"mov dword ptr [ecx + 0x24], {p4_hi}",

				p4_lo = in(reg) ::nt_syscall::lo!(p4),
				p4_hi = in(reg) ::nt_syscall::hi!(p4),
				in("ecx") aligned,
				options(preserves_flags),
			);
			asm!(
				// Prepare used part of the stack
				"mov dword ptr [ecx + 0x28], {p5_lo}",
				"mov dword ptr [ecx + 0x2c], {p5_hi}",
				"mov dword ptr [ecx + 0x30], {p6_lo}",
				"mov dword ptr [ecx + 0x34], {p6_hi}",

				p5_lo = in(reg) ::nt_syscall::lo!(p5),
				p5_hi = in(reg) ::nt_syscall::hi!(p5),
				p6_lo = in(reg) ::nt_syscall::lo!(p6),
				p6_hi = in(reg) ::nt_syscall::hi!(p6),
				in("ecx") aligned,
				options(preserves_flags),
			);
			asm!(
				"mov dword ptr [ecx + 0x38], {p7_lo}",
				"mov dword ptr [ecx + 0x3c], {p7_hi}",
				"mov dword ptr [ecx + 0x40], {p8_lo}",
				"mov dword ptr [ecx + 0x44], {p8_hi}",

				p7_lo = in(reg) ::nt_syscall::lo!(p7),
				p7_hi = in(reg) ::nt_syscall::hi!(p7),
				p8_lo = in(reg) ::nt_syscall::lo!(p8),
				p8_hi = in(reg) ::nt_syscall::hi!(p8),
				in("ecx") aligned,
				options(preserves_flags),
			);
			asm!(
				"mov dword ptr [ecx + 0x48], {p9_lo}",
				"mov dword ptr [ecx + 0x4c], {p9_hi}",
				"mov dword ptr [ecx + 0x50], {p10_lo}",
				"mov dword ptr [ecx + 0x54], {p10_hi}",

				p9_lo = in(reg) ::nt_syscall::lo!(p9),
				p9_hi = in(reg) ::nt_syscall::hi!(p9),
				p10_lo = in(reg) ::nt_syscall::lo!(p10),
				p10_hi = in(reg) ::nt_syscall::hi!(p10),
				in("ecx") aligned,
				options(preserves_flags),
			);
			asm!(
				"mov dword ptr [ecx + 0x58], {p11_lo}",
				"mov dword ptr [ecx + 0x5c], {p11_hi}",
				"mov dword ptr [ecx + 0x60], {p12_lo}",
				"mov dword ptr [ecx + 0x64], {p12_hi}",

				p11_lo = in(reg) ::nt_syscall::lo!(p11),
				p11_hi = in(reg) ::nt_syscall::hi!(p11),
				p12_lo = in(reg) ::nt_syscall::lo!(p12),
				p12_hi = in(reg) ::nt_syscall::hi!(p12),
				in("ecx") aligned,
				options(preserves_flags),
			);
			::nt_syscall::enter_x64!();
			asm!(
				// Align real stack
				"mov ebx, esp",
				"mov esp, ecx",

				// Prepare registers
				".byte 0x48", "mov eax, [ecx + 0x00]", // rax
				".byte 0x4c", "mov edx, [ecx + 0x08]", // r10
				".byte 0x48", "mov edx, [ecx + 0x10]", // rdx
				".byte 0x4c", "mov eax, [ecx + 0x18]", // r8
				".byte 0x4c", "mov ecx, [ecx + 0x20]", // r9

				// Syscall and restore stack
				"syscall",
				"mov esp, ebx",
				
				in("ecx") aligned,
				lateout("ebx") _,
				lateout("ecx") _,
				lateout("edx") _,
				lateout("eax") status,
			);
			::nt_syscall::enter_x86!();
		}

		status
	}};
	($($index:expr)*, $($p1:expr)*, $($p2:expr)*, $($p3:expr)*, $($p4:expr)*, $($p5:expr)*, $($p6:expr)*, $($p7:expr)*, $($p8:expr)*, $($p9:expr)*, $($p10:expr)*, $($p11:expr)*, $($p12:expr)*, $($p13:expr)*) => {{
		let index: u64 = ($($index)*) as u64;
		let p1: u64 = ($($p1)*) as u64;
		let p2: u64 = ($($p2)*) as u64;
		let p3: u64 = ($($p3)*) as u64;
		let p4: u64 = ($($p4)*) as u64;
		let p5: u64 = ($($p5)*) as u64;
		let p6: u64 = ($($p6)*) as u64;
		let p7: u64 = ($($p7)*) as u64;
		let p8: u64 = ($($p8)*) as u64;
		let p9: u64 = ($($p9)*) as u64;
		let p10: u64 = ($($p10)*) as u64;
		let p11: u64 = ($($p11)*) as u64;
		let p12: u64 = ($($p12)*) as u64;
		let p13: u64 = ($($p13)*) as u64;

		let mut aligned: u32;
		let mut status: u32;
		unsafe {
			asm!(
				// Align copy stack
				"mov ecx, esp",
				"and ecx, 0xfffffff0",
				"sub ecx, 0x80", // Add extra space in case of down alignment of esp

				out("ecx") aligned,
				options(pure, nomem, nostack, preserves_flags),
			);
			asm!(
				// Prepare stack spill space
				"mov dword ptr [ecx + 0x00], {index_lo}",
				"mov dword ptr [ecx + 0x04], {index_hi}",
				"mov dword ptr [ecx + 0x08], {p1_lo}",
				"mov dword ptr [ecx + 0x0c], {p1_hi}",

				index_lo = in(reg) ::nt_syscall::lo!(index),
				index_hi = in(reg) ::nt_syscall::hi!(index),
				p1_lo = in(reg) ::nt_syscall::lo!(p1),
				p1_hi = in(reg) ::nt_syscall::hi!(p1),
				in("ecx") aligned,
				options(preserves_flags),
			);
			asm!(
				"mov dword ptr [ecx + 0x10], {p2_lo}",
				"mov dword ptr [ecx + 0x14], {p2_hi}",
				"mov dword ptr [ecx + 0x18], {p3_lo}",
				"mov dword ptr [ecx + 0x1c], {p3_hi}",

				p2_lo = in(reg) ::nt_syscall::lo!(p2),
				p2_hi = in(reg) ::nt_syscall::hi!(p2),
				p3_lo = in(reg) ::nt_syscall::lo!(p3),
				p3_hi = in(reg) ::nt_syscall::hi!(p3),
				in("ecx") aligned,
				options(preserves_flags),
			);
			asm!(
				"mov dword ptr [ecx + 0x20], {p4_lo}",
				"mov dword ptr [ecx + 0x24], {p4_hi}",

				p4_lo = in(reg) ::nt_syscall::lo!(p4),
				p4_hi = in(reg) ::nt_syscall::hi!(p4),
				in("ecx") aligned,
				options(preserves_flags),
			);
			asm!(
				// Prepare used part of the stack
				"mov dword ptr [ecx + 0x28], {p5_lo}",
				"mov dword ptr [ecx + 0x2c], {p5_hi}",
				"mov dword ptr [ecx + 0x30], {p6_lo}",
				"mov dword ptr [ecx + 0x34], {p6_hi}",

				p5_lo = in(reg) ::nt_syscall::lo!(p5),
				p5_hi = in(reg) ::nt_syscall::hi!(p5),
				p6_lo = in(reg) ::nt_syscall::lo!(p6),
				p6_hi = in(reg) ::nt_syscall::hi!(p6),
				in("ecx") aligned,
				options(preserves_flags),
			);
			asm!(
				"mov dword ptr [ecx + 0x38], {p7_lo}",
				"mov dword ptr [ecx + 0x3c], {p7_hi}",
				"mov dword ptr [ecx + 0x40], {p8_lo}",
				"mov dword ptr [ecx + 0x44], {p8_hi}",

				p7_lo = in(reg) ::nt_syscall::lo!(p7),
				p7_hi = in(reg) ::nt_syscall::hi!(p7),
				p8_lo = in(reg) ::nt_syscall::lo!(p8),
				p8_hi = in(reg) ::nt_syscall::hi!(p8),
				in("ecx") aligned,
				options(preserves_flags),
			);
			asm!(
				"mov dword ptr [ecx + 0x48], {p9_lo}",
				"mov dword ptr [ecx + 0x4c], {p9_hi}",
				"mov dword ptr [ecx + 0x50], {p10_lo}",
				"mov dword ptr [ecx + 0x54], {p10_hi}",

				p9_lo = in(reg) ::nt_syscall::lo!(p9),
				p9_hi = in(reg) ::nt_syscall::hi!(p9),
				p10_lo = in(reg) ::nt_syscall::lo!(p10),
				p10_hi = in(reg) ::nt_syscall::hi!(p10),
				in("ecx") aligned,
				options(preserves_flags),
			);
			asm!(
				"mov dword ptr [ecx + 0x58], {p11_lo}",
				"mov dword ptr [ecx + 0x5c], {p11_hi}",
				"mov dword ptr [ecx + 0x60], {p12_lo}",
				"mov dword ptr [ecx + 0x64], {p12_hi}",

				p11_lo = in(reg) ::nt_syscall::lo!(p11),
				p11_hi = in(reg) ::nt_syscall::hi!(p11),
				p12_lo = in(reg) ::nt_syscall::lo!(p12),
				p12_hi = in(reg) ::nt_syscall::hi!(p12),
				in("ecx") aligned,
				options(preserves_flags),
			);
			asm!(
				"mov dword ptr [ecx + 0x68], {p13_lo}",
				"mov dword ptr [ecx + 0x6c], {p13_hi}",

				p13_lo = in(reg) ::nt_syscall::lo!(p13),
				p13_hi = in(reg) ::nt_syscall::hi!(p13),
				in("ecx") aligned,
				options(preserves_flags),
			);
			::nt_syscall::enter_x64!();
			asm!(
				// Align real stack
				"mov ebx, esp",
				"mov esp, ecx",

				// Prepare registers
				".byte 0x48", "mov eax, [ecx + 0x00]", // rax
				".byte 0x4c", "mov edx, [ecx + 0x08]", // r10
				".byte 0x48", "mov edx, [ecx + 0x10]", // rdx
				".byte 0x4c", "mov eax, [ecx + 0x18]", // r8
				".byte 0x4c", "mov ecx, [ecx + 0x20]", // r9

				// Syscall and restore stack
				"syscall",
				"mov esp, ebx",
				
				in("ecx") aligned,
				lateout("ebx") _,
				lateout("ecx") _,
				lateout("edx") _,
				lateout("eax") status,
			);
			::nt_syscall::enter_x86!();
		}

		status
	}};
}




#[macro_export]
macro_rules! syscall_x86 {
	($($index:expr)*) => {{
		let mut status: u32 = 0;
		asm!(
			"sub esp, 0x4",

			"mov edx, esp",
			"sub edx, 0x4",
			".byte 0xe8", ".byte 0x00", ".byte 0x00", ".byte 0x00", ".byte 0x00",
			"add dword ptr [esp], 0x6",
			"sysenter",
			"add esp, 0x04",

			in("eax") ($($index)*) as u32,
			lateout("ecx") _,
			lateout("edx") _,
			lateout("eax") status,
		);
		status
	}};
	($($index:expr)*, $($p1:expr)*) => {{
		let mut status: u32 = 0;
		asm!(
			"push {p1:e}",
			"sub esp, 0x4",

			"mov edx, esp",
			"sub edx, 0x4",
			".byte 0xe8", ".byte 0x00", ".byte 0x00", ".byte 0x00", ".byte 0x00",
			"add dword ptr [esp], 0x6",
			"sysenter",
			"add esp, 0x08",

			p1 = in(reg) ($($p1)*) as u32,

			in("eax") ($($index)*) as u32,
			lateout("ecx") _,
			lateout("edx") _,
			lateout("eax") status,
		);
		status
	}};
	($($index:expr)*, $($p1:expr)*, $($p2:expr)*) => {{
		let mut status: u32 = 0;
		asm!(
			"push {p2:e}",
			"push {p1:e}",
			"sub esp, 0x4",

			"mov edx, esp",
			"sub edx, 0x4",
			".byte 0xe8", ".byte 0x00", ".byte 0x00", ".byte 0x00", ".byte 0x00",
			"add dword ptr [esp], 0x6",
			"sysenter",
			"add esp, 0x0c",

			p2 = in(reg) ($($p2)*) as u32,
			p1 = in(reg) ($($p1)*) as u32,

			in("eax") ($($index)*) as u32,
			lateout("ecx") _,
			lateout("edx") _,
			lateout("eax") status,
		);
		status
	}};
	($($index:expr)*, $($p1:expr)*, $($p2:expr)*, $($p3:expr)*) => {{
		let mut status: u32 = 0;
		asm!(
			"push {p3:e}",
			"push {p2:e}",
			"push {p1:e}",
			"sub esp, 0x4",

			"mov edx, esp",
			"sub edx, 0x4",
			".byte 0xe8", ".byte 0x00", ".byte 0x00", ".byte 0x00", ".byte 0x00",
			"add dword ptr [esp], 0x6",
			"sysenter",
			"add esp, 0x10",

			p3 = in(reg) ($($p3)*) as u32,
			p2 = in(reg) ($($p2)*) as u32,
			p1 = in(reg) ($($p1)*) as u32,

			in("eax") ($($index)*) as u32,
			lateout("ecx") _,
			lateout("edx") _,
			lateout("eax") status,
		);
		status
	}};
	($($index:expr)*, $($p1:expr)*, $($p2:expr)*, $($p3:expr)*, $($p4:expr)*) => {{
		let mut status: u32 = 0;
		asm!(
			"push {p4:e}",
			"push {p3:e}",
			"push {p2:e}",
			"push {p1:e}",
			"sub esp, 0x4",

			"mov edx, esp",
			"sub edx, 0x4",
			".byte 0xe8", ".byte 0x00", ".byte 0x00", ".byte 0x00", ".byte 0x00",
			"add dword ptr [esp], 0x6",
			"sysenter",
			"add esp, 0x14",

			p4 = in(reg) ($($p4)*) as u32,
			p3 = in(reg) ($($p3)*) as u32,
			p2 = in(reg) ($($p2)*) as u32,
			p1 = in(reg) ($($p1)*) as u32,

			in("eax") ($($index)*) as u32,
			lateout("ecx") _,
			lateout("edx") _,
			lateout("eax") status,
		);
		status
	}};
	($($index:expr)*, $($p1:expr)*, $($p2:expr)*, $($p3:expr)*, $($p4:expr)*, $($p5:expr)*) => {{
		let mut status: u32 = 0;
		asm!(
			"push {p5:e}",

			p5 = in(reg) ($($p5)*) as u32,
		);
		asm!(
			"push {p4:e}",
			"push {p3:e}",
			"push {p2:e}",
			"push {p1:e}",
			"sub esp, 0x4",

			"mov edx, esp",
			"sub edx, 0x4",
			".byte 0xe8", ".byte 0x00", ".byte 0x00", ".byte 0x00", ".byte 0x00",
			"add dword ptr [esp], 0x6",
			"sysenter",
			"add esp, 0x18",

			p4 = in(reg) ($($p4)*) as u32,
			p3 = in(reg) ($($p3)*) as u32,
			p2 = in(reg) ($($p2)*) as u32,
			p1 = in(reg) ($($p1)*) as u32,

			in("eax") ($($index)*) as u32,
			lateout("ecx") _,
			lateout("edx") _,
			lateout("eax") status,
		);
		status
	}};
	($($index:expr)*, $($p1:expr)*, $($p2:expr)*, $($p3:expr)*, $($p4:expr)*, $($p5:expr)*, $($p6:expr)*) => {{
		let mut status: u32 = 0;
		asm!(
			"push {p6:e}",
			"push {p5:e}",
			
			p6 = in(reg) ($($p6)*) as u32,
			p5 = in(reg) ($($p5)*) as u32,
		);
		asm!(
			"push {p4:e}",
			"push {p3:e}",
			"push {p2:e}",
			"push {p1:e}",
			"sub esp, 0x4",

			"mov edx, esp",
			"sub edx, 0x4",
			".byte 0xe8", ".byte 0x00", ".byte 0x00", ".byte 0x00", ".byte 0x00",
			"add dword ptr [esp], 0x6",
			"sysenter",
			"add esp, 0x1c",

			p4 = in(reg) ($($p4)*) as u32,
			p3 = in(reg) ($($p3)*) as u32,
			p2 = in(reg) ($($p2)*) as u32,
			p1 = in(reg) ($($p1)*) as u32,

			in("eax") ($($index)*) as u32,
			lateout("ecx") _,
			lateout("edx") _,
			lateout("eax") status,
		);
		status
	}};
	($($index:expr)*, $($p1:expr)*, $($p2:expr)*, $($p3:expr)*, $($p4:expr)*, $($p5:expr)*, $($p6:expr)*, $($p7:expr)*) => {{
		let mut status: u32 = 0;
		asm!(
			"push {p7:e}",
			"push {p6:e}",
			"push {p5:e}",

			p7 = in(reg) ($($p7)*) as u32,
			p6 = in(reg) ($($p6)*) as u32,
			p5 = in(reg) ($($p5)*) as u32,
		);
		asm!(
			"push {p4:e}",
			"push {p3:e}",
			"push {p2:e}",
			"push {p1:e}",
			"sub esp, 0x4",

			"mov edx, esp",
			"sub edx, 0x4",
			".byte 0xe8", ".byte 0x00", ".byte 0x00", ".byte 0x00", ".byte 0x00",
			"add dword ptr [esp], 0x6",
			"sysenter",
			"add esp, 0x20",

			p4 = in(reg) ($($p4)*) as u32,
			p3 = in(reg) ($($p3)*) as u32,
			p2 = in(reg) ($($p2)*) as u32,
			p1 = in(reg) ($($p1)*) as u32,

			in("eax") ($($index)*) as u32,
			lateout("ecx") _,
			lateout("edx") _,
			lateout("eax") status,
		);
		status
	}};
	($($index:expr)*, $($p1:expr)*, $($p2:expr)*, $($p3:expr)*, $($p4:expr)*, $($p5:expr)*, $($p6:expr)*, $($p7:expr)*, $($p8:expr)*) => {{
		let mut status: u32 = 0;
		asm!(
			"push {p8:e}",
			"push {p7:e}",
			"push {p6:e}",
			"push {p5:e}",

			p8 = in(reg) ($($p8)*) as u32,
			p7 = in(reg) ($($p7)*) as u32,
			p6 = in(reg) ($($p6)*) as u32,
			p5 = in(reg) ($($p5)*) as u32,
		);
		asm!(
			"push {p4:e}",
			"push {p3:e}",
			"push {p2:e}",
			"push {p1:e}",
			"sub esp, 0x4",

			"mov edx, esp",
			"sub edx, 0x4",
			".byte 0xe8", ".byte 0x00", ".byte 0x00", ".byte 0x00", ".byte 0x00",
			"add dword ptr [esp], 0x6",
			"sysenter",
			"add esp, 0x24",

			p4 = in(reg) ($($p4)*) as u32,
			p3 = in(reg) ($($p3)*) as u32,
			p2 = in(reg) ($($p2)*) as u32,
			p1 = in(reg) ($($p1)*) as u32,

			in("eax") ($($index)*) as u32,
			lateout("ecx") _,
			lateout("edx") _,
			lateout("eax") status,
		);
		status
	}};
	($($index:expr)*, $($p1:expr)*, $($p2:expr)*, $($p3:expr)*, $($p4:expr)*, $($p5:expr)*, $($p6:expr)*, $($p7:expr)*, $($p8:expr)*, $($p9:expr)*) => {{
		let mut status: u32 = 0;
		asm!(
			"push {p9:e}",
			"push {p8:e}",
			"push {p7:e}",
			"push {p6:e}",
			"push {p5:e}",

			p9 = in(reg) ($($p9)*) as u32,
			p8 = in(reg) ($($p8)*) as u32,
			p7 = in(reg) ($($p7)*) as u32,
			p6 = in(reg) ($($p6)*) as u32,
			p5 = in(reg) ($($p5)*) as u32,
		);
		asm!(
			"push {p4:e}",
			"push {p3:e}",
			"push {p2:e}",
			"push {p1:e}",
			"sub esp, 0x4",

			"mov edx, esp",
			"sub edx, 0x4",
			".byte 0xe8", ".byte 0x00", ".byte 0x00", ".byte 0x00", ".byte 0x00",
			"add dword ptr [esp], 0x6",
			"sysenter",
			"add esp, 0x28",

			p4 = in(reg) ($($p4)*) as u32,
			p3 = in(reg) ($($p3)*) as u32,
			p2 = in(reg) ($($p2)*) as u32,
			p1 = in(reg) ($($p1)*) as u32,

			in("eax") ($($index)*) as u32,
			lateout("ecx") _,
			lateout("edx") _,
			lateout("eax") status,
		);
		status
	}};
	($($index:expr)*, $($p1:expr)*, $($p2:expr)*, $($p3:expr)*, $($p4:expr)*, $($p5:expr)*, $($p6:expr)*, $($p7:expr)*, $($p8:expr)*, $($p9:expr)*, $($p10:expr)*) => {{
		let mut status: u32 = 0;
		asm!(
			"push {p10:e}",
			"push {p9:e}",
			"push {p8:e}",
			"push {p7:e}",
			"push {p6:e}",
			"push {p5:e}",

			p10 = in(reg) ($($p10)*) as u32,
			p9 = in(reg) ($($p9)*) as u32,
			p8 = in(reg) ($($p8)*) as u32,
			p7 = in(reg) ($($p7)*) as u32,
			p6 = in(reg) ($($p6)*) as u32,
			p5 = in(reg) ($($p5)*) as u32,
		);
		asm!(
			"push {p4:e}",
			"push {p3:e}",
			"push {p2:e}",
			"push {p1:e}",
			"sub esp, 0x4",

			"mov edx, esp",
			"sub edx, 0x4",
			".byte 0xe8", ".byte 0x00", ".byte 0x00", ".byte 0x00", ".byte 0x00",
			"add dword ptr [esp], 0x6",
			"sysenter",
			"add esp, 0x2c",

			p4 = in(reg) ($($p4)*) as u32,
			p3 = in(reg) ($($p3)*) as u32,
			p2 = in(reg) ($($p2)*) as u32,
			p1 = in(reg) ($($p1)*) as u32,

			in("eax") ($($index)*) as u32,
			lateout("ecx") _,
			lateout("edx") _,
			lateout("eax") status,
		);
		status
	}};
	($($index:expr)*, $($p1:expr)*, $($p2:expr)*, $($p3:expr)*, $($p4:expr)*, $($p5:expr)*, $($p6:expr)*, $($p7:expr)*, $($p8:expr)*, $($p9:expr)*, $($p10:expr)*, $($p11:expr)*) => {{
		let mut status: u32 = 0;
		asm!(
			"push {p11:e}",

			p11 = in(reg) ($($p11)*) as u32,
		);
		asm!(
			"push {p10:e}",
			"push {p9:e}",
			"push {p8:e}",
			"push {p7:e}",
			"push {p6:e}",
			"push {p5:e}",

			p10 = in(reg) ($($p10)*) as u32,
			p9 = in(reg) ($($p9)*) as u32,
			p8 = in(reg) ($($p8)*) as u32,
			p7 = in(reg) ($($p7)*) as u32,
			p6 = in(reg) ($($p6)*) as u32,
			p5 = in(reg) ($($p5)*) as u32,
		);
		asm!(
			"push {p4:e}",
			"push {p3:e}",
			"push {p2:e}",
			"push {p1:e}",
			"sub esp, 0x4",

			"mov edx, esp",
			"sub edx, 0x4",
			".byte 0xe8", ".byte 0x00", ".byte 0x00", ".byte 0x00", ".byte 0x00",
			"add dword ptr [esp], 0x6",
			"sysenter",
			"add esp, 0x30",

			p4 = in(reg) ($($p4)*) as u32,
			p3 = in(reg) ($($p3)*) as u32,
			p2 = in(reg) ($($p2)*) as u32,
			p1 = in(reg) ($($p1)*) as u32,

			in("eax") ($($index)*) as u32,
			lateout("ecx") _,
			lateout("edx") _,
			lateout("eax") status,
		);
		status
	}};
	($($index:expr)*, $($p1:expr)*, $($p2:expr)*, $($p3:expr)*, $($p4:expr)*, $($p5:expr)*, $($p6:expr)*, $($p7:expr)*, $($p8:expr)*, $($p9:expr)*, $($p10:expr)*, $($p11:expr)*, $($p12:expr)*) => {{
		let mut status: u32 = 0;
		asm!(
			"push {p12:e}",
			"push {p11:e}",

			p12 = in(reg) ($($p12)*) as u32,
			p11 = in(reg) ($($p11)*) as u32,
		);
		asm!(
			"push {p10:e}",
			"push {p9:e}",
			"push {p8:e}",
			"push {p7:e}",
			"push {p6:e}",
			"push {p5:e}",

			p10 = in(reg) ($($p10)*) as u32,
			p9 = in(reg) ($($p9)*) as u32,
			p8 = in(reg) ($($p8)*) as u32,
			p7 = in(reg) ($($p7)*) as u32,
			p6 = in(reg) ($($p6)*) as u32,
			p5 = in(reg) ($($p5)*) as u32,
		);
		asm!(
			"push {p4:e}",
			"push {p3:e}",
			"push {p2:e}",
			"push {p1:e}",
			"sub esp, 0x4",

			"mov edx, esp",
			"sub edx, 0x4",
			".byte 0xe8", ".byte 0x00", ".byte 0x00", ".byte 0x00", ".byte 0x00",
			"add dword ptr [esp], 0x6",
			"sysenter",
			"add esp, 0x34",

			p4 = in(reg) ($($p4)*) as u32,
			p3 = in(reg) ($($p3)*) as u32,
			p2 = in(reg) ($($p2)*) as u32,
			p1 = in(reg) ($($p1)*) as u32,

			in("eax") ($($index)*) as u32,
			lateout("ecx") _,
			lateout("edx") _,
			lateout("eax") status,
		);
		status
	}};
	($($index:expr)*, $($p1:expr)*, $($p2:expr)*, $($p3:expr)*, $($p4:expr)*, $($p5:expr)*, $($p6:expr)*, $($p7:expr)*, $($p8:expr)*, $($p9:expr)*, $($p10:expr)*, $($p11:expr)*, $($p12:expr)*, $($p13:expr)*) => {{
		let mut status: u32 = 0;
		asm!(
			"push {p13:e}",
			"push {p12:e}",
			"push {p11:e}",

			p13 = in(reg) ($($p13)*) as u32,
			p12 = in(reg) ($($p12)*) as u32,
			p11 = in(reg) ($($p11)*) as u32,
		)
		asm!(
			"push {p10:e}",
			"push {p9:e}",
			"push {p8:e}",
			"push {p7:e}",
			"push {p6:e}",
			"push {p5:e}",

			p10 = in(reg) ($($p10)*) as u32,
			p9 = in(reg) ($($p9)*) as u32,
			p8 = in(reg) ($($p8)*) as u32,
			p7 = in(reg) ($($p7)*) as u32,
			p6 = in(reg) ($($p6)*) as u32,
			p5 = in(reg) ($($p5)*) as u32,
		);
		asm!(
			"push {p4:e}",
			"push {p3:e}",
			"push {p2:e}",
			"push {p1:e}",
			"sub esp, 0x4",

			"mov edx, esp",
			"sub edx, 0x4",
			".byte 0xe8", ".byte 0x00", ".byte 0x00", ".byte 0x00", ".byte 0x00",
			"add dword ptr [esp], 0x6",
			"sysenter",
			"add esp, 0x38",

			p4 = in(reg) ($($p4)*) as u32,
			p3 = in(reg) ($($p3)*) as u32,
			p2 = in(reg) ($($p2)*) as u32,
			p1 = in(reg) ($($p1)*) as u32,

			in("eax") ($($index)*) as u32,
			lateout("ecx") _,
			lateout("edx") _,
			lateout("eax") status,
		);
		status
	}};
}