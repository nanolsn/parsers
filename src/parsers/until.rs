use crate::{Parse, Parsed};

#[derive(Copy, Clone, Debug)]
pub struct Until<P, U>(pub(crate) P, pub(crate) U);

impl<'p, P, S, U> Parse<'p> for Until<P, U>
    where
        P: Parse<'p, Res=S, On=&'p str>,
        S: AsRef<str> + 'p,
        U: Parse<'p, On=&'p str>,
{
    type Res = (String, U::Res);
    type Err = P::Err;
    type On = &'p str;

    fn parse(&self, mut rest: Self::On) -> Parsed<Self::Res, Self::Err, Self::On> {
        let mut s = String::new();

        loop {
            match self.1.parse(rest) {
                Ok((u, rest)) => break Ok(((s, u), rest)),
                Err(_) => {
                    match self.0.parse(rest) {
                        Ok((out, r)) => {
                            rest = r;
                            s.push_str(out.as_ref());
                        }
                        Err(e) => break Err(e),
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ANY, par};

    #[test]
    fn until() {
        let u = par(ANY).until("%^");

        assert_eq!(u.parse("@#_%_$%^&"), Ok((("@#_%_$".to_string(), "%^"), "&")));

        let u = par(ANY).until("!");

        assert_eq!(u.parse("..."), Err(()));
    }
}
