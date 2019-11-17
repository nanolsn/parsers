use crate::Comply;
use crate::Parser;

#[derive(Copy, Clone, Debug)]
pub struct Ret<V>(pub V);

impl<'p, V> Comply<'p> for Ret<V>
    where
        V: Copy + 'p,
{
    type Res = V;
    type Err = ();
    type On = &'p str;

    fn comply(&self, _: &mut Parser<Self::On>) -> Result<Self::Res, Self::Err> {
        Ok(self.0)
    }
}

pub fn ret<V>(value: V) -> Ret<V>
    where
        V: Copy,
{
    Ret(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ret() {
        let r = super::ret(12);

        assert_eq!(
            Parser::new("hello").parse(r),
            (Ok(12), "hello"),
        );
    }
}
