use crate::Parse;

pub const ANY: Any = Any;

#[derive(Copy, Clone, Debug)]
pub struct Any;

impl<'i> Parse<&'i str> for Any {
    type Err = ();
    type Out = String;

    fn parse(&self, input: &'i str) -> Result<(Self::Out, &'i str), Self::Err> {
        match input.chars().next() {
            None => Err(()),
            Some(c) => {
                let (left, right) = input.split_at(c.len_utf8());
                Ok((left.to_string(), right))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn any() {
        assert_eq!(ANY.parse("%^&"), Ok(("%".to_string(), "^&")));
        assert_eq!(ANY.parse(""), Err(()));
    }
}
