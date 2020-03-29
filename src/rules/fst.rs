use crate::{
    apply::Apply,
    ruled::Ruled,
};

#[derive(Copy, Clone, Debug)]
pub struct Fst<A, B>(pub A, pub B);

impl<A, B, I> Apply<I> for Fst<A, B>
    where
        A: Apply<I>,
        B: Apply<I, Err=A::Err>,
{
    type Err = A::Err;
    type Res = A::Res;

    fn apply(self, input: I) -> Ruled<I, Self::Res, Self::Err> {
        let Fst(a, b) = self;

        a.apply(input)
            .and_then(|r, i| b.apply(i).map(|_| r))
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
    fn fst() {
        let r = rule('0') << '1';
        assert_eq!(apply(r, "01."), Ruled::Ok("0", "."));
        assert_eq!(apply(r, "0!."), Ruled::Err(()));
        assert_eq!(apply(r, "!1."), Ruled::Err(()));

        let r = rule('q') << 'w' << " " << "e";
        assert_eq!(apply(r, "qw er"), Ruled::Ok("q", "r"));
        assert_eq!(apply(r, "qw e"), Ruled::Ok("q", ""));
        assert_eq!(apply(r, "qw "), Ruled::Err(()));
    }
}
