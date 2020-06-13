use crate::prelude::*;

#[derive(Debug)]
pub struct To<R, T>(R, std::marker::PhantomData<T>);

impl<R, T> To<R, T> {
    pub fn new(rule: R) -> Self { To(rule, std::marker::PhantomData) }
}

impl<R, T> Clone for To<R, T>
    where
        R: Clone,
{
    fn clone(&self) -> Self { To::new(self.0.clone()) }
}

impl<R, T> Copy for To<R, T>
    where
        R: Copy,
{}

impl<'r, I, R, T> Rule<'r, I> for To<R, T>
    where
        R: Rule<'r, I>,
        R::Mat: std::convert::Into<T>,
{
    type Mat = T;
    type Exp = R::Exp;

    fn rule(&'r self, input: I) -> Ruled<I, Self::Mat, Self::Exp> {
        self.0.rule(input).map(|r| r.into())
    }
}

impl_ops!(To<R, U>);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn into() {
        let r = '@'.to::<String>();
        assert_eq!(r.rule("@"), Match("@".to_owned(), ""));
        assert_eq!(r.rule("!"), Expected(Failed::Char('@')));
    }
}
