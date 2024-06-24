//! When using the move keyword, all external variables used in the closure will be moved into the closure.
//! 
//! Sometimes you may only need to move a few variables, and the rest of the variables will remain referenced.
//! 
//! At this time, you can use closure-capture to specify the variables to be captured.
//! 
//! ### Usage
//! 
//! link `closure-capture`
//! 
//! cargo.toml
//! ```toml
//! [dependencies]
//! closure-capture = "0.1"
//! ```
//! 
//! Move variables a and b into the closure
//! 
//! ```rust
//! fn main() {
//!     let a = 1;
//!     let b = 2;
//!     
//!     std::thread::spawn(closure_capture::closure!([a, b] () {
//!         println!("{}", a + b)
//!     }))
//!     .join()
//!     .unwrap();
//! }
//! ```
//! 
//! Move variables a and b into the closure and modify a
//! 
//! ```rust
//! fn main() {
//!     let a = 1;
//!     let b = 2;
//!     
//!     std::thread::spawn(closure_capture::closure!([mut a, b] () {
//!         a += b;
//!         println!("{}", a)
//!     }))
//!     .join()
//!     .unwrap();
//! }
//! ```
//! 
//! With async block
//! 
//! ```rust
//! #[tokio::main]
//! async fn main() {
//!     let a = 1;
//!     let b = 2;
//! 
//!     tokio::spawn(closure_capture::async_block!([mut a, b] async {
//!         a += b;
//!         println!("{}", a)
//!     }))
//!     .await
//!     .unwrap();
//! }
//! ```

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
    ([$($args1:tt)*] ($($args2:tt)*) -> $ret:ty $block:block) => {
        {
            $crate::_wrap_process!($($args1)*);

            |$($args2)*| -> $ret {
                $crate::_move_process!($($args1)*);
                $block
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