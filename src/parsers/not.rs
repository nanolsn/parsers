use crate::{Parse, Parser};

#[derive(Copy, Clone, Debug)]
pub struct Not<P>(pub(crate) P);

impl<P, I> Parse<I> for Not<P>
    where
        P: Parse<I>,
        I: Copy,
{
    type Err = P::Out;
    type Out = P::Err;

    fn parse(&self, input: I) -> Result<(Self::Out, I), Self::Err> {
        match self.0.parse(input) {
            Ok((out, _)) => Err(out),
            Err(e) => Ok((e, input)),
        }
    }
}

impl<P> std::ops::Not for Parser<P> {
    type Output = Parser<Not<P>>;

    fn not(self) -> Self::Output {
        Parser(Not(self.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::par;

    #[test]
    fn not() {
        let p = !par("hello");

        assert_eq!(p.parse("hello"), Err("hello"));
        assert_eq!(p.parse("hi"), Ok(((), "hi")));
    }
}
