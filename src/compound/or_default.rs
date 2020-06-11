use crate::{
    rule::Rule,
    ruled::Ruled,
};

#[derive(Copy, Clone, Debug)]
pub struct OrDefault<R>(pub R);

impl<I, R> Rule<I> for OrDefault<R>
    where
        R: Rule<I>,
        I: Copy,
        R::Mat: Default,
{
    type Exp = R::Exp;
    type Mat = R::Mat;

    fn rule(self, input: I) -> Ruled<I, Self::Res, Self::Err> {
        match self.0.rule(input) {
            o @ Ruled::Match(_, _) => o,
            Ruled::Expected(_) => Ruled::Match(Default::default(), input),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rul::rul;

    #[test]
    fn or_default() {
        let r = rul("hello").or_default();
        assert_eq!(r.rule("hello"), Ruled::Match("hello", ""));
        assert_eq!(r.rule("hi"), Ruled::Match("", "hi"));
    }
}
