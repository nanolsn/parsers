use crate::{
    rule::Rule,
    ruled::Ruled,
};

#[derive(Copy, Clone, Debug)]
pub struct AndThen<R, F>(pub R, pub F);

impl<I, R, F, K> Rule<I> for AndThen<R, F>
    where
        R: Rule<I>,
        F: FnOnce(R::Mat) -> K,
        K: Rule<I>,
        R::Exp: Into<K::Exp>,
{
    type Exp = K::Exp;
    type Mat = K::Mat;

    fn rule(self, input: I) -> Ruled<I, Self::Res, Self::Err> {
        let AndThen(p, f) = self;

        p.rule(input)
            .map_exp(|e| e.into())
            .and_then(|r, i| f(r).rule(i))
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
    fn and_then() {
        let r = (rul("qw") | '1')
            .and_then(|s: &str| rul('.') >> s);

        assert_eq!(r.rule("qw.qw"), Ruled::Match("qw", ""));
        assert_eq!(r.rule("1.1"), Ruled::Match("1", ""));
        assert_eq!(r.rule("qw.1"), Ruled::Expected(SomeOf::Str("qw")));
        assert_eq!(r.rule("."), Ruled::Expected(SomeOf::Char('1')));
    }
}
