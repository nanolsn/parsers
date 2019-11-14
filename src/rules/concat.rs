use crate::Comply;
use crate::Parser;
use crate::Rule;
use std::ops::BitAnd;

#[derive(Copy, Clone, Debug)]
pub struct Concat<A, B>(pub A, pub B);

impl<'p, A, B, T, S> Comply<'p> for Concat<A, B>
    where
        A: Comply<'p, Res=T>,
        B: Comply<'p, Res=S, Err=A::Err, On=A::On>,
        T: Into<String> + 'p,
        S: AsRef<str> + 'p,
{
    type Res = String;
    type Err = A::Err;
    type On = A::On;

    fn comply(&self, parser: &mut Parser<Self::On>) -> Result<Self::Res, Self::Err> {
        let pos = parser.get_pos();
        let a = self.0.comply(parser).map_err(|e| {
            assert_eq!(parser.get_pos(), pos);
            e
        })?;

        let b = self.1.comply(parser).map_err(|e| {
            parser.set_pos(pos);
            e
        })?;

        let mut a = a.into();
        a.push_str(b.as_ref());
        Ok(a)
    }
}

impl<'p, A, B> BitAnd<B> for Rule<A>
    where
        A: Comply<'p>,
        B: Comply<'p, Err=A::Err, On=A::On>,
{
    type Output = Rule<Concat<A, B>>;

    fn bitand(self, rhs: B) -> Self::Output {
        Rule(Concat(self.0, rhs))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rule;

    #[test]
    fn concat() {
        let r = rule('@') & '#';

        assert_eq!(
            Parser::new("@#".to_owned().as_str()).parse(r),
            (Ok("@#".to_string()), ""),
        );
        assert_eq!(
            Parser::new("@!").parse(r),
            (Err(()), "@!"),
        );
        assert_eq!(
            Parser::new("#$").parse(r),
            (Err(()), "#$"),
        );
    }
}
