use crate::{Comply, Parser};

#[derive(Copy, Clone, Debug)]
pub struct UntilVec<R, U>(pub R, pub U);

impl<'p, R, U> Comply<'p> for UntilVec<R, U>
    where
        R: Comply<'p>,
        U: Comply<'p, On=R::On>,
        R::On: Copy,
{
    type Res = (Vec<R::Res>, U::Res);
    type Err = R::Err;
    type On = R::On;

    fn comply(&self, parser: &mut Parser<Self::On>) -> Result<Self::Res, Self::Err> {
        let pos = parser.get_pos();
        let mut v = Vec::new();

        loop {
            match self.1.comply(parser) {
                Ok(u) => break Ok((v, u)),
                Err(_) => {
                    match self.0.comply(parser) {
                        Ok(r) => v.push(r),
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
    fn until_vec() {
        let u = any().until_vec("%^");

        assert_eq!(
            Parser::new("@#_%_$%^&").parse(u),
            (Ok((vec!["@", "#", "_", "%", "_", "$"], "%^")), "&"),
        );

        let u = any().until_vec("!");

        assert_eq!(
            Parser::new("...").parse(u),
            (Err(()), "..."),
        );
    }
}
