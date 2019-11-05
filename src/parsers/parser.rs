use crate::Parse;

#[derive(Copy, Clone, Debug)]
pub struct Parser<P>(pub(crate) P);

impl<P, I> Parse<I> for Parser<P>
    where
        P: Parse<I>,
{
    type Err = P::Err;
    type Out = P::Out;

    fn parse(&self, input: I) -> Result<(Self::Out, I), Self::Err> {
        self.0.parse(input)
    }
}

pub fn parser<P, I>(parse: P) -> Parser<P>
    where
        P: Parse<I>,
{
    Parser(parse)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser() {
        let p = super::parser("a");

        assert_eq!(p.parse("a b"), Ok(("a", " b")));
        assert_eq!(p.parse("b"), Err(()));
    }
}
