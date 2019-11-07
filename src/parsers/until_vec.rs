use crate::Parse;

#[derive(Copy, Clone, Debug)]
pub struct UntilVec<P, U>(pub(crate) P, pub(crate) U);

impl<'i, P, U, I> Parse<I> for UntilVec<P, U>
    where
        P: Parse<I>,
        U: Parse<I>,
        I: Copy,
{
    type Err = P::Err;
    type Out = (Vec<P::Out>, U::Out);

    fn parse(&self, mut rest: I) -> Result<(Self::Out, I), Self::Err> {
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
