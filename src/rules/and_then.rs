use crate::{
    apply::Apply,
    ruled::Ruled,
};

#[derive(Copy, Clone, Debug)]
pub struct AndThen<R, F>(pub R, pub F);

impl<I, R, F, K> Apply<I> for AndThen<R, F>
    where
        R: Apply<I>,
        F: FnOnce(R::Res) -> K,
        K: Apply<I>,
        R::Err: Into<K::Err>,
{
    type Err = K::Err;
    type Res = K::Res;

    fn apply(self, input: I) -> Ruled<I, Self::Res, Self::Err> {
        let AndThen(p, f) = self;

        p.apply(input)
            .map_err(|e| e.into())
            .and_then(|r, i| f(r).apply(i))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        apply::apply,
        rule::rule,
        expected::Expected,
    };

    //noinspection RsBorrowChecker
    #[test]
    fn and_then() {
        let r = (rule("qw") | '1')
            .and_then(|s: &str| rule('.') >> s);

        assert_eq!(apply(r, "qw.qw"), Ruled::Ok("qw", ""));
        assert_eq!(apply(r, "1.1"), Ruled::Ok("1", ""));
        assert_eq!(apply(r, "qw.1"), Ruled::Err(Expected::Str("qw")));
        assert_eq!(apply(r, "."), Ruled::Err(Expected::Char('1')));
    }
}
