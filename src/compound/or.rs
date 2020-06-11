use crate::{
    Rule,
    Ruled,
    Concat,
    compound::Cat,
};

#[derive(Copy, Clone, Debug)]
pub struct Or<A, B>(pub A, pub B);

impl<'r, I: 'r, A, B> Rule<'r, I> for Or<A, B>
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

impl<A, B, T> std::ops::BitOr<T> for Or<A, B> {
    type Output = Or<Or<A, B>, T>;

    fn bitor(self, rhs: T) -> Self::Output { Or(self, rhs) }
}

impl<A, B, T> std::ops::BitAnd<T> for Or<A, B>
    where
        &'static str: Concat<A, B>,
{
    type Output = Cat<Or<A, B>, T, &'static str>;

    fn bitand(self, rhs: T) -> Self::Output { Cat::new(self, rhs) }
}

impl<A, B, T> std::ops::Add<T> for Or<A, B>
    where
        String: Concat<A, B>,
{
    type Output = Cat<Or<A, B>, T, String>;

    fn add(self, rhs: T) -> Self::Output { Cat::new(self, rhs) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Ruled::*;

    #[test]
    fn or() {
        let r = '@'.or('#');
        assert_eq!(r.rule("@"), Match("@", ""));
        assert_eq!(r.rule("#"), Match("#", ""));
        assert_eq!(r.rule("$"), Expected('#'));

        let r = "qwe".or("123").or("null");
        assert_eq!(r.rule("qwe"), Match("qwe", ""));
        assert_eq!(r.rule("1234"), Match("123", "4"));
        assert_eq!(r.rule("nullable"), Match("null", "able"));
        assert_eq!(r.rule("qw"), Expected("null"));
    }
}
