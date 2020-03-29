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
    };

    #[test]
    fn cat() {
        let r = rule('@') & '#';
        assert_eq!(apply(r, "@#"), Ruled::Ok("@#".to_owned(), ""));
        assert_eq!(apply(r, "@!"), Ruled::Err(()));
        assert_eq!(apply(r, "@"), Ruled::Err(()));

        let r = rule("q") & "w" & "e";
        assert_eq!(apply(r, "qwe"), Ruled::Ok("qwe".to_owned(), ""));
        assert_eq!(apply(r, "qwe123"), Ruled::Ok("qwe".to_owned(), "123"));
        assert_eq!(apply(r, "123"), Ruled::Err(()));
    }
}
