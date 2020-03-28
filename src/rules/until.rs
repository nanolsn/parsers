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
        R: Apply<I>,
        U: Apply<I>,
        I: Copy,
        T: Concat<T, R::Res>,
{
    type Err = R::Err;
    type Res = (T, U::Res);

    fn apply(&self, mut input: I) -> Ruled<I, Self::Res, Self::Err> {
        let mut res = T::empty();

        loop {
            match self.2.apply(input) {
                Ruled::Ok(u, i) => break Ruled::Ok((res, u), i),
                Ruled::Err(_) => {
                    match self.1.apply(input) {
                        Ruled::Ok(r, i) => {
                            input = i;
                            res = T::concat(res, r);
                        }
                        Ruled::Err(e) => break Ruled::Err(e),
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
    };

    #[test]
    fn until() {
        let r = char_range('0'..='9').until("12");
        assert_eq!(apply(&r, "110211234"), Ruled::Ok(("11021".to_owned(), "12"), "34"));

        let r = rule('.').until("!");
        assert_eq!(apply(&r, "...!!"), Ruled::Ok(("...".to_owned(), "!"), "!"));
        assert_eq!(apply(&r, "..."), Ruled::Err(()));
    }
}
