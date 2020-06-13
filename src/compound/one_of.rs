use crate::prelude::*;

#[derive(Copy, Clone, Debug)]
pub struct OneOf<'a, A>(pub &'a [A]);

pub fn one_of<A>(rules: &[A]) -> OneOf<A> { OneOf(rules) }

impl<'r, 'a, I, A> Rule<'r, I> for OneOf<'a, A>
    where
        A: Rule<'r, I>,
        I: Copy,
{
    type Mat = A::Mat;
    type Exp = ();

    fn rule(&'r self, input: I) -> Ruled<I, Self::Mat, Self::Exp> {
        for rule in self.0 {
            if let Match(r, i) = rule.rule(input) {
                return Match(r, i);
            }
        }

        Expected(())
    }
}

impl_ops!(OneOf<'a, A>);

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

        assert_eq!(r.rule("hi"), Match("hi", ""));
        assert_eq!(r.rule("fi"), Match("fi", ""));
        assert_eq!(r.rule("sci"), Match("sci", ""));
        assert_eq!(r.rule("lo"), Expected(()));
    }
}
