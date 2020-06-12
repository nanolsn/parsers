use crate::prelude::*;

#[derive(Copy, Clone, Debug)]
pub struct Snd<A, B>(pub A, pub B);

impl<'r, I: 'r, A, B> Rule<'r, I> for Snd<A, B>
    where
        A: Rule<'r, I>,
        B: Rule<'r, I>,
        A::Exp: Into<B::Exp>,
{
    type Mat = B::Mat;
    type Exp = B::Exp;

    fn rule(&'r self, input: I) -> Ruled<I, Self::Mat, Self::Exp> {
        self.0.rule(input)
            .map_exp(|e| e.into())
            .and_then(|_, i| self.1.rule(i))
    }
}

impl_ops!(Snd<A, B>);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snd() {
        let r = '0'.snd('1');
        assert_eq!(r.rule("01."), Match("1", "."));
        assert_eq!(r.rule("0!."), Expected(Failed::Char('1')));
        assert_eq!(r.rule("!1."), Expected(Failed::Char('0')));

        let r = rul('q') >> 'w' >> " " >> "e";
        assert_eq!(r.rule("qw er"), Match("e", "r"));
        assert_eq!(r.rule("qw e"), Match("e", ""));
        assert_eq!(r.rule("qw "), Expected(Failed::Str("e")));
    }
}
