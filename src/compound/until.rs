use crate::{
    prelude::*,
    Concat,
};

#[derive(Debug)]
pub struct Until<R, U, C>(R, U, std::marker::PhantomData<C>);

impl<R, U, C> Until<R, U, C> {
    pub fn new(rule: R, until: U) -> Self { Until(rule, until, std::marker::PhantomData) }
}

impl<R, U, C> Clone for Until<R, U, C>
    where
        R: Clone,
        U: Clone,
{
    fn clone(&self) -> Self { Until::new(self.0.clone(), self.1.clone()) }
}

impl<R, U, C> Copy for Until<R, U, C>
    where
        R: Copy,
        U: Copy,
{}

impl<'r, I, R, U, C> Rule<'r, I> for Until<R, U, C>
    where
        R: Rule<'r, I>,
        U: Rule<'r, I>,
        I: Copy,
        C: Concat<C, R::Mat>,
{
    type Mat = (C, U::Mat);
    type Exp = R::Exp;

    fn rule(&'r self, mut input: I) -> Ruled<I, Self::Mat, Self::Exp> {
        let mut res = C::empty();

        loop {
            match self.1.rule(input) {
                Match(u, i) => break Ruled::Match((res, u), i),
                Expected(_) => {
                    match self.0.rule(input) {
                        Match(r, i) => {
                            input = i;
                            res = C::concat(res, r);
                        }
                        Expected(e) => break Expected(e),
                    }
                }
            }
        }
    }
}

impl_or!(Until<R, U, C>);
impl_shifts!(Until<R, U, C>);
impl_not!(Until<R, U, C>);

impl<R, U, T> std::ops::BitAnd<T> for Until<R, U, &'static str> {
    type Output = super::Cat<Until<R, U, &'static str>, T, &'static str>;

    fn bitand(self, rhs: T) -> Self::Output { super::Cat::new(self, rhs) }
}

impl<R, U, T> std::ops::Add<T> for Until<R, U, String> {
    type Output = super::Cat<Until<R, U, String>, T, String>;

    fn add(self, rhs: T) -> Self::Output { super::Cat::new(self, rhs) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn until() {
        let r = char_range('0'..='9').until("12");
        assert_eq!(r.rule("110211234"), Match(("11021".to_owned(), "12"), "34"));

        let r = rul('.').until("!");
        assert_eq!(r.rule("...!!"), Match(("...".to_owned(), "!"), "!"));
        assert_eq!(r.rule("..."), Expected(Failed::Char('.')));
    }
}
