use crate::{
    rule::Rule,
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

impl<I, T, R, U> Rule<I> for Until<T, R, U>
    where
        R: Rule<I> + Copy,
        U: Rule<I> + Copy,
        I: Copy,
        T: Concat<T, R::Mat>,
{
    type Exp = R::Exp;
    type Mat = (T, U::Mat);

    fn rule(self, mut input: I) -> Ruled<I, Self::Res, Self::Err> {
        let mut res = T::empty();

        loop {
            match self.2.rule(input) {
                Ruled::Match(u, i) => break Ruled::Match((res, u), i),
                Ruled::Expected(_) => {
                    match self.1.rule(input) {
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
        rul::rul,
        compound::char_range::char_range,
        SomeOf,
    };

    #[test]
    fn until() {
        let r = char_range('0'..='9').until("12");
        assert_eq!(r.rule("110211234"), Ruled::Match(("11021".to_owned(), "12"), "34"));

        let r = rul('.').until("!");
        assert_eq!(r.rule("...!!"), Ruled::Match(("...".to_owned(), "!"), "!"));
        assert_eq!(r.rule("..."), Ruled::Expected(SomeOf::Char('.')));
    }
}
