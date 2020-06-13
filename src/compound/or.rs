use crate::prelude::*;

#[derive(Copy, Clone, Debug)]
pub struct Or<A, B>(pub A, pub B);

impl<'r, I, A, B> Rule<'r, I> for Or<A, B>
    where
        A: Rule<'r, I>,
        B: Rule<'r, I, Mat=A::Mat, Exp=A::Exp>,
        I: Copy,
{
    type Mat = B::Mat;
    type Exp = A::Exp;

    fn rule(&'r self, input: I) -> Ruled<I, Self::Mat, Self::Exp> {
        self.0.rule(input)
            .map(|l| l.into())
            .or_else(|_| self.1.rule(input))
    }
}

impl_ops!(Or<A, B>);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn or() {
        let r = '@'.or('#');
        assert_eq!(r.rule("@"), Match("@", ""));
        assert_eq!(r.rule("#"), Match("#", ""));
        assert_eq!(r.rule("$"), Expected(Failed::Char('#')));

        let r = "qwe".or("123").or("null");
        assert_eq!(r.rule("qwe"), Match("qwe", ""));
        assert_eq!(r.rule("1234"), Match("123", "4"));
        assert_eq!(r.rule("nullable"), Match("null", "able"));
        assert_eq!(r.rule("qw"), Expected(Failed::Str("null")));
    }
}
