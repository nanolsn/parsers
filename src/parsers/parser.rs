use crate::{Parse, Repeat, Second, OrParser};
use crate::maps::{Map, MapErr};

#[derive(Copy, Clone, Debug)]
pub struct Parser<P>(pub(crate) P);

impl<P> Parser<P> {
    pub fn and_then<U>(self, parser: U) -> Parser<Second<P, U>> {
        Parser(Second(self.0, parser))
    }

    pub fn or_else<U>(self, parser: U) -> Parser<OrParser<P, U>> {
        Parser(OrParser(self.0, parser))
    }

    pub fn repeat(self, times: usize) -> Parser<Repeat<P>> {
        Parser(Repeat(self.0, times))
    }

    pub fn map<F>(self, f: F) -> Parser<Map<P, F>> {
        Parser(Map(self.0, f))
    }

    pub fn map_err<F>(self, f: F) -> Parser<MapErr<P, F>> {
        Parser(MapErr(self.0, f))
    }
}

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

pub fn par<P, I>(parse: P) -> Parser<P>
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
        let p = super::par("a");

        assert_eq!(p.parse("a b"), Ok(("a", " b")));
        assert_eq!(p.parse("b"), Err(()));
    }
}
