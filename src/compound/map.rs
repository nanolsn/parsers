use crate::{
    rule::Rule,
    ruled::Ruled,
};

#[derive(Copy, Clone, Debug)]
pub struct Map<R, F>(pub R, pub F);

impl<I, R, F, K> Rule<I> for Map<R, F>
    where
        R: Rule<I>,
        F: FnOnce(R::Mat) -> K,
{
    type Exp = R::Exp;
    type Mat = K;

    fn rule(self, input: I) -> Ruled<I, Self::Res, Self::Err> {
        let Map(p, f) = self;

        p.rule(input)
            .map(|r| f(r))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        rul::rul,
        some_of::SomeOf,
    };

    //noinspection RsBorrowChecker
    #[test]
    fn map() {
        use std::str::FromStr;

        let r = (rul('1') | '2')
            .map(|s: &str| i32::from_str(s).unwrap());

        assert_eq!(r.rule("1"), Ruled::Match(1, ""));
        assert_eq!(r.rule("2"), Ruled::Match(2, ""));
        assert_eq!(r.rule("3"), Ruled::Expected(SomeOf::Char('2')));
    }
}
