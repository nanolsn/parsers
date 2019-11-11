use crate::{Parse, Parsed};

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

#[cfg(test)]
mod tests {
    use crate::{par, Parse};

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
