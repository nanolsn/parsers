use crate::{
    rule::Rule,
    ruled::Ruled,
};

#[derive(Copy, Clone, Debug)]
pub struct OrElse<R, F>(pub R, pub F);

impl<I, R, F, K> Rule<I> for OrElse<R, F>
    where
        R: Rule<I>,
        F: FnOnce(R::Exp) -> K,
        K: Rule<I>,
        I: Copy,
        R::Mat: Into<K::Mat>,
{
    type Exp = K::Exp;
    type Mat = K::Mat;

    fn rule(self, input: I) -> Ruled<I, Self::Res, Self::Err> {
        let OrElse(p, f) = self;

        p.rule(input)
            .map(|r| r.into())
            .or_else(|e| f(e).rule(input))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        rul::rul,
        SomeOf,
    };

    #[test]
    fn or_else() {
        let r = rul("qw").or_else(|_| rul('1'));
        assert_eq!(r.rule("qw"), Ruled::Match("qw", ""));
        assert_eq!(r.rule("1"), Ruled::Match("1", ""));
        assert_eq!(r.rule("."), Ruled::Expected(SomeOf::Char('1')));
    }
}
