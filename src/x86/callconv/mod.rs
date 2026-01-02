mod cdecl;
pub use cdecl::*;

mod fastcall;
pub use fastcall::*;

mod stdcall;
pub use stdcall::*;

mod syscall;
pub use syscall::*;

mod thiscall;
pub use thiscall::*;

mod vectorcall;
pub use vectorcall::*;
