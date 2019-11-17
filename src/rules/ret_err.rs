use crate::Comply;
use crate::Parser;

#[derive(Copy, Clone, Debug)]
pub struct RetErr<E>(pub E);

impl<'p, E> Comply<'p> for RetErr<E>
    where
        E: Copy + 'p,
{
    type Res = ();
    type Err = E;
    type On = &'p str;

    fn comply(&self, _: &mut Parser<Self::On>) -> Result<Self::Res, Self::Err> {
        Err(self.0)
    }
}

pub fn ret_err<E>(err: E) -> RetErr<E>
    where
        E: Copy,
{
    RetErr(err)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ret_err() {
        let r = super::ret_err(12);

        assert_eq!(
            Parser::new("hello").parse(r),
            (Err(12), "hello"),
        );
    }
}
