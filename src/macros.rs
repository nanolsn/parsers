#[macro_export]
/// Still in development.
macro_rules! rule {
    ( ( $($e:tt)+ ) ) => {
        rule!($($e)+)
    };

    ($l:tt & $r:tt $($rest:tt)+) => {
        rule!(($l & $r) $($rest)+)
    };

    ($l:tt & $($r:tt)+) => {
        rule!($l).cat(rule!($($r)+))
    };

    ($l:tt | $($r:tt)+) => {
        rule!($l).or(rule!($($r)+))
    };

    ($e:tt) => {
        rule($e)
    };
}
