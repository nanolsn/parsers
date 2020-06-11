use crate::{
    apply::Apply,
    ruled::Ruled,
};

#[derive(Copy, Clone, Debug)]
pub struct Snd<A, B>(pub A, pub B);

impl<A, B, I> Apply<I> for Snd<A, B>
    where
        A: Apply<I>,
        B: Apply<I>,
        A::Err: Into<B::Err>,
{
    type Err = B::Err;
    type Res = B::Res;

    fn apply(self, input: I) -> Ruled<I, Self::Res, Self::Err> {
        let Snd(a, b) = self;

        a.apply(input)
            .map_err(|e| e.into())
            .and_then(|_, i| b.apply(i))
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
    fn snd() {
        let r = rule('0') >> '1';
        assert_eq!(apply(r, "01."), Ruled::Match("1", "."));
        assert_eq!(apply(r, "0!."), Ruled::Expected(Expected::Char('1')));
        assert_eq!(apply(r, "!1."), Ruled::Expected(Expected::Char('0')));

        let r = rule('q') >> 'w' >> " " >> "e";
        assert_eq!(apply(r, "qw er"), Ruled::Match("e", "r"));
        assert_eq!(apply(r, "qw e"), Ruled::Match("e", ""));
        assert_eq!(apply(r, "qw "), Ruled::Expected(Expected::Str("e")));
    }
}
