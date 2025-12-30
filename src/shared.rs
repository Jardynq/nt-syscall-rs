macro_rules! define_args {
    ($ty:ty, $fty:ty, $name:ident) => {
        pub macro $name($$($$args:tt)*) {{
            const COUNT: usize = count_args!($$($$args)*);
            const SIZE: usize = COUNT * core::mem::size_of::<$ty>();
            #[allow(unused_mut)]
            let mut buffer = core::hint::black_box([0u8; SIZE]);
            #[allow(unused_mut)]
            let mut ptr = buffer.as_mut_ptr() as *mut $ty;
            macro push($$value:expr) {{
                unsafe {
                    *ptr = $$value;
                    ptr = ptr.add(1);
                }
            }}
            munch_args!($ty, $fty, push, $$($$args)*);
            let _ = ptr;
            core::hint::black_box(buffer)
        }}
    }
}

pub(crate) macro munch_args {
    ($ty:ty, $fty:ty, $push:ident $(,)?) => {},
    ($ty:ty, $fty:ty, $push:ident, fn $arg:ident $(, $($tail:tt)*)?) => {
        $push!(($arg) as *const () as usize as $ty);
        $( munch_args!($ty, $fty, $push, $($tail)*); )?
    },
    ($ty:ty, $fty:ty, $push:ident, &mut $arg:ident $(, $($tail:tt)*)?) => {
        $push!((&mut $arg) as *mut $ty as $ty);
        $( munch_args!($ty, $fty, $push, $($tail)*); )?
    },
    ($ty:ty, $fty:ty, $push:ident, &$arg:ident $(, $($tail:tt)*)?) => {
        $push!((& $arg) as *const $ty as $ty);
        $( munch_args!($ty, $fty, $push, $($tail)*); )?
    },
    ($ty:ty, $fty:ty, $push:ident, float $arg:expr $(, $($tail:tt)*)?) => {
        $push!((($arg) as $fty).to_bits() as $ty);
        $( munch_args!($ty, $fty, $push, $($tail)*); )?
    },
    ($ty:ty, $fty:ty, $push:ident, $arg:expr $(, $($tail:tt)*)?) => {
        $push!(($arg) as $ty);
        $( munch_args!($ty, $fty, $push, $($tail)*); )?
    },
}

pub(crate) macro count_args {
    ()                                      => { 0 },
    (fn $arg:ident      $(, $($tail:tt)*)?) => { 1 + count_args!($($($tail)*)?) },
    (&mut $arg:ident    $(, $($tail:tt)*)?) => { 1 + count_args!($($($tail)*)?) },
    (&$arg:ident        $(, $($tail:tt)*)?) => { 1 + count_args!($($($tail)*)?) },
    (float $arg:expr    $(, $($tail:tt)*)?) => { 1 + count_args!($($($tail)*)?) },
    ($arg:expr          $(, $($tail:tt)*)?) => { 1 + count_args!($($($tail)*)?) },
}

define_args!(u32, f32, args32);
define_args!(u64, f64, args64);
