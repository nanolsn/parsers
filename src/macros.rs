#[macro_export]
macro_rules! rule {
    ($l:tt * $m:tt $op:tt $($r:tt)*) => {
        rule!((rule!($l) * $m) $op $($r)*)
    };

    ($l:tt & $($r:tt)*) => {
        (rule!($l)).cat(rule!($($r)*))
    };

    ($l:tt | $($r:tt)*) => {
        (rule!($l)).or(rule!($($r)*))
    };

    ($e:expr) => {
        rule($e)
    };
}
