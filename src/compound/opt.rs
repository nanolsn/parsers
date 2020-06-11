use crate::{
    rule::Rule,
    ruled::Ruled,
};

#[derive(Copy, Clone, Debug)]
pub struct Opt<R>(pub R);

impl<I, R> Rule<I> for Opt<R>
    where
        R: Rule<I>,
        I: Copy,
{
    type Exp = R::Exp;
    type Mat = Option<R::Mat>;

    fn rule(self, input: I) -> Ruled<I, Self::Res, Self::Err> {
        match self.0.rule(input) {
            Ruled::Match(r, i) => Ruled::Match(Some(r), i),
            Ruled::Expected(_) => Ruled::Match(None, input),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rul::rul;

    #[test]
    fn opt() {
        let r = rul("q").opt();
        assert_eq!(r.rule("qw"), Ruled::Match(Some("q"), "w"));
        assert_eq!(r.rule("w"), Ruled::Match(None, "w"));
    }
}
