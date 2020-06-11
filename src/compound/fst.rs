use crate::{
    rule::Rule,
    ruled::Ruled,
};

#[derive(Copy, Clone, Debug)]
pub struct Fst<A, B>(pub A, pub B);

impl<A, B, I> Rule<I> for Fst<A, B>
    where
        A: Rule<I>,
        B: Rule<I>,
        B::Exp: Into<A::Exp>,
{
    type Exp = A::Exp;
    type Mat = A::Mat;

    fn rule(self, input: I) -> Ruled<I, Self::Res, Self::Err> {
        let Fst(a, b) = self;

        a.rule(input)
            .and_then(|r, i| b
                .rule(i)
                .map(|_| r)
                .map_exp(|e| e.into())
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        rul::rul,
        some_of::SomeOf,
    };

    #[test]
    fn fst() {
        let r = rul('0') << '1';
        assert_eq!(r.rule("01."), Ruled::Match("0", "."));
        assert_eq!(r.rule("0!."), Ruled::Expected(SomeOf::Char('1')));
        assert_eq!(r.rule("!1."), Ruled::Expected(SomeOf::Char('0')));

        let r = rul('q') << 'w' << " " << "e";
        assert_eq!(r.rule("qw er"), Ruled::Match("q", "r"));
        assert_eq!(r.rule("qw e"), Ruled::Match("q", ""));
        assert_eq!(r.rule("qw "), Ruled::Expected(SomeOf::Str("e")));
    }
}
