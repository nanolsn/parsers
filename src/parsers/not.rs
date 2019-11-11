use crate::{Parse, Parser, Parsed};

#[derive(Copy, Clone, Debug)]
pub struct Not<P>(pub(crate) P);

impl<'p, P> Parse<'p> for Not<P>
    where
        P: Parse<'p>,
        P::On: Copy,
{
    type Res = P::Err;
    type Err = P::Res;
    type On = P::On;

    fn parse(&self, input: Self::On) -> Parsed<Self::Res, Self::Err, Self::On> {
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
