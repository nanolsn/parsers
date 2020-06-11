use crate::{
    rule::Rule,
    ruled::Ruled,
};

#[derive(Debug)]
pub struct Into<T, R>(pub std::marker::PhantomData<*const T>, pub R);

impl<T, R> Into<T, R> {
    pub fn new(rule: R) -> Self { Into(std::marker::PhantomData, rule) }
}

impl<T, R> Clone for Into<T, R>
    where
        R: Clone,
{
    fn clone(&self) -> Self { Into::new(self.1.clone()) }
}

impl<T, R> Copy for Into<T, R>
    where
        R: Copy,
{}

impl<I, T, R> Rule<I> for Into<T, R>
    where
        R: Rule<I>,
        R::Mat: std::convert::Into<T>,
{
    type Exp = R::Exp;
    type Mat = T;

    fn rule(self, input: I) -> Ruled<I, Self::Res, Self::Err> {
        use std::convert::Into;

        self.1.rule(input).map(|r| r.into())
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
    fn into() {
        let r = rul('@').into::<String>();
        assert_eq!(r.rule("@"), Ruled::Match("@".to_owned(), ""));
        assert_eq!(r.rule("!"), Ruled::Expected(SomeOf::Char('@')));
    }
}
