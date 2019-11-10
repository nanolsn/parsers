use crate::{Parse, Repeat, Until, RangeVec, UntilVec, Pred, Opt, Boxed, PredFn};
use crate::maps::{Map, MapErr};
use crate::parsers::range::Range;
use std::ops::Deref;

#[derive(Copy, Clone, Debug)]
pub struct Parser<P>(pub(crate) P);

impl<P> Parser<P> {
    pub fn and_then<F, N, I>(self, f: F) -> Parser<AndThen<P, F>>
        where
            P: Parse<I>,
            F: Fn(&P::Out) -> N,
            N: Parse<I, Err=P::Err>,
    {
        Parser(AndThen(self.0, f))
    }

    pub fn or_else<F, N, I>(self, f: F) -> Parser<OrElse<P, F>>
        where
            P: Parse<I>,
            F: Fn(P::Err) -> N,
            N: Parse<I, Out=P::Out>,
            I: Copy,
    {
        Parser(OrElse(self.0, f))
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

    pub fn boxed<'p, I>(self) -> Parser<Boxed<'p, I, P::Out, P::Err>>
        where
            P: Parse<I> + 'p,
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

pub fn pred_fn<F>(f: F) -> Parser<PredFn<F>>
    where
        F: Fn(char) -> bool,
{
    Parser(PredFn(f))
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

#[derive(Copy, Clone, Debug)]
pub struct AndThen<P, F>(pub(crate) P, pub(crate) F);

impl<P, F, N, I> Parse<I> for AndThen<P, F>
    where
        P: Parse<I>,
        F: Fn(&P::Out) -> N,
        N: Parse<I, Err=P::Err>,
{
    type Err = P::Err;
    type Out = (P::Out, N::Out);

    fn parse(&self, input: I) -> Result<(Self::Out, I), Self::Err> {
        self.0.parse(input)
            .and_then(|(p, rest)| (self.1)(&p).parse(rest)
                .map(|(n, rest)| ((p, n), rest))
            )
    }
}

#[derive(Copy, Clone, Debug)]
pub struct OrElse<P, F>(pub(crate) P, pub(crate) F);

impl<P, F, N, I> Parse<I> for OrElse<P, F>
    where
        P: Parse<I>,
        F: Fn(P::Err) -> N,
        N: Parse<I, Out=P::Out>,
        I: Copy,
{
    type Err = N::Err;
    type Out = P::Out;

    fn parse(&self, input: I) -> Result<(Self::Out, I), Self::Err> {
        self.0.parse(input).or_else(|e| (self.1)(e).parse(input))
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

    #[test]
    fn parser_and_then() {
        let num = (par('a'..='z') | par('A'..='Z')) * ..;
        let p = num.and_then(|n: &String| par(':') >> n.clone());

        assert_eq!(p.parse_result("Hello:Hello"), Ok(("Hello".to_string(), "Hello")));
        assert_eq!(p.parse_result("Hello:Hi!"), Err(()));
    }

    #[test]
    fn parser_or_else() {
        let p = par('w').map_err(|_| 'q').or_else(|e| par(e));

        assert_eq!(p.parse_result("w"), Ok("w"));
        assert_eq!(p.parse_result("q"), Ok("q"));
        assert_eq!(p.parse_result("e"), Err(()));
    }
}
