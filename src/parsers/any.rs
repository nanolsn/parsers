use crate::{Parse, Parser, Parsed};

pub const ANY: Any = Any;

#[derive(Copy, Clone, Debug)]
pub struct Any;

impl<'p> Parse<'p> for Any {
    type Res = &'p str;
    type Err = ();
    type On = &'p str;

    fn parse(&self, input: Self::On) -> Parsed<Self::Res, Self::Err, Self::On> {
        match input.chars().next() {
            None => Err(()),
            Some(c) => Ok(input.split_at(c.len_utf8())),
        }
    }
}

pub fn any() -> Parser<Any> {
    Parser(Any)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn any() {
        assert_eq!(ANY.parse("%^&"), Ok(("%", "^&")));
        assert_eq!(ANY.parse(""), Err(()));
    }
}
