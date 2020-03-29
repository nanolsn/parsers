use crate::{
    apply::Apply,
    ruled::Ruled,
    rule::Rule,
};

#[derive(Copy, Clone, Debug)]
pub struct Oct;

pub fn oct() -> Rule<Oct> { Rule(Oct) }

impl<'i> Apply<&'i str> for Oct {
    type Err = ();
    type Res = &'i str;

    fn apply(self, input: &'i str) -> Ruled<&'i str, Self::Res, Self::Err> {
        match input.chars().next() {
            Some(c @ '0'..='7') => input.split_at(c.len_utf8()).into(),
            _ => Ruled::Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::apply::apply;

    #[test]
    fn oct() {
        assert_eq!(apply(super::oct(), "0"), Ruled::Ok("0", ""));
        assert_eq!(apply(super::oct(), "7"), Ruled::Ok("7", ""));
        assert_eq!(apply(super::oct(), "8"), Ruled::Err(()));
        assert_eq!(apply(super::oct(), "a"), Ruled::Err(()));
        assert_eq!(apply(super::oct(), "A"), Ruled::Err(()));
    }
}
