use crate::Parse;

#[derive(Copy, Clone, Debug)]
pub struct Until<P, U>(pub(crate) P, pub(crate) U);

impl<'i, P, U> Parse<&'i str> for Until<P, U>
    where
        P: Parse<&'i str, Out=String>,
        U: Parse<&'i str>,
{
    type Err = P::Err;
    type Out = (String, U::Out);

    fn parse(&self, mut rest: &'i str) -> Result<(Self::Out, &'i str), Self::Err> {
        let mut s = String::new();

        loop {
            match self.1.parse(rest) {
                Ok((u, rest)) => break Ok(((s, u), rest)),
                Err(_) => {
                    match self.0.parse(rest) {
                        Ok((out, r)) => {
                            rest = r;
                            s.push_str(&out);
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
        let u = par(ANY).until("%^");

        assert_eq!(u.parse("@#_%_$%^&"), Ok((("@#_%_$".to_string(), "%^".to_string()), "&")));

        let u = par(ANY).until("!");

        assert_eq!(u.parse("..."), Err(()));
    }
}
