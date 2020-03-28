use crate::{
    apply::Apply,
    ruled::Ruled,
};

#[derive(Copy, Clone, Debug)]
pub struct Snd<A, B>(pub A, pub B);

impl<A, B, I> Apply<I> for Snd<A, B>
    where
        A: Apply<I>,
        B: Apply<I, Err=A::Err>,
{
    type Err = A::Err;
    type Res = B::Res;

    fn apply(&self, input: I) -> Ruled<I, Self::Res, Self::Err> {
        self.0.apply(input)
            .and_then(|_, i| self.1.apply(i))
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
    fn snd() {
        let r = rule('0') >> '1';
        assert_eq!(apply(&r, "01."), Ruled::Ok("1", "."));
        assert_eq!(apply(&r, "0!."), Ruled::Err(()));
        assert_eq!(apply(&r, "!1."), Ruled::Err(()));

        let r = rule('q') >> 'w' >> " " >> "e";
        assert_eq!(apply(&r, "qw er"), Ruled::Ok("e", "r"));
        assert_eq!(apply(&r, "qw e"), Ruled::Ok("e", ""));
        assert_eq!(apply(&r, "qw "), Ruled::Err(()));
    }
}
