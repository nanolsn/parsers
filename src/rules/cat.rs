use crate::{
    apply::Apply,
    ruled::Ruled,
    concat::Concat,
};

#[derive(Debug)]
pub struct Cat<T, A, B>(pub std::marker::PhantomData<*const T>, pub A, pub B);

impl<T, A, B> Cat<T, A, B> {
    pub fn new(a: A, b: B) -> Self { Cat(std::marker::PhantomData, a, b) }
}

impl<T, A, B> Clone for Cat<T, A, B>
    where
        A: Clone,
        B: Clone,
{
    fn clone(&self) -> Self { Cat::new(self.1.clone(), self.2.clone()) }
}

impl<T, A, B> Copy for Cat<T, A, B>
    where
        A: Copy,
        B: Copy,
{}

impl<I, T, A, B> Apply<I> for Cat<T, A, B>
    where
        A: Apply<I>,
        B: Apply<I, Err=A::Err>,
        T: Concat<A::Res, B::Res>,
{
    type Err = A::Err;
    type Res = T;

    fn apply(self, input: I) -> Ruled<I, Self::Res, Self::Err> {
        let Cat(_, a, b) = self;

        a.apply(input)
            .and_then(|l, i| b.apply(i)
                .map(|r| T::concat(l, r))
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        apply::apply,
        rule::rule,
        expected::Expected,
    };

    #[test]
    fn cat() {
        let r = rule('@') & '#';
        assert_eq!(apply(r, "@#"), Ruled::Match("@#".to_owned(), ""));
        assert_eq!(apply(r, "@!"), Ruled::Expected(Expected::Char('#')));
        assert_eq!(apply(r, "@"), Ruled::Expected(Expected::Char('#')));

        let r = rule("q") & "w" & "e";
        assert_eq!(apply(r, "qwe"), Ruled::Match("qwe".to_owned(), ""));
        assert_eq!(apply(r, "qwe123"), Ruled::Match("qwe".to_owned(), "123"));
        assert_eq!(apply(r, "123"), Ruled::Expected(Expected::Str("q")));
    }

    #[test]
    fn concat() {
        let r = rule("q").cat("w").concat("e").concat('1');
        assert_eq!(apply(r, "qwe1"), Ruled::Match("qwe1".to_owned(), ""));

        let r = rule("q").cat("w").concat("e").concat('1');
        assert_eq!(apply(r, "qwe1"), Ruled::Match("qwe1", ""));

        let r = rule("q").map(|q| vec![q]).cat("w").concat("e").concat("1");
        assert_eq!(apply(r, "qwe1"), Ruled::Match(vec!["q", "w", "e", "1"], ""));
    }
}
