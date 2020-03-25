use crate::{
    apply::Apply,
    ruled::Ruled,
};

#[derive(Copy, Clone, Debug)]
pub struct AndThen<R, F>(pub R, pub F);

impl<I, R, F, K> Apply<I> for AndThen<R, F>
    where
        R: Apply<I>,
        F: Fn(R::Res) -> K,
        K: Apply<I, Err=R::Err>,
{
    type Err = R::Err;
    type Res = K::Res;

    fn apply(&self, input: I) -> Ruled<I, Self::Res, Self::Err> {
        self.0.apply(input)
            .and_then(|r, i| (self.1)(r).apply(i))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        apply::apply,
        rule::rule,
    };

    //noinspection RsBorrowChecker
    #[test]
    fn and_then() {
        let r = (rule("qw") | '1')
            .and_then(|s: &str| rule('.') >> s);

        assert_eq!(apply(r, "qw.qw"), Ruled::Ok("qw", ""));
        assert_eq!(apply(r, "1.1"), Ruled::Ok("1", ""));
        assert_eq!(apply(r, "qw.1"), Ruled::Err(()));
        assert_eq!(apply(r, "."), Ruled::Err(()));
    }
}
