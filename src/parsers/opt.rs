use crate::{Parse, Parsed};

#[derive(Copy, Clone, Debug)]
pub struct Opt<P>(pub(crate) P);

impl<'p, P> Parse<'p> for Opt<P>
    where
        P: Parse<'p, Res=&'p str, On=&'p str>,
{
    type Res = P::Res;
    type Err = P::Err;
    type On = &'p str;

    fn parse(&self, input: Self::On) -> Parsed<Self::Res, Self::Err, Self::On> {
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
