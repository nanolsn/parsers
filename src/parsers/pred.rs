use crate::{Parse, Parsed};

#[derive(Copy, Clone, Debug)]
pub struct Pred<P, F>(pub(crate) P, pub(crate) F);

impl<'p, P, F, A> Parse<'p> for Pred<P, F>
    where
        P: Parse<'p, Res=A>,
        F: Fn(&A) -> bool,
        A: 'p,
{
    type Res = P::Res;
    type Err = ();
    type On = P::On;

    fn parse(&self, input: Self::On) -> Parsed<Self::Res, Self::Err, Self::On> {
        match self.0.parse(input) {
            Ok((out, rest)) => {
                if (self.1)(&out) {
                    Ok((out, rest))
                } else {
                    Err(())
                }
            }
            Err(_) => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::par;

    #[test]
    fn until() {
        let u = (par("@") | "#").pred(|s: &&str| *s == "@");

        assert_eq!(u.parse("@"), Ok(("@", "")));
        assert_eq!(u.parse("#"), Err(()));
    }
}
