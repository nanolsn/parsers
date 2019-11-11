use crate::{Parse, Parsed};

#[derive(Copy, Clone, Debug)]
pub struct UntilVec<P, U>(pub(crate) P, pub(crate) U);

impl<'p, P, U> Parse<'p> for UntilVec<P, U>
    where
        P: Parse<'p>,
        U: Parse<'p, On=P::On>,
        P::On: Copy,
{
    type Res = (Vec<P::Res>, U::Res);
    type Err = P::Err;
    type On = P::On;

    fn parse(&self, mut rest: Self::On) -> Parsed<Self::Res, Self::Err, Self::On> {
        let mut v = Vec::new();

        loop {
            match self.1.parse(rest) {
                Ok((u, rest)) => break Ok(((v, u), rest)),
                Err(_) => {
                    match self.0.parse(rest) {
                        Ok((out, r)) => {
                            rest = r;
                            v.push(out);
                        }
                        Err(e) => break Err(e),
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ANY, par};

    #[test]
    fn until() {
        let u = par(ANY).until_vec("%^");

        assert_eq!(u.parse("@#_%_$%^&"), Ok(((vec!["@", "#", "_", "%", "_", "$"], "%^"), "&")));

        let u = par(ANY).until("!");

        assert_eq!(u.parse("..."), Err(()));
    }
}
