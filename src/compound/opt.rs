use crate::prelude::*;

#[derive(Copy, Clone, Debug)]
pub struct Opt<R>(pub R);

impl<'r, I: 'r, R> Rule<'r, I> for Opt<R>
    where
        R: Rule<'r, I>,
        I: Copy,
{
    type Mat = Option<R::Mat>;
    type Exp = R::Exp;

    fn rule(&'r self, input: I) -> Ruled<I, Self::Mat, Self::Exp> {
        match self.0.rule(input) {
            Match(r, i) => Match(Some(r), i),
            Expected(_) => Match(None, input),
        }
    }
}

impl_ops!(Opt<R>);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opt() {
        let r = "q".opt();
        assert_eq!(r.rule("q"), Match(Some("q"), ""));
        assert_eq!(r.rule("w"), Match(None, "w"));
    }
}
