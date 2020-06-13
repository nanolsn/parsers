use crate::prelude::*;

#[derive(Copy, Clone, Debug)]
pub struct Fst<A, B>(pub A, pub B);

impl<'r, I, A, B> Rule<'r, I> for Fst<A, B>
    where
        A: Rule<'r, I>,
        B: Rule<'r, I>,
        B::Exp: Into<A::Exp>,
{
    type Mat = A::Mat;
    type Exp = A::Exp;

    fn rule(&'r self, input: I) -> Ruled<I, Self::Mat, Self::Exp> {
        self.0.rule(input)
            .and_then(|r, i| self.1.rule(i)
                .map(|_| r)
                .map_exp(|e| e.into())
            )
    }
}

impl_ops!(Fst<A, B>);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fst() {
        let r = '0'.fst('1');
        assert_eq!(r.rule("01."), Match("0", "."));
        assert_eq!(r.rule("0!."), Expected(Failed::Char('1')));
        assert_eq!(r.rule("!1."), Expected(Failed::Char('0')));

        let r = rul('q') << 'w' << " " << "e";
        assert_eq!(r.rule("qw er"), Match("q", "r"));
        assert_eq!(r.rule("qw e"), Match("q", ""));
        assert_eq!(r.rule("qw "), Expected(Failed::Str("e")));
    }
}
