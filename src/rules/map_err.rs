use crate::{Comply, Parser};

#[derive(Copy, Clone, Debug)]
pub struct MapErr<R, F>(pub R, pub F);

impl<'p, R, F, G> Comply<'p> for MapErr<R, F>
    where
        R: Comply<'p>,
        F: Fn(R::Err) -> G,
        G: 'p,
{
    type Res = R::Res;
    type Err = G;
    type On = R::On;

    fn comply(&self, parser: &mut Parser<Self::On>) -> Result<Self::Res, Self::Err> {
        self.0.comply(parser).map_err(|e| (self.1)(e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rule;

    #[test]
    fn and_then() {
        let r = MapErr(rule('1'), |_| 1);

        assert_eq!(
            Parser::new("1").parse(r),
            (Ok("1"), ""),
        );
        assert_eq!(
            Parser::new("2").parse(r),
            (Err(1), "2"),
        );
    }
}
