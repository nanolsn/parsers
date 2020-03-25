use crate::{
    apply::Apply,
    ruled::Ruled,
};

#[derive(Copy, Clone, Debug)]
pub struct OrElse<R, F>(pub R, pub F);

impl<I, R, F, K> Apply<I> for OrElse<R, F>
    where
        R: Apply<I>,
        F: Fn(R::Err) -> K,
        K: Apply<I, Res=R::Res>,
        I: Copy,
{
    type Err = K::Err;
    type Res = R::Res;

    fn apply(&self, input: I) -> Ruled<I, Self::Res, Self::Err> {
        self.0.apply(input)
            .or_else(|e| (self.1)(e).apply(input))
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
    fn or_else() {
        let r = rule("qw").or_else(|_| rule('1'));
        assert_eq!(apply(r, "qw"), Ruled::Ok("qw", ""));
        assert_eq!(apply(r, "1"), Ruled::Ok("1", ""));
        assert_eq!(apply(r, "."), Ruled::Err(()));
    }
}
