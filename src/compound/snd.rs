use crate::{
    rule::Rule,
    ruled::Ruled,
};

#[derive(Copy, Clone, Debug)]
pub struct Snd<A, B>(pub A, pub B);

impl<A, B, I> Rule<I> for Snd<A, B>
    where
        A: Rule<I>,
        B: Rule<I>,
        A::Exp: Into<B::Exp>,
{
    type Exp = B::Exp;
    type Mat = B::Mat;

    fn rule(self, input: I) -> Ruled<I, Self::Res, Self::Err> {
        let Snd(a, b) = self;

        a.rule(input)
            .map_exp(|e| e.into())
            .and_then(|_, i| b.rule(i))
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
    fn snd() {
        let r = rul('0') >> '1';
        assert_eq!(r.rule("01."), Ruled::Match("1", "."));
        assert_eq!(r.rule("0!."), Ruled::Expected(SomeOf::Char('1')));
        assert_eq!(r.rule("!1."), Ruled::Expected(SomeOf::Char('0')));

        let r = rul('q') >> 'w' >> " " >> "e";
        assert_eq!(r.rule("qw er"), Ruled::Match("e", "r"));
        assert_eq!(r.rule("qw e"), Ruled::Match("e", ""));
        assert_eq!(r.rule("qw "), Ruled::Expected(SomeOf::Str("e")));
    }
}
