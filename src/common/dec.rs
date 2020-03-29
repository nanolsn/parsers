use crate::{
    apply::Apply,
    ruled::Ruled,
    rule::Rule,
};

#[derive(Copy, Clone, Debug)]
pub struct Dec;

pub fn dec() -> Rule<Dec> { Rule(Dec) }

impl<'i> Apply<&'i str> for Dec {
    type Err = ();
    type Res = &'i str;

    fn apply(self, input: &'i str) -> Ruled<&'i str, Self::Res, Self::Err> {
        match input.chars().next() {
            Some(c @ '0'..='9') => input.split_at(c.len_utf8()).into(),
            _ => Ruled::Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::apply::apply;

    #[test]
    fn dec() {
        assert_eq!(apply(super::dec(), "0"), Ruled::Ok("0", ""));
        assert_eq!(apply(super::dec(), "9"), Ruled::Ok("9", ""));
        assert_eq!(apply(super::dec(), "a"), Ruled::Err(()));
    }
}
