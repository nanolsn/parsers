use crate::{
    apply::Apply,
    ruled::Ruled,
    rule::Rule,
};

#[derive(Copy, Clone, Debug)]
pub struct OneOf<'a, A>(pub &'a [A]);

pub fn one_of<A>(rules: &[A]) -> Rule<OneOf<A>> { Rule(OneOf(rules)) }

impl<'a, I, A> Apply<I> for OneOf<'a, A>
    where
        A: Apply<I> + Copy,
        I: Copy,
{
    type Err = ();
    type Res = A::Res;

    fn apply(self, input: I) -> Ruled<I, Self::Res, Self::Err> {
        for rule in self.0 {
            if let Ruled::Match(r, i) = rule.apply(input) {
                return Ruled::Match(r, i);
            }
        }

        Ruled::Expected(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::apply::apply;

    #[test]
    fn one_of() {
        let r = super::one_of(&[
            "hi",
            "fi",
            "sci",
        ]);

        assert_eq!(apply(r, "hi"), Ruled::Match("hi", ""));
        assert_eq!(apply(r, "fi"), Ruled::Match("fi", ""));
        assert_eq!(apply(r, "sci"), Ruled::Match("sci", ""));
        assert_eq!(apply(r, "lo"), Ruled::Expected(()));
    }
}
