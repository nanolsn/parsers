use crate::{
    rule::Rule,
    ruled::Ruled,
    rul::Rul,
};

#[derive(Copy, Clone, Debug)]
pub struct OneOf<'a, A>(pub &'a [A]);

pub fn one_of<A>(rules: &[A]) -> Rul<OneOf<A>> { Rul(OneOf(rules)) }

impl<'a, I, A> Rule<I> for OneOf<'a, A>
    where
        A: Rule<I> + Copy,
        I: Copy,
{
    type Exp = ();
    type Mat = A::Mat;

    fn rule(self, input: I) -> Ruled<I, Self::Res, Self::Err> {
        for rule in self.0 {
            if let Ruled::Match(r, i) = rule.rule(input) {
                return Ruled::Match(r, i);
            }
        }

        Ruled::Expected(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_of() {
        let r = super::one_of(&[
            "hi",
            "fi",
            "sci",
        ]);

        assert_eq!(r.rule("hi"), Ruled::Match("hi", ""));
        assert_eq!(r.rule("fi"), Ruled::Match("fi", ""));
        assert_eq!(r.rule("sci"), Ruled::Match("sci", ""));
        assert_eq!(r.rule("lo"), Ruled::Expected(()));
    }
}
