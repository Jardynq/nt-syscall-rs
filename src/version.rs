#[macro_export] 
#[cfg(target_arch="x86_64")]
macro_rules! teb {
	() => {{
		let mut teb: *mut ::ntapi::ntpebteb::TEB;
		asm!(
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
macro_rules! ordinal_macro {
	( $arch:ident, $( $( $ordinal:ident )* ),* ) => {{
		let build = unsafe {
			::nt_syscall::peb!().OSBuildNumber
		};
		let mut value = [ $(::nt_syscall::ordinal_invalid::$( $ordinal )* ),* ];

		::nt_syscall::windows_10_20h2!($arch, build, value, $( $( $ordinal )* ),*);
		::nt_syscall::windows_10_2004!($arch, build, value, $( $( $ordinal )* ),*);
		::nt_syscall::windows_10_1909!($arch, build, value, $( $( $ordinal )* ),*);
		::nt_syscall::windows_10_1903!($arch, build, value, $( $( $ordinal )* ),*);
		::nt_syscall::windows_10_1809!($arch, build, value, $( $( $ordinal )* ),*);
		::nt_syscall::windows_10_1803!($arch, build, value, $( $( $ordinal )* ),*);
		::nt_syscall::windows_10_1709!($arch, build, value, $( $( $ordinal )* ),*);
		::nt_syscall::windows_10_1703!($arch, build, value, $( $( $ordinal )* ),*);
		::nt_syscall::windows_10_1607!($arch, build, value, $( $( $ordinal )* ),*);
		::nt_syscall::windows_10_1511!($arch, build, value, $( $( $ordinal )* ),*);
		::nt_syscall::windows_10_1507!($arch, build, value, $( $( $ordinal )* ),*);

		::nt_syscall::windows_8_1!($arch, build, value, $( $( $ordinal )* ),*);
		::nt_syscall::windows_8_0!($arch, build, value, $( $( $ordinal )* ),*);

		::nt_syscall::windows_server_2012_r2!($arch, build, value, $( $( $ordinal )* ),*);
		::nt_syscall::windows_server_2012_sp0!($arch, build, value, $( $( $ordinal )* ),*);

		::nt_syscall::windows_7_sp1!($arch, build, value, $( $( $ordinal )* ),*);
		::nt_syscall::windows_7_sp0!($arch, build, value, $( $( $ordinal )* ),*);

		::nt_syscall::windows_server_2008_r2_sp1!($arch, build, value, $( $( $ordinal )* ),*);
		::nt_syscall::windows_server_2008_r2!($arch, build, value, $( $( $ordinal )* ),*);
		::nt_syscall::windows_server_2008_sp2!($arch, build, value, $( $( $ordinal )* ),*);
		::nt_syscall::windows_server_2008_sp0!($arch, build, value, $( $( $ordinal )* ),*);

		::nt_syscall::windows_vista_sp2!($arch, build, value, $( $( $ordinal )* ),*);
		::nt_syscall::windows_vista_sp1!($arch, build, value, $( $( $ordinal )* ),*);
		::nt_syscall::windows_vista_sp0!($arch, build, value, $( $( $ordinal )* ),*);

		::nt_syscall::windows_server_2003_r2_sp2!($arch, build, value, $( $( $ordinal )* ),*);
		::nt_syscall::windows_server_2003_r2!($arch, build, value, $( $( $ordinal )* ),*);
		::nt_syscall::windows_server_2003_sp2!($arch, build, value, $( $( $ordinal )* ),*);
		::nt_syscall::windows_server_2003_sp1!($arch, build, value, $( $( $ordinal )* ),*);
		::nt_syscall::windows_server_2003_sp0!($arch, build, value, $( $( $ordinal )* ),*);

		::nt_syscall::windows_xp_sp3!($arch, build, value, $( $( $ordinal )* ),*);
		::nt_syscall::windows_xp_sp2!($arch, build, value, $( $( $ordinal )* ),*);
		::nt_syscall::windows_xp_sp1!($arch, build, value, $( $( $ordinal )* ),*);
		::nt_syscall::windows_xp_sp0!($arch, build, value, $( $( $ordinal )* ),*);

		value
	}};
}

#[macro_export]
macro_rules! ordinal_x64 {
	( $( $( $ordinal:ident )* ),* ) => {{
		::nt_syscall::ordinal_macro!(ordinal_x64, $( $( $ordinal )* ),* )
	}}
}
#[macro_export]
macro_rules! ordinal_x86 {
	( $( $( $ordinal:ident )* ),* ) => {{
		::nt_syscall::ordinal_macro!(ordinal_x86, $( $( $ordinal )* ),* )
	}}
}

#[macro_export]
#[cfg(target_arch="x86_64")]
macro_rules! ordinal {
	( $( $( $ordinal:ident )* ),* ) => {{
		::nt_syscall::ordinal_x64!( $( $( $ordinal )* ),* )
	}};
}
#[macro_export]
#[cfg(target_arch="x86")]
macro_rules! ordinal {
	( $( $( $ordinal:ident )* ),* ) => {{
		match ::nt_syscall::cpu_mode!() {
			::nt_syscall::CpuMode::EmulatedX86 => {
				::nt_syscall::ordinal_x64!( $( $( $ordinal )* ),* )
			}
			::nt_syscall::CpuMode::NativeX86 => {
				::nt_syscall::ordinal_x86!( $( $( $ordinal )* ),* )
			}
			_ => {
				[ $(::nt_syscall::ordinal_invalid::$( $ordinal )* ),* ]
			}
		}
	}};
}

#[macro_export]
macro_rules! ordinal_valid {
	( $( $ordinals:expr )* ) => {{
		let mut is_valid = true;
		for ordinal in $( $ordinals )* {
			if ordinal == u32::MAX {
				is_valid = false;
				break;
			}
		}
		is_valid
	}};
}




#[cfg(not(feature = "windows_10_20h2"))] #[macro_export] macro_rules! windows_10_20h2 { ($b:expr, $v:expr, $( $o:ident ),*) => { } }
#[cfg(feature = "windows_10_20h2")] #[macro_export] macro_rules! windows_10_20h2 {
	($a:ident, $b:expr, $v:expr, $($o:ident),*) => {
		if $b == 19042 {$v = [ $( ::nt_syscall::$a::windows_10_20h2::$o ),* ] }
	}
}
#[cfg(not(feature = "windows_10_2004"))] #[macro_export] macro_rules! windows_10_2004 { ($b:expr, $v:expr, $( $o:ident ),*) => { } }
#[cfg(feature = "windows_10_2004")] #[macro_export] macro_rules! windows_10_2004 {
	($a:ident, $b:expr, $v:expr, $($o:ident),*) => {
		if $b == 19041 {$v = [ $( ::nt_syscall::$a::windows_10_2004::$o ),* ] }
	}
}
#[cfg(not(feature = "windows_10_1909"))] #[macro_export] macro_rules! windows_10_1909 { ($b:expr, $v:expr, $( $o:ident ),*) => { } }
#[cfg(feature = "windows_10_1909")] #[macro_export] macro_rules! windows_10_1909 {
	($a:ident, $b:expr, $v:expr, $($o:ident),*) => {
		if $b == 18363 {$v = [ $( ::nt_syscall::$a::windows_10_1909::$o ),* ] }
	}
}
#[cfg(not(feature = "windows_10_1903"))] #[macro_export] macro_rules! windows_10_1903 { ($b:expr, $v:expr, $( $o:ident ),*) => { } }
#[cfg(feature = "windows_10_1903")] #[macro_export] macro_rules! windows_10_1903 {
	($a:ident, $b:expr, $v:expr, $($o:ident),*) => {
		if $b == 18362 {$v = [ $( ::nt_syscall::$a::windows_10_1903::$o ),* ] }
	}
}
#[cfg(not(feature = "windows_10_1809"))] #[macro_export] macro_rules! windows_10_1809 { ($b:expr, $v:expr, $( $o:ident ),*) => { } }
#[cfg(feature = "windows_10_1809")] #[macro_export] macro_rules! windows_10_1809 {
	($a:ident, $b:expr, $v:expr, $($o:ident),*) => {
		if $b == 17763 {$v = [ $( ::nt_syscall::$a::windows_10_1809::$o ),* ] }
	}
}
#[cfg(not(feature = "windows_10_1803"))] #[macro_export] macro_rules! windows_10_1803 { ($b:expr, $v:expr, $( $o:ident ),*) => { } }
#[cfg(feature = "windows_10_1803")] #[macro_export] macro_rules! windows_10_1803 {
	($a:ident, $b:expr, $v:expr, $($o:ident),*) => {
		if $b == 17134 {$v = [ $( ::nt_syscall::$a::windows_10_1803::$o ),* ] }
	}
}
#[cfg(not(feature = "windows_10_1709"))] #[macro_export] macro_rules! windows_10_1709 { ($b:expr, $v:expr, $( $o:ident ),*) => { } }
#[cfg(feature = "windows_10_1709")] #[macro_export] macro_rules! windows_10_1709 {
	($a:ident, $b:expr, $v:expr, $($o:ident),*) => {
		if $b == 16299 {$v = [ $( ::nt_syscall::$a::windows_10_1709::$o ),* ] }
	}
}
#[cfg(not(feature = "windows_10_1703"))] #[macro_export] macro_rules! windows_10_1703 { ($b:expr, $v:expr, $( $o:ident ),*) => { } }
#[cfg(feature = "windows_10_1703")] #[macro_export] macro_rules! windows_10_1703 {
	($a:ident, $b:expr, $v:expr, $($o:ident),*) => {
		if $b == 15063 {$v = [ $( ::nt_syscall::$a::windows_10_1703::$o ),* ] }
	}
}
#[cfg(not(feature = "windows_10_1607"))] #[macro_export] macro_rules! windows_10_1607 { ($b:expr, $v:expr, $( $o:ident ),*) => { } }
#[cfg(feature = "windows_10_1607")] #[macro_export] macro_rules! windows_10_1607 {
	($a:ident, $b:expr, $v:expr, $($o:ident),*) => {
		if $b == 14393 {$v = [ $( ::nt_syscall::$a::windows_10_1607::$o ),* ] }
	}
}
#[cfg(not(feature = "windows_10_1511"))] #[macro_export] macro_rules! windows_10_1511 { ($b:expr, $v:expr, $( $o:ident ),*) => { } }
#[cfg(feature = "windows_10_1511")] #[macro_export] macro_rules! windows_10_1511 {
	($a:ident, $b:expr, $v:expr, $($o:ident),*) => {
		if $b == 10586 {$v = [ $( ::nt_syscall::$a::windows_10_1511::$o ),* ] }
	}
}
#[cfg(not(feature = "windows_10_1507"))] #[macro_export] macro_rules! windows_10_1507 { ($b:expr, $v:expr, $( $o:ident ),*) => { } }
#[cfg(feature = "windows_10_1507")] #[macro_export] macro_rules! windows_10_1507 {
	($a:ident, $b:expr, $v:expr, $($o:ident),*) => {
		if $b == 10240 {$v = [ $( ::nt_syscall::$a::windows_10_1507::$o ),* ] }
	}
}


#[cfg(not(feature = "windows_8_1"))] #[macro_export] macro_rules! windows_8_1 { ($b:expr, $v:expr, $( $o:ident ),*) => { } }
#[cfg(feature = "windows_8_1")] #[macro_export] macro_rules! windows_8_1 {
	($a:ident, $b:expr, $v:expr, $($o:ident),*) => {
		if $b == 9600 {$v = [ $( ::nt_syscall::$a::windows_8_1::$o ),* ] }
	}
}
#[cfg(not(feature = "windows_8_0"))] #[macro_export] macro_rules! windows_8_0 { ($b:expr, $v:expr, $( $o:ident ),*) => { } }
#[cfg(feature = "windows_8_0")] #[macro_export] macro_rules! windows_8_0 {
	($a:ident, $b:expr, $v:expr, $($o:ident),*) => {
		if $b == 9200 {$v = [ $( ::nt_syscall::$a::windows_8_0::$o ),* ] }
	}
}


#[cfg(not(feature = "windows_server_2012_r2"))] #[macro_export] macro_rules! windows_server_2012_r2 { ($b:expr, $v:expr, $( $o:ident ),*) => { } }
#[cfg(feature = "windows_server_2012_r2")] #[macro_export] macro_rules! windows_server_2012_r2 {
	($a:ident, $b:expr, $v:expr, $($o:ident),*) => {
		if $b == 9600 {$v = [ $( ::nt_syscall::$a::windows_server_2012_r2::$o ),* ] }
	}
}
#[cfg(not(feature = "windows_server_2012_sp0"))] #[macro_export] macro_rules! windows_server_2012_sp0 { ($b:expr, $v:expr, $( $o:ident ),*) => { } }
#[cfg(feature = "windows_server_2012_sp0")] #[macro_export] macro_rules! windows_server_2012_sp0 {
	($a:ident, $b:expr, $v:expr, $($o:ident),*) => {
		if $b == 9200 {$v = [ $( ::nt_syscall::$a::windows_server_2012_sp0::$o ),* ] }
	}
}


#[cfg(not(feature = "windows_7_sp1"))] #[macro_export] macro_rules! windows_7_sp1 { ($b:expr, $v:expr, $( $o:ident ),*) => { } }
#[cfg(feature = "windows_7_sp1")] #[macro_export] macro_rules! windows_7_sp1 {
	($a:ident, $b:expr, $v:expr, $($o:ident),*) => {
		if $b == 7601 {$v = [ $( ::nt_syscall::$a::windows_7_sp1::$o ),* ] }
	}
}
#[cfg(not(feature = "windows_7_sp0"))] #[macro_export] macro_rules! windows_7_sp0 { ($b:expr, $v:expr, $( $o:ident ),*) => { } }
#[cfg(feature = "windows_7_sp0")] #[macro_export] macro_rules! windows_7_sp0 {
	($a:ident, $b:expr, $v:expr, $($o:ident),*) => {
		if $b == 7600 {$v = [ $( ::nt_syscall::$a::windows_7_sp0::$o ),* ] }
	}
}


#[cfg(not(feature = "windows_server_2008_r2_sp1"))] #[macro_export] macro_rules! windows_server_2008_r2_sp1 { ($b:expr, $v:expr, $( $o:ident ),*) => { } }
#[cfg(feature = "windows_server_2008_r2_sp1")] #[macro_export] macro_rules! windows_server_2008_r2_sp1 {
	($a:ident, $b:expr, $v:expr, $($o:ident),*) => {
		if $b == 7601 {$v = [ $( ::nt_syscall::$a::windows_server_2008_r2_sp1::$o ),* ] }
	}
}
#[cfg(not(feature = "windows_server_2008_r2"))] #[macro_export] macro_rules! windows_server_2008_r2 { ($b:expr, $v:expr, $( $o:ident ),*) => { } }
#[cfg(feature = "windows_server_2008_r2")] #[macro_export] macro_rules! windows_server_2008_r2 {
	($a:ident, $b:expr, $v:expr, $($o:ident),*) => {
		if $b == 7600 {$v = [ $( ::nt_syscall::$a::windows_server_2008_r2::$o ),* ] }
	}
}
#[cfg(not(feature = "windows_server_2008_sp2"))] #[macro_export] macro_rules! windows_server_2008_sp2 { ($b:expr, $v:expr, $( $o:ident ),*) => { } }
#[cfg(feature = "windows_server_2008_sp2")] #[macro_export] macro_rules! windows_server_2008_sp2 {
	($a:ident, $b:expr, $v:expr, $($o:ident),*) => {
		if $b == 6002 {$v = [ $( ::nt_syscall::$a::windows_server_2008_sp2::$o ),* ] }
	}
}
#[cfg(not(feature = "windows_server_2008_sp0"))] #[macro_export] macro_rules! windows_server_2008_sp0 { ($b:expr, $v:expr, $( $o:ident ),*) => { } }
#[cfg(feature = "windows_server_2008_sp0")] #[macro_export] macro_rules! windows_server_2008_sp0 {
	($a:ident, $b:expr, $v:expr, $($o:ident),*) => {
		if $b == 6001 {$v = [ $( ::nt_syscall::$a::windows_server_2008_sp0::$o ),* ] }
	}
}


#[cfg(not(feature = "windows_vista_sp2"))] #[macro_export] macro_rules! windows_vista_sp2 { ($b:expr, $v:expr, $( $o:ident ),*) => { } }
#[cfg(feature = "windows_vista_sp2")] #[macro_export] macro_rules! windows_vista_sp2 {
	($a:ident, $b:expr, $v:expr, $($o:ident),*) => {
		if $b == 6002 {$v = [ $( ::nt_syscall::$a::windows_vista_sp2::$o ),* ] }
	}
}
#[cfg(not(feature = "windows_vista_sp1"))] #[macro_export] macro_rules! windows_vista_sp1 { ($b:expr, $v:expr, $( $o:ident ),*) => { } }
#[cfg(feature = "windows_vista_sp1")] #[macro_export] macro_rules! windows_vista_sp1 {
	($a:ident, $b:expr, $v:expr, $($o:ident),*) => {
		if $b == 6001 {$v = [ $( ::nt_syscall::$a::windows_vista_sp1::$o ),* ] }
	}
}
#[cfg(not(feature = "windows_vista_sp0"))] #[macro_export] macro_rules! windows_vista_sp0 { ($b:expr, $v:expr, $( $o:ident ),*) => { } }
#[cfg(feature = "windows_vista_sp0")] #[macro_export] macro_rules! windows_vista_sp0 {
	($a:ident, $b:expr, $v:expr, $($o:ident),*) => {
		if $b == 6000 {$v = [ $( ::nt_syscall::$a::windows_vista_sp0::$o ),* ] }
	}
}


#[cfg(not(feature = "windows_server_2003_r2_sp2"))] #[macro_export] macro_rules! windows_server_2003_r2_sp2 { ($b:expr, $v:expr, $( $o:ident ),*) => { } }
#[cfg(feature = "windows_server_2003_r2_sp2")] #[macro_export] macro_rules! windows_server_2003_r2_sp2 {
	($a:ident, $b:expr, $v:expr, $($o:ident),*) => {
		if $b == 3790 {$v = [ $( ::nt_syscall::$a::windows_server_2003_r2_sp2::$o ),* ] }
	}
}
#[cfg(not(feature = "windows_server_2003_r2"))] #[macro_export] macro_rules! windows_server_2003_r2 { ($b:expr, $v:expr, $( $o:ident ),*) => { } }
#[cfg(feature = "windows_server_2003_r2")] #[macro_export] macro_rules! windows_server_2003_r2 {
	($a:ident, $b:expr, $v:expr, $($o:ident),*) => {
		if $b == 3790 {$v = [ $( ::nt_syscall::$a::windows_server_2003_r2::$o ),* ] }
	}
}
#[cfg(not(feature = "windows_server_2003_sp2"))] #[macro_export] macro_rules! windows_server_2003_sp2 { ($b:expr, $v:expr, $( $o:ident ),*) => { } }
#[cfg(feature = "windows_server_2003_sp2")] #[macro_export] macro_rules! windows_server_2003_sp2 {
	($a:ident, $b:expr, $v:expr, $($o:ident),*) => {
		if $b == 3790 {$v = [ $( ::nt_syscall::$a::windows_server_2003_sp2::$o ),* ] }
	}
}
#[cfg(not(feature = "windows_server_2003_sp1"))] #[macro_export] macro_rules! windows_server_2003_sp1 { ($b:expr, $v:expr, $( $o:ident ),*) => { } }
#[cfg(feature = "windows_server_2003_sp1")] #[macro_export] macro_rules! windows_server_2003_sp1 {
	($a:ident, $b:expr, $v:expr, $($o:ident),*) => {
		if $b == 3790 {$v = [ $( ::nt_syscall::$a::windows_server_2003_sp1::$o ),* ] }
	}
}
#[cfg(not(feature = "windows_server_2003_sp0"))] #[macro_export] macro_rules! windows_server_2003_sp0 { ($b:expr, $v:expr, $( $o:ident ),*) => { } }
#[cfg(feature = "windows_server_2003_sp0")] #[macro_export] macro_rules! windows_server_2003_sp0 {
	($a:ident, $b:expr, $v:expr, $($o:ident),*) => {
		if $b == 3790 {$v = [ $( ::nt_syscall::$a::windows_server_2003_sp0::$o ),* ] }
	}
}


#[cfg(not(feature = "windows_xp_sp3"))] #[macro_export] macro_rules! windows_xp_sp3 { ($b:expr, $v:expr, $( $o:ident ),*) => { } }
#[cfg(feature = "windows_xp_sp3")] #[macro_export] macro_rules! windows_xp_sp3 {
	($a:ident, $b:expr, $v:expr, $($o:ident),*) => {
		if $b == 2600 {$v = [ $( ::nt_syscall::$a::windows_xp_sp3::$o ),* ] }
	}
}
#[cfg(not(feature = "windows_xp_sp2"))] #[macro_export] macro_rules! windows_xp_sp2 { ($b:expr, $v:expr, $( $o:ident ),*) => { } }
#[cfg(feature = "windows_xp_sp2")] #[macro_export] macro_rules! windows_xp_sp2 {
	($a:ident, $b:expr, $v:expr, $($o:ident),*) => {
		if $b == 2600 {$v = [ $( ::nt_syscall::$a::windows_xp_sp2::$o ),* ] }
	}
}
#[cfg(not(feature = "windows_xp_sp1"))] #[macro_export] macro_rules! windows_xp_sp1 { ($b:expr, $v:expr, $( $o:ident ),*) => { } }
#[cfg(feature = "windows_xp_sp1")] #[macro_export] macro_rules! windows_xp_sp1 {
	($a:ident, $b:expr, $v:expr, $($o:ident),*) => {
		if $b == 2600 {$v = [ $( ::nt_syscall::$a::windows_xp_sp1::$o ),* ] }
	}
}
#[cfg(not(feature = "windows_xp_sp0"))] #[macro_export] macro_rules! windows_xp_sp0 { ($b:expr, $v:expr, $( $o:ident ),*) => { } }
#[cfg(feature = "windows_xp_sp0")] #[macro_export] macro_rules! windows_xp_sp0 {
	($a:ident, $b:expr, $v:expr, $($o:ident),*) => {
		if $b == 2600 {$v = [ $( ::nt_syscall::$a::windows_xp_sp0::$o ),* ] }
	}
}
