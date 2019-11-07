use crate::Parse;

#[derive(Copy, Clone, Debug)]
pub struct Until<P, U>(pub(crate) P, pub(crate) U);

impl<P, U, I> Parse<I> for Until<P, U>
    where
        P: Parse<I>,
        U: Parse<I>,
        I: Copy,
{
    type Err = P::Err;
    type Out = U::Out;

    fn parse(&self, mut rest: I) -> Result<(Self::Out, I), Self::Err> {
        loop {
            match self.1.parse(rest) {
                Ok(res) => break Ok(res),
                Err(_) => {
                    match self.0.parse(rest) {
                        Ok((_, r)) => rest = r,
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
        let u = par(ANY).until("%^");

        assert_eq!(u.parse("@#_%_$%^&"), Ok(("%^".to_string(), "&")));

        let u = par(ANY).until("!");

        assert_eq!(u.parse("..."), Err(()));
    }
}
