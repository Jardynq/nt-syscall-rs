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

pub(crate) macro count_args_helper {
    (($($cb:tt)*), ($($args:tt)*), $($tail:tt)*) => {
        count_args_helper!(@count ($($cb)*), ($($args)*), (0), $($tail)*)
    },

    (@count ($($cb:tt)*), ($($args:tt)*), ($($acc:tt)*), fn $arg:ident   $(, $($tail:tt)*)?) => { count_args_helper!(@count ($($cb)*), ($($args)*), (1 + $($acc)*), $($($tail)*)?) },
    (@count ($($cb:tt)*), ($($args:tt)*), ($($acc:tt)*), &mut $arg:ident $(, $($tail:tt)*)?) => { count_args_helper!(@count ($($cb)*), ($($args)*), (1 + $($acc)*), $($($tail)*)?) },
    (@count ($($cb:tt)*), ($($args:tt)*), ($($acc:tt)*), &$arg:ident     $(, $($tail:tt)*)?) => { count_args_helper!(@count ($($cb)*), ($($args)*), (1 + $($acc)*), $($($tail)*)?) },
    (@count ($($cb:tt)*), ($($args:tt)*), ($($acc:tt)*), float $arg:expr $(, $($tail:tt)*)?) => { count_args_helper!(@count ($($cb)*), ($($args)*), (1 + $($acc)*), $($($tail)*)?) },
    (@count ($($cb:tt)*), ($($args:tt)*), ($($acc:tt)*), $arg:expr       $(, $($tail:tt)*)?) => { count_args_helper!(@count ($($cb)*), ($($args)*), (1 + $($acc)*), $($($tail)*)?) },
    (@count ($($cb:tt)*), ($($args:tt)*), ($($acc:tt)*), )                                   => { count_args_helper!(@call ($($cb)*), ($($args)*), $($acc)*) },

    (@call ($($cb:tt)*), ($($args:tt)*), 0) => { $($cb)*!($($args)* 0) },
    (@call ($($cb:tt)*), ($($args:tt)*), 1 + 0) => { $($cb)*!($($args)* 1) },
    (@call ($($cb:tt)*), ($($args:tt)*), 1 + 1 + 0) => { $($cb)*!($($args)* 2) },
    (@call ($($cb:tt)*), ($($args:tt)*), 1 + 1 + 1 + 0) => { $($cb)*!($($args)* 3) },
    (@call ($($cb:tt)*), ($($args:tt)*), 1 + 1 + 1 + 1 + 0) => { $($cb)*!($($args)* 4) },
    (@call ($($cb:tt)*), ($($args:tt)*), 1 + 1 + 1 + 1 + 1 + 0) => { $($cb)*!($($args)* 5) },
    (@call ($($cb:tt)*), ($($args:tt)*), 1 + 1 + 1 + 1 + 1 + 1 + 0) => { $($cb)*!($($args)* 6) },
    (@call ($($cb:tt)*), ($($args:tt)*), 1 + 1 + 1 + 1 + 1 + 1 + 1 + 0) => { $($cb)*!($($args)* 7) },
    (@call ($($cb:tt)*), ($($args:tt)*), 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 0) => { $($cb)*!($($args)* 8) },
    (@call ($($cb:tt)*), ($($args:tt)*), 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 0) => { $($cb)*!($($args)* 9) },
    (@call ($($cb:tt)*), ($($args:tt)*), 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 0) => { $($cb)*!($($args)* 10) },
    (@call ($($cb:tt)*), ($($args:tt)*), 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 0) => { $($cb)*!($($args)* 11) },
    (@call ($($cb:tt)*), ($($args:tt)*), 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 0) => { $($cb)*!($($args)* 12) },
    (@call ($($cb:tt)*), ($($args:tt)*), 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 0) => { $($cb)*!($($args)* 13) },
}

define_args!(u32, f32, args32);
define_args!(u64, f64, args64);

pub macro decode($($lit:tt)+) {
    const_str::hex!(
        const_str::split!(
            const_str::trim_ascii!(
                const_str::replace!(
                    const_str::replace!(
                        const_str::replace!(
                            const_str::replace!(
                                $($lit)+,
                                "0x",
                                ""
                            ),
                            "\n.byte ",
                            ", "
                        ),
                        ".byte ",
                        ""
                    ),
                    "\n",
                    ""
                )
            ),
            ", "
        )
    )
}
