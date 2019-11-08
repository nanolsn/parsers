use crate::Parse;

#[derive(Copy, Clone, Debug)]
pub struct Opt<P>(pub(crate) P);

impl<'i, P> Parse<&'i str> for Opt<P>
    where
        P: Parse<&'i str, Out=&'i str>,
{
    type Err = P::Err;
    type Out = P::Out;

    fn parse(&self, input: &'i str) -> Result<(Self::Out, &'i str), Self::Err> {
        match self.0.parse(input) {
            o @ Ok(_) => o,
            Err(_) => Ok(("", input)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{par, stringed_par};

    #[test]
    fn opt() {
        let p = stringed_par("hello") & par('!').opt();

        assert_eq!(p.parse("hello."), Ok(("hello".to_string(), ".")));
        assert_eq!(p.parse("hello!"), Ok(("hello!".to_string(), "")));
    }
}
