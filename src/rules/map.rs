use crate::{Comply, Parser};

#[derive(Copy, Clone, Debug)]
pub struct Map<R, F>(pub R, pub F);

impl<'p, R, F, K> Comply<'p> for Map<R, F>
    where
        R: Comply<'p>,
        F: Fn(R::Res) -> K,
        K: 'p,
{
    type Res = K;
    type Err = R::Err;
    type On = R::On;

    fn comply(&self, parser: &mut Parser<Self::On>) -> Result<Self::Res, Self::Err> {
        self.0.comply(parser).map(|r| (self.1)(r))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{rule, Or};
    use std::str::FromStr;

    #[test]
    fn and_then() {
        let r = Map(Or(rule('1'), '2'), |s: &str| i32::from_str(s).unwrap());

        assert_eq!(
            Parser::new("1").parse(r),
            (Ok(1), ""),
        );
        assert_eq!(
            Parser::new("2").parse(r),
            (Ok(2), ""),
        );
        assert_eq!(
            Parser::new("3").parse(r),
            (Err(()), "3"),
        );
    }
}
