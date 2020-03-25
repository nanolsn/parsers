use crate::{
    apply::Apply,
    ruled::Ruled,
    concat::Concat,
};

#[derive(Debug)]
pub struct Cat<A, B, C>(pub A, pub B, pub std::marker::PhantomData<*const C>);

impl<A, B, C> Clone for Cat<A, B, C>
    where
        A: Clone,
        B: Clone,
{
    fn clone(&self) -> Self {
        Cat(self.0.clone(), self.1.clone(), std::marker::PhantomData)
    }
}

impl<A, B, C> Copy for Cat<A, B, C>
    where
        A: Copy,
        B: Copy,
{}

impl<I, A, B, C> Apply<I> for Cat<A, B, C>
    where
        A: Apply<I>,
        B: Apply<I, Err=A::Err>,
        C: Concat<A::Res, B::Res>,
{
    type Err = A::Err;
    type Res = C;

    fn apply(&self, input: I) -> Ruled<I, Self::Res, Self::Err> {
        self.0.apply(input)
            .and_then(|l, i| self.1.apply(i)
                .map(|r| C::concat(l, r))
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

        let r = rule('@').map(|s| vec![s]).cat('#');
        assert_eq!(apply(r, "@#"), Ruled::Ok(vec!["@", "#"], ""));
    }
}
