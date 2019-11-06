use crate::Parse;

pub struct Map<P, F>(pub(crate) P, pub(crate) F);

impl<P, F, A, B, I> Parse<I> for Map<P, F>
    where
        P: Parse<I, Out=A>,
        F: Fn(A) -> B,
{
    type Err = P::Err;
    type Out = B;

    fn parse(&self, input: I) -> Result<(Self::Out, I), Self::Err> {
        self.0.parse(input).map(|(out, rest)| ((self.1)(out), rest))
    }
}

pub struct MapErr<P, F>(pub(crate) P, pub(crate) F);

impl<P, F, E, G, I> Parse<I> for MapErr<P, F>
    where
        P: Parse<I, Err=E>,
        F: Fn(E) -> G + Copy,
{
    type Err = G;
    type Out = P::Out;

    fn parse(&self, input: I) -> Result<(Self::Out, I), Self::Err> {
        self.0.parse(input).map_err(self.1)
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
