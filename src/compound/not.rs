use crate::{
    rule::Rule,
    ruled::Ruled,
};

#[derive(Copy, Clone, Debug)]
pub struct Not<A>(pub A);

impl<I, A> Rule<I> for Not<A>
    where
        A: Rule<I>,
        I: Copy,
{
    type Exp = A::Mat;
    type Mat = A::Exp;

    fn rule(self, input: I) -> Ruled<I, Self::Res, Self::Err> {
        match self.0.rule(input) {
            Ruled::Match(r, _) => Ruled::Expected(r),
            Ruled::Expected(e) => Ruled::Match(e, input),
        }
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
    fn not() {
        let r = !rul('a');
        assert_eq!(r.rule("a"), Ruled::Expected("a"));
        assert_eq!(r.rule("b"), Ruled::Match(SomeOf::Char('a'), "b"));
    }
}
