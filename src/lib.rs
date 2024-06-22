#[repr(transparent)]
pub struct Wrap<T: ?Sized>(T);

macro_rules! wrap_process {
    ($arg:ident, $($args:tt)*) => {
        wrap_process!($arg);
        wrap_process!($($args)*);
    };
    (mut $arg:ident, $($args:tt)*) => {
        wrap_process!($arg);
        wrap_process!($($args)*);
    };
    (mut $arg:ident) => {
        wrap_process!($arg);
    };
    ($arg:ident) => {
        let $arg = crate::Wrap($arg);
    };
    () => {};
}

macro_rules! move_process {
     ($arg:ident, $($args:tt)*) => {
        move_process!($arg);
        move_process!($($args)*);
    };
    (mut $arg:ident, $($args:tt)*) => {
        move_process!(mut $arg);
        move_process!($($args)*);
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

macro_rules! closure {
    ([$($args1:tt)*] ($($args2:tt)*) $expr:expr) => {
        {
            wrap_process!($($args1)*);

            |$($args2)*| {
                move_process!($($args1)*);
                $expr
            }
        }
    };
    ([$($args1:tt)*] ($($args2:tt)*) -> $ret:tt $expr:expr) => {
        {
            wrap_process!($($args1)*);

            |$($args2)*| -> $ret {
                move_process!($($args1)*);
                $expr
            }
        }
    };
}

pub fn test() {
    let a = 1;
    let b = 2;

    let r = closure!([a, mut b] (c: i32, d) -> i32 {
        b -= 1;
        a + b + c + d
    })(1, 1);

    println!("{}", r)
}

#[cfg(test)]
mod test {

    #[test]
    fn test1() {
        let a = 1;
        let b = 2;

        let r = closure!([a, mut b] (c: i32, d) -> i32 {
            b -= 1;
            a + b + c + d
        })(1, 1);

        println!("{}", r)
    }
}