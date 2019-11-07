use crate::Parse;

pub const ANY: Any = Any;

#[derive(Copy, Clone, Debug)]
pub struct Any;

impl<'i> Parse<&'i str> for Any {
    type Err = ();
    type Out = &'i str;

    fn parse(&self, input: &'i str) -> Result<(Self::Out, &'i str), Self::Err> {
        match input.chars().next() {
            None => Err(()),
            Some(c) => Ok(input.split_at(c.len_utf8())),
        }
    }
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
