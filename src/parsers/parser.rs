use crate::{Parse, Repeat, Second, OrParser, Until, RangeVec, UntilVec, Pred, Opt, Boxed};
use crate::maps::{Map, MapErr};
use crate::parsers::range::Range;
use std::ops::Deref;

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

    pub fn range(self, from: usize, to: usize) -> Parser<Range<P>> {
        Parser(Range {
            parser: self.0,
            from,
            to: Some(to),
        })
    }

    pub fn n_or_more(self, n: usize) -> Parser<Range<P>> {
        Parser(Range {
            parser: self.0,
            from: n,
            to: None,
        })
    }

    pub fn range_vec(self, from: usize, to: usize) -> Parser<RangeVec<P>> {
        Parser(RangeVec {
            parser: self.0,
            from,
            to: Some(to),
        })
    }

    pub fn n_or_more_vec(self, n: usize) -> Parser<RangeVec<P>> {
        Parser(RangeVec {
            parser: self.0,
            from: n,
            to: None,
        })
    }

    pub fn until<U>(self, parser: U) -> Parser<Until<P, U>> {
        Parser(Until(self.0, parser))
    }

    pub fn until_vec<U>(self, parser: U) -> Parser<UntilVec<P, U>> {
        Parser(UntilVec(self.0, parser))
    }

    pub fn pred<F, A>(self, f: F) -> Parser<Pred<P, F>>
        where
            F: Fn(&A) -> bool,
    {
        Parser(Pred(self.0, f))
    }

    pub fn opt(self) -> Parser<Opt<P>> {
        Parser(Opt(self.0))
    }

    pub fn boxed<I>(self) -> Parser<Boxed<I, P::Out, P::Err>>
        where
            P: Parse<I> + 'static,
    {
        Parser(Boxed(Box::new(self.0)))
    }

    pub fn map<F, A, B>(self, f: F) -> Parser<Map<P, F>>
        where
            F: Fn(A) -> B,
    {
        Parser(Map(self.0, f))
    }

    pub fn map_err<F, E, G>(self, f: F) -> Parser<MapErr<P, F>>
        where
            F: Fn(E) -> G,
    {
        Parser(MapErr(self.0, f))
    }
}

impl<'i, P> Parser<P>
    where
        P: Parse<&'i str>,
{
    pub fn into_stringed_par(self) -> Parser<StringedParser<P>> {
        stringed_par(self.0)
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

#[derive(Copy, Clone, Debug)]
pub struct StringedParser<P>(P);

impl<'i, P> Parse<&'i str> for StringedParser<P>
    where
        P: Parse<&'i str, Out=&'i str>,
{
    type Err = P::Err;
    type Out = String;

    fn parse(&self, input: &'i str) -> Result<(Self::Out, &'i str), Self::Err> {
        self.0.parse(input).map(|(out, rest)| (out.to_string(), rest))
    }
}

pub fn stringed_par<'i, P>(parse: P) -> Parser<StringedParser<P>>
    where
        P: Parse<&'i str>,
{
    Parser(StringedParser(parse))
}

impl<P> Deref for Parser<P> {
    type Target = P;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
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

    #[test]
    fn parser_map_to_string() {
        let p = super::par("a").into_stringed_par();

        assert_eq!(p.parse("a b"), Ok(("a".to_string(), " b")));
        assert_eq!(p.parse("b"), Err(()));

        let p = super::stringed_par("a");

        assert_eq!(p.parse("a b"), Ok(("a".to_string(), " b")));
        assert_eq!(p.parse("b"), Err(()));
    }
}
