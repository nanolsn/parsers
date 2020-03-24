#[macro_export]
macro_rules! rule {
    ($l:tt * $m:tt $op:tt $($r:tt)*) => {
        (rule!($l) * $m) $op rule!($($r)*)
    };

    ($l:tt & $($r:tt)*) => {
        rule!($l) & rule!($($r)*)
    };

    ($l:tt | $($r:tt)*) => {
        rule!($l) | rule!($($r)*)
    };

    ($e:expr) => {
        Rule($e)
    };
}
