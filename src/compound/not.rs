use crate::prelude::*;

#[derive(Copy, Clone, Debug)]
pub struct Not<A>(pub A);

impl<'r, I, A> Rule<'r, I> for Not<A>
    where
        A: Rule<'r, I>,
        I: Copy,
{
    type Mat = A::Exp;
    type Exp = A::Mat;

    fn rule(&'r self, input: I) -> Ruled<I, Self::Mat, Self::Exp> {
        match self.0.rule(input) {
            Match(r, _) => Expected(r),
            Expected(e) => Match(e, input),
        }
    }
}

impl_ops!(Not<A>);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn not() {
        let r = !rul('a');
        assert_eq!(r.rule("a"), Expected("a"));
        assert_eq!(r.rule("b"), Match(Failed::Char('a'), "b"));
    }
}
