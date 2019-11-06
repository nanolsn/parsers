use crate::Parse;

#[derive(Copy, Clone, Debug)]
pub struct UntilConcat<P, U>(pub(crate) P, pub(crate) U);

impl<'i, P, U> Parse<&'i str> for UntilConcat<P, U>
    where
        P: Parse<&'i str, Out=&'i str>,
        U: Parse<&'i str>,
{
    type Err = P::Err;
    type Out = &'i str;

    fn parse(&self, input: &'i str) -> Result<(Self::Out, &'i str), Self::Err> {
        let mut rest = input;
        let mut len = 0;

        loop {
            match self.1.parse(rest) {
                Ok((_, r)) => break Ok((&input[..len], r)),
                Err(_) => {
                    match self.0.parse(rest) {
                        Ok((out, r)) => {
                            rest = r;
                            len += out.len();
                        },
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
    fn until_concat() {
        let u = par(ANY).until_concat("%^");

        assert_eq!(u.parse("@#_%_$%^&"), Ok(("@#_%_$", "&")));

        let u = par(ANY).until_concat("!");

        assert_eq!(u.parse("..."), Err(()));
    }
}
