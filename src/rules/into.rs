use crate::{
    apply::Apply,
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

impl<I, T, R> Apply<I> for Into<T, R>
    where
        R: Apply<I>,
        R::Res: std::convert::Into<T>,
{
    type Err = R::Err;
    type Res = T;

    fn apply(&self, input: I) -> Ruled<I, Self::Res, Self::Err> {
        use std::convert::Into;

        self.1.apply(input).map(|r| r.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        apply::apply,
        rule::rule,
    };

    #[test]
    fn into() {
        let r = rule('@').into::<String>();
        assert_eq!(apply(&r, "@"), Ruled::Ok("@".to_owned(), ""));
        assert_eq!(apply(&r, "!"), Ruled::Err(()));
    }
}
