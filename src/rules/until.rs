use crate::{
    apply::Apply,
    ruled::Ruled,
    concat::Concat,
};

#[derive(Debug)]
pub struct Until<T, R, U>(pub std::marker::PhantomData<*const T>, pub R, pub U);

impl<T, R, U> Until<T, R, U> {
    pub fn new(rule: R, until: U) -> Self { Until(std::marker::PhantomData, rule, until) }
}

impl<T, R, U> Clone for Until<T, R, U>
    where
        R: Clone,
        U: Clone,
{
    fn clone(&self) -> Self { Until::new(self.1.clone(), self.2.clone()) }
}

impl<T, R, U> Copy for Until<T, R, U>
    where
        R: Copy,
        U: Copy,
{}

impl<I, T, R, U> Apply<I> for Until<T, R, U>
    where
        R: Apply<I> + Copy,
        U: Apply<I> + Copy,
        I: Copy,
        T: Concat<T, R::Res>,
{
    type Err = R::Err;
    type Res = (T, U::Res);

    fn apply(self, mut input: I) -> Ruled<I, Self::Res, Self::Err> {
        let mut res = T::empty();

        loop {
            match self.2.apply(input) {
                Ruled::Match(u, i) => break Ruled::Match((res, u), i),
                Ruled::Expected(_) => {
                    match self.1.apply(input) {
                        Ruled::Match(r, i) => {
                            input = i;
                            res = T::concat(res, r);
                        }
                        Ruled::Expected(e) => break Ruled::Expected(e),
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        apply::apply,
        rule::rule,
        rules::char_range::char_range,
        expected::Expected,
    };

    #[test]
    fn until() {
        let r = char_range('0'..='9').until("12");
        assert_eq!(apply(r, "110211234"), Ruled::Match(("11021".to_owned(), "12"), "34"));

        let r = rule('.').until("!");
        assert_eq!(apply(r, "...!!"), Ruled::Match(("...".to_owned(), "!"), "!"));
        assert_eq!(apply(r, "..."), Ruled::Expected(Expected::Char('.')));
    }
}
