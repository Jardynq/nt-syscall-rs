#[macro_export] 
#[cfg(target_arch="x86_64")]
macro_rules! teb {
	() => {{
		let mut teb: *mut ::ntapi::ntpebteb::TEB;
		core::arch::asm!(
			"mov rax, gs:[0x30]",
			out("rax") teb
		);
		::core::mem::transmute::<_, &mut ::ntapi::ntpebteb::TEB>(teb)
	}}
}
#[macro_export] 
#[cfg(target_arch="x86")]
macro_rules! teb {
	() => {{
		let mut teb: *mut ::ntapi::ntpebteb::TEB;
		asm!(
			"mov eax, fs:[0x18]",
			out("eax") teb
		);
		::core::mem::transmute::<_, &mut ::ntapi::ntpebteb::TEB>(teb)
	}}
}

#[macro_export] 
#[cfg(target_arch="x86_64")]
macro_rules! teb_x64 {
	() => {
		Some(::nt_syscall::teb!())
	}
}
#[macro_export] 
#[cfg(target_arch="x86")]
macro_rules! teb_x64 {
	() => {{
		let mut teb: *mut ::ntapi::ntpebteb::TEB;
		if ::nt_syscall::is_emulated!() {
			::nt_syscall::enter_x64!();
			asm!(
				"mov rax, gs:[0x30]",
				out("rax") teb
			);
			::nt_syscall::enter_x86!();
			Some(::core::mem::transmute::<_, &mut ::ntapi::ntpebteb::TEB>(teb))
		} else {
			None
		}
	}}
}
#[macro_export] 
macro_rules! peb {
	() => {{
		::core::mem::transmute::<_, &mut ::ntapi::ntpebteb::PEB>(::nt_syscall::teb!().ProcessEnvironmentBlock)
	}}
}




#[macro_export]
macro_rules! _get_indices_internal {
	( $arch:ident, $( $( $indices:ident )* ),* ) => {{
		let build = unsafe {
			::nt_syscall::peb!().OSBuildNumber
		};
		let mut value = [ $(::nt_syscall::indices::invalid::$( $indices )* ),* ];

		::nt_syscall::windows_10_20h2!($arch, build, value, $( $( $indices )* ),*);
		::nt_syscall::windows_10_2004!($arch, build, value, $( $( $indices )* ),*);
		::nt_syscall::windows_10_1909!($arch, build, value, $( $( $indices )* ),*);
		::nt_syscall::windows_10_1903!($arch, build, value, $( $( $indices )* ),*);
		::nt_syscall::windows_10_1809!($arch, build, value, $( $( $indices )* ),*);
		::nt_syscall::windows_10_1803!($arch, build, value, $( $( $indices )* ),*);
		::nt_syscall::windows_10_1709!($arch, build, value, $( $( $indices )* ),*);
		::nt_syscall::windows_10_1703!($arch, build, value, $( $( $indices )* ),*);
		::nt_syscall::windows_10_1607!($arch, build, value, $( $( $indices )* ),*);
		::nt_syscall::windows_10_1511!($arch, build, value, $( $( $indices )* ),*);
		::nt_syscall::windows_10_1507!($arch, build, value, $( $( $indices )* ),*);

		::nt_syscall::windows_8_1!($arch, build, value, $( $( $indices )* ),*);
		::nt_syscall::windows_8_0!($arch, build, value, $( $( $indices )* ),*);

		::nt_syscall::windows_server_2012_r2!($arch, build, value, $( $( $indices )* ),*);
		::nt_syscall::windows_server_2012_sp0!($arch, build, value, $( $( $indices )* ),*);

		::nt_syscall::windows_7_sp1!($arch, build, value, $( $( $indices )* ),*);
		::nt_syscall::windows_7_sp0!($arch, build, value, $( $( $indices )* ),*);

		::nt_syscall::windows_server_2008_r2_sp1!($arch, build, value, $( $( $indices )* ),*);
		::nt_syscall::windows_server_2008_r2!($arch, build, value, $( $( $indices )* ),*);
		::nt_syscall::windows_server_2008_sp2!($arch, build, value, $( $( $indices )* ),*);
		::nt_syscall::windows_server_2008_sp0!($arch, build, value, $( $( $indices )* ),*);

		::nt_syscall::windows_vista_sp2!($arch, build, value, $( $( $indices )* ),*);
		::nt_syscall::windows_vista_sp1!($arch, build, value, $( $( $indices )* ),*);
		::nt_syscall::windows_vista_sp0!($arch, build, value, $( $( $indices )* ),*);

		::nt_syscall::windows_server_2003_r2_sp2!($arch, build, value, $( $( $indices )* ),*);
		::nt_syscall::windows_server_2003_r2!($arch, build, value, $( $( $indices )* ),*);
		::nt_syscall::windows_server_2003_sp2!($arch, build, value, $( $( $indices )* ),*);
		::nt_syscall::windows_server_2003_sp1!($arch, build, value, $( $( $indices )* ),*);
		::nt_syscall::windows_server_2003_sp0!($arch, build, value, $( $( $indices )* ),*);

		::nt_syscall::windows_xp_sp3!($arch, build, value, $( $( $indices )* ),*);
		::nt_syscall::windows_xp_sp2!($arch, build, value, $( $( $indices )* ),*);
		::nt_syscall::windows_xp_sp1!($arch, build, value, $( $( $indices )* ),*);
		::nt_syscall::windows_xp_sp0!($arch, build, value, $( $( $indices )* ),*);

		value
	}};
}

#[macro_export]
macro_rules! get_indices_x64 {
	( $( $( $indices:ident )* ),* ) => {{
		::nt_syscall::_get_indices_internal!(x64, $( $( $indices )* ),* )
	}}
}
#[macro_export]
macro_rules! get_indices_x86 {
	( $( $( $indices:ident )* ),* ) => {{
		::nt_syscall::_get_indices_internal!(x86, $( $( $indices )* ),* )
	}}
}

#[macro_export]
#[cfg(target_arch="x86_64")]
macro_rules! get_indices {
	( $( $( $indices:ident )* ),* ) => {{
		::nt_syscall::get_indices_x64!( $( $( $indices )* ),* )
	}};
}
#[macro_export]
#[cfg(target_arch="x86")]
macro_rules! get_indices {
	( $( $( $indices:ident )* ),* ) => {{
		match ::nt_syscall::cpu_mode!() {
			::nt_syscall::CpuMode::EmulatedX86 => {
				::nt_syscall::get_indices_x64!( $( $( $indices )* ),* )
			}
			::nt_syscall::CpuMode::NativeX86 => {
				::nt_syscall::get_indices_x86!( $( $( $indices )* ),* )
			}
			_ => {
				[ $(::nt_syscall::indices::invalid::$( $indices )* ),* ]
			}
		}
	}};
}

#[macro_export]
macro_rules! are_indices_valid {
	( $( $indices:expr )* ) => {{
		let mut is_valid = true;
		for index in $( $indices )* {
			if index == u32::MAX {
				is_valid = false;
				break;
			}
		}
		is_valid
	}};
}




macro_rules! add_version {
	($build:expr, $version:ident, $feature:expr) => {
		#[cfg(not(feature = $feature))]
		#[macro_export]
		macro_rules! $version {
			($$build_2:expr, $$values:expr, $$( $$indices:ident ),*) => { }
		}
		
		#[cfg(feature = $feature)]
		#[macro_export]
		macro_rules! $version {
			($$arch:ident, $$build_2:expr, $$values:expr, $$( $$indices:ident ),*) => {
				if $$build_2 == $build {
					$values = [
						$$( ::nt_syscall::indices::$arch::$version::$$indices ),*
					];
				}
			}
		}
	};
}

add_version!(19042, windows_10_20h2, "windows_10_20h2");
add_version!(19041, windows_10_2004, "windows_10_2004");
add_version!(18363, windows_10_1909, "windows_10_1909");
add_version!(18362, windows_10_1903, "windows_10_1903");
add_version!(17763, windows_10_1809, "windows_10_1809");
add_version!(17134, windows_10_1803, "windows_10_1803");
add_version!(16299, windows_10_1709, "windows_10_1709");
add_version!(15063, windows_10_1703, "windows_10_1703");
add_version!(14393, windows_10_1607, "windows_10_1607");
add_version!(10586, windows_10_1511, "windows_10_1511");
add_version!(10240, windows_10_1507, "windows_10_1507");
add_version!(9600, windows_8_1, "windows_8_1");
add_version!(9200, windows_8_0, "windows_8_0");
add_version!(9600, windows_server_2012_r2, "windows_server_2012_r2");
add_version!(9200, windows_server_2012_sp0, "windows_server_2012_sp0");
add_version!(7601, windows_7_sp1, "windows_7_sp1");
add_version!(7600, windows_7_sp0, "windows_7_sp0");
add_version!(7601, windows_server_2008_r2_sp1, "windows_server_2008_r2_sp1");
add_version!(7600, windows_server_2008_r2, "windows_server_2008_r2");
add_version!(6002, windows_server_2008_sp2, "windows_server_2008_sp2");
add_version!(6001, windows_server_2008_sp0, "windows_server_2008_sp0");
add_version!(6002, windows_vista_sp2, "windows_vista_sp2");
add_version!(6001, windows_vista_sp1, "windows_vista_sp1");
add_version!(6000, windows_vista_sp0, "windows_vista_sp0");
add_version!(3790, windows_server_2003_r2_sp2, "windows_server_2003_r2_sp2");
add_version!(3790, windows_server_2003_r2, "windows_server_2003_r2");
add_version!(3790, windows_server_2003_sp2, "windows_server_2003_sp2");
add_version!(3790, windows_server_2003_sp1, "windows_server_2003_sp1");
add_version!(3790, windows_server_2003_sp0, "windows_server_2003_sp0");
add_version!(2600, windows_xp_sp3, "windows_xp_sp3");
add_version!(2600, windows_xp_sp2, "windows_xp_sp2");
add_version!(2600, windows_xp_sp1, "windows_xp_sp1");
add_version!(2600, windows_xp_sp0, "windows_xp_sp0");
