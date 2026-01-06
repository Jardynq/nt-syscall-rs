pub(crate) macro args($uty:ty, $($args:tt)*) {{
    const COUNT: usize = count_args!($($args)*);
    const SIZE: usize = COUNT * core::mem::size_of::<$uty>();
    #[allow(unused_mut)]
    let mut buffer = [0u8; SIZE];
    #[allow(unused_mut)]
    let mut ptr = buffer.as_mut_ptr() as *mut $uty;
    macro push($$value:expr) {{
        unsafe {
            *ptr = $$value as $uty;
            ptr = ptr.add(1);
        }
    }}
    munch_args!($uty, push, $($args)*);
    let _ = ptr;
    buffer
}}

pub(crate) macro args_in($uty:ty, $buffer:expr, $($args:tt)*) {{
    #[allow(unused_mut)]
    let mut ptr = ($buffer).as_ptr() as *mut $uty;
    macro push($$value:expr) {{
        unsafe {
            *ptr = $$value as $uty;
            ptr = ptr.add(1);
        }
    }}
    munch_args!($uty, push, $($args)*);
    let _ = ptr;
}}

pub(crate) macro count_args {
    ($(,)?)                                 => { 0 },
    (ptr: $arg:expr     $(, $($tail:tt)*)?) => { count_args!($($($tail)*)?) + 1 },
    ($ty:ty: $arg:expr  $(, $($tail:tt)*)?) => { count_args!($($($tail)*)?) + 1 },
    ($arg:expr          $(, $($tail:tt)*)?) => { count_args!($($($tail)*)?) + 1 },
}

pub(crate) macro count_types {
    ($cb:ident ($($count:tt)+), $ty:tt $(, $($tail:tt)*)?) => {
        count_types!($cb ($($count)+ + 1) $(, $($tail)*)?)
    },
    ($cb:ident ($($count:tt)+) $(,)?) => {
        $cb!(@ $($count)+)
    },
}

pub(crate) macro munch_args {
    ($uty:ty, $push:ident $(,)?) => {},

    ($uty:ty, $push:ident, ptr: $arg:expr $(, $($tail:tt)*)?) => {
        $push!(core::mem::transmute::<_, usize>($arg));
        $( munch_args!($uty, $push, $($tail)*); )?
    },
    ($uty:ty, $push:ident, f32: $arg:expr $(, $($tail:tt)*)?) => {
        $push!((($arg) as f32).to_bits());
        $( munch_args!($uty, $push, $($tail)*); )?
    },
    ($uty:ty, $push:ident, f64: $arg:expr $(, $($tail:tt)*)?) => {
        $push!((($arg) as f64).to_bits());
        $( munch_args!($uty, $push, $($tail)*); )?
    },
    ($uty:ty, $push:ident, $ty:ty: $arg:expr $(, $($tail:tt)*)?) => {
        $push!(($arg) as $ty);
        $( munch_args!($uty, $push, $($tail)*); )?
    },
    ($uty:ty, $push:ident, $arg:expr $(, $($tail:tt)*)?) => {
        $push!(($arg));
        $( munch_args!($uty, $push, $($tail)*); )?
    },
}
