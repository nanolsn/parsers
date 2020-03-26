#[macro_export]
macro_rules! rule {
    ($e:tt) => {
        rule_cat!($e)
    };
}

#[macro_export]
macro_rules! rule_cat {
    ( ( $($e:tt)+ ) ) => {
        rule_cat!($($e)+)
    };

    ($l:tt & $r:tt $($rest:tt)+) => {
        rule_cat!(($l & $r) $($rest)+)
    };

    ($l:tt & $($r:tt)+) => {
        rule_cat!($l).cat(rule_cat!($($r)+))
    };

    ($l:tt | $($r:tt)+) => {
        rule_cat!($l).or(rule_cat!($($r)+))
    };

    ($e:tt) => {
        rule($e)
    };
}
