#[repr(transparent)]
#[doc(hidden)]
pub struct _Wrap<T: ?Sized>(pub T);

#[macro_export]
#[doc(hidden)]
macro_rules! _wrap_process {
    ($arg:ident, $($args:tt)*) => {
        $crate::_wrap_process!($arg);
        $crate::_wrap_process!($($args)*);
    };
    (mut $arg:ident, $($args:tt)*) => {
        $crate::_wrap_process!($arg);
        $crate::_wrap_process!($($args)*);
    };
    (mut $arg:ident) => {
        $crate::_wrap_process!($arg);
    };
    ($arg:ident) => {
        let $arg = $crate::_Wrap($arg);
    };
    () => {};
}

#[macro_export]
#[doc(hidden)]
macro_rules! _move_process {
     ($arg:ident, $($args:tt)*) => {
        $crate::_move_process!($arg);
        $crate::_move_process!($($args)*);
    };
    (mut $arg:ident, $($args:tt)*) => {
        $crate::_move_process!(mut $arg);
        $crate::_move_process!($($args)*);
    };
    (mut $arg:ident) => {
       let $arg = $arg;
       let mut $arg = $arg.0;
    };
    ($arg:ident) => {
       let $arg = $arg;
       let $arg = $arg.0;
    };
    () => {};
}

#[macro_export]
macro_rules! closure {
    ([$($args1:tt)*] ($($args2:tt)*) $expr:expr) => {
        {
            $crate::_wrap_process!($($args1)*);

            |$($args2)*| {
                $crate::_move_process!($($args1)*);
                $expr
            }
        }
    };
    ([$($args1:tt)*] ($($args2:tt)*) -> $ret:ty $expr:expr) => {
        {
            $crate::_wrap_process!($($args1)*);

            |$($args2)*| -> $ret {
                $crate::_move_process!($($args1)*);
                $expr
            }
        }
    };
}

#[macro_export]
macro_rules! async_block {
    ([$($args1:tt)*] async $block:block) => {
        {
            $crate::_wrap_process!($($args1)*);

            async {
                $crate::_move_process!($($args1)*);
                $block
            }
        }
    };
}