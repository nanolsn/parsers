use crate::{Comply, Parser};

#[derive(Copy, Clone, Debug)]
pub struct Until<R, U>(pub R, pub U);

impl<'p, R, S, U> Comply<'p> for Until<R, U>
    where
        R: Comply<'p, Res=S, On=&'p str>,
        S: AsRef<str> + 'p,
        U: Comply<'p, On=&'p str>,
{
    type Res = (String, U::Res);
    type Err = R::Err;
    type On = &'p str;

    fn comply(&self, parser: &mut Parser<Self::On>) -> Result<Self::Res, Self::Err> {
        let pos = parser.get_pos();
        let mut s = String::new();

        loop {
            match self.1.comply(parser) {
                Ok(u) => break Ok((s, u)),
                Err(_) => {
                    match self.0.comply(parser) {
                        Ok(r) => s.push_str(r.as_ref()),
                        Err(e) => {
                            parser.set_pos(pos);
                            break Err(e)
                        },
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::any;

    #[test]
    fn until() {
        let u = any().until("%^");

        assert_eq!(
            Parser::new("@#_%_$%^&").parse(u),
            (Ok(("@#_%_$".to_string(), "%^")), "&"),
        );

        let u = any().until("!");

        assert_eq!(
            Parser::new("...").parse(u),
            (Err(()), "..."),
        );
    }
}
