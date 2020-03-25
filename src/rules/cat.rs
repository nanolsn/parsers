use crate::{
    apply::Apply,
    ruled::Ruled,
    concat::Concat,
};

#[derive(Copy, Clone, Debug)]
pub struct Cat<A, B>(pub A, pub B);

impl<I, A, B> Apply<I> for Cat<A, B>
    where
        A: Apply<I>,
        B: Apply<I, Err=A::Err>,
        A::Res: Concat<B::Res>,
{
    type Err = A::Err;
    type Res = <A::Res as Concat<B::Res>>::Res;

    fn apply(&self, input: I) -> Ruled<I, Self::Res, Self::Err> {
        self.0.apply(input)
            .and_then(|l, i| self.1.apply(i)
                .map(|r| l.concat(r))
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
