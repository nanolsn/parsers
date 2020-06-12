use crate::prelude::*;

#[derive(Copy, Clone, Debug)]
pub struct OrDefault<R>(pub R);

impl<'r, I: 'r, R> Rule<'r, I> for OrDefault<R>
    where
        R: Rule<'r, I>,
        I: Copy,
        R::Mat: Default,
{
    type Mat = R::Mat;
    type Exp = R::Exp;

    fn rule(&'r self, input: I) -> Ruled<I, Self::Mat, Self::Exp> {
        match self.0.rule(input) {
            o @ Ruled::Match(_, _) => o,
            Ruled::Expected(_) => Ruled::Match(Default::default(), input),
        }
    }
}

impl_ops!(OrDefault<R>);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn or_default() {
        let r = "hello".or_default();
        assert_eq!(r.rule("hello"), Match("hello", ""));
        assert_eq!(r.rule("hi"), Match("", "hi"));
    }
}
