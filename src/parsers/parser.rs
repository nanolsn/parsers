use crate::{Parse, Until, RangeVec, UntilVec, Pred, Opt, PredFn, Parsed};
use crate::parsers::range::Range;
use std::ops::Deref;

#[derive(Copy, Clone, Debug)]
pub struct Parser<P>(pub(crate) P);

impl<P> Parser<P> {
    pub fn and_then<'p, F>(self, f: F) -> Parser<AndThen<P, F>> {
        Parser(AndThen(self.0, f))
    }

    pub fn or_else<F>(self, f: F) -> Parser<OrElse<P, F>> {
        Parser(OrElse(self.0, f))
    }

    pub fn repeat(self, times: usize) -> Parser<Range<P>> {
        Parser(Range {
            parser: self.0,
            from: times,
            to: Some(times),
        })
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

    pub fn boxed<'p>(self) -> BoxedParser<'p, P::Res, P::Err, P::On>
        where
            P: Parse<'p> + 'p,
    {
        Parser(Box::new(self.0))
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

impl<'p, P> Parse<'p> for Parser<P>
    where
        P: Parse<'p>,
{
    type Res = P::Res;
    type Err = P::Err;
    type On = P::On;

    fn parse(&self, input: Self::On) -> Parsed<Self::Res, Self::Err, Self::On> {
        self.0.parse(input)
    }
}

pub fn par<'p, P>(parse: P) -> Parser<P>
    where
        P: Parse<'p>,
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

impl<'p, P> Parse<'p> for StringedParser<P>
    where
        P: Parse<'p, Res=&'p str, On=&'p str>,
{
    type Res = String;
    type Err = P::Err;
    type On = &'p str;

    fn parse(&self, input: Self::On) -> Parsed<Self::Res, Self::Err, Self::On> {
        self.0.parse(input).map(|(out, rest)| (out.to_string(), rest))
    }
}

pub fn stringed_par<'p, P>(parse: P) -> Parser<StringedParser<P>>
    where
        P: Parse<'p>,
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

impl<'p, P, F, N> Parse<'p> for AndThen<P, F>
    where
        P: Parse<'p>,
        F: Fn(P::Res) -> N,
        N: Parse<'p, Err=P::Err, On=P::On>,
{
    type Res = N::Res;
    type Err = P::Err;
    type On = P::On;

    fn parse(&self, input: Self::On) -> Parsed<Self::Res, Self::Err, Self::On> {
        self.0.parse(input)
            .and_then(|(p, rest)| (self.1)(p).parse(rest))
    }
}

#[derive(Copy, Clone, Debug)]
pub struct OrElse<P, F>(pub(crate) P, pub(crate) F);

impl<'p, P, F, N> Parse<'p> for OrElse<P, F>
    where
        P: Parse<'p>,
        F: Fn(P::Err) -> N,
        N: Parse<'p, Res=P::Res, On=P::On>,
        P::On: Copy,
{
    type Res = P::Res;
    type Err = N::Err;
    type On = P::On;

    fn parse(&self, input: Self::On) -> Parsed<Self::Res, Self::Err, Self::On> {
        self.0.parse(input).or_else(|e| (self.1)(e).parse(input))
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Map<P, F>(pub(crate) P, pub(crate) F);

impl<'p, P, F, A, B> Parse<'p> for Map<P, F>
    where
        P: Parse<'p, Res=A>,
        F: Fn(A) -> B,
        A: 'p,
        B: 'p,
{
    type Res = B;
    type Err = P::Err;
    type On = P::On;

    fn parse(&self, input: Self::On) -> Parsed<Self::Res, Self::Err, Self::On> {
        self.0.parse(input).map(|(out, rest)| ((self.1)(out), rest))
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MapErr<P, F>(pub(crate) P, pub(crate) F);

impl<'p, P, F, E, G> Parse<'p> for MapErr<P, F>
    where
        P: Parse<'p, Err=E>,
        F: Fn(E) -> G,
        E: 'p,
        G: 'p,
{
    type Res = P::Res;
    type Err = G;
    type On = P::On;

    fn parse(&self, input: Self::On) -> Parsed<Self::Res, Self::Err, Self::On> {
        self.0.parse(input).map_err(|e| (self.1)(e))
    }
}

pub type BoxedParser<'p, R, E=(), O=&'p str> = Parser<Box<dyn Parse<'p, Res=R, Err=E, On=O> + 'p>>;

impl<'p, R, E, O> Parse<'p> for Box<dyn Parse<'p, Res=R, Err=E, On=O> + 'p>
    where
        R: 'p,
        E: 'p,
{
    type Res = R;
    type Err = E;
    type On = O;

    fn parse(&self, input: Self::On) -> Result<(Self::Res, Self::On), Self::Err> {
        self.as_ref().parse(input)
    }
}

pub fn boxed<'p, P>(parser: P) -> BoxedParser<'p, P::Res, P::Err, P::On>
    where
        P: Parse<'p> + 'p,
{
    Parser(Box::new(parser))
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
        let p = super::stringed_par("a");

        assert_eq!(p.parse("a b"), Ok(("a".to_string(), " b")));
        assert_eq!(p.parse("b"), Err(()));

        let p = super::stringed_par("a");

        assert_eq!(p.parse("a b"), Ok(("a".to_string(), " b")));
        assert_eq!(p.parse("b"), Err(()));
    }

    #[test]
    fn parser_and_then() {
        let num = (par('a'..='z') | par('A'..='Z')) * ..;
        let p = num.and_then(|n: String| par(':') >> n);

        assert_eq!(p.parse_result("Hello:Hello"), Ok("Hello"));
        assert_eq!(p.parse_result("Hello:Hi!"), Err(()));
    }

    #[test]
    fn parser_or_else() {
        let p = par('w').map_err(|_| 'q').or_else(|e| par(e));

        assert_eq!(p.parse_result("w"), Ok("w"));
        assert_eq!(p.parse_result("q"), Ok("q"));
        assert_eq!(p.parse_result("e"), Err(()));
    }

    #[test]
    fn parse_map() {
        let l = par("a").map(|_| 1);

        assert_eq!(l.parse("ab"), Ok((1, "b")));
        assert_eq!(l.parse("c"), Err(()));
    }

    #[test]
    fn parse_map_err() {
        let l = par("a").map_err(|_| 1);

        assert_eq!(l.parse("ab"), Ok(("a", "b")));
        assert_eq!(l.parse("c"), Err(1));
    }
}
