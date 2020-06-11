use crate::{
    apply::Apply,
    ruled::Ruled,
    rule::Rule,
    expected::Expected,
};

#[derive(Copy, Clone, Debug)]
pub struct Bin;

pub fn bin() -> Rule<Bin> { Rule(Bin) }

impl<'i> Apply<&'i str> for Bin {
    type Err = Expected<'static>;
    type Res = &'i str;

    fn apply(self, input: &'i str) -> Ruled<&'i str, Self::Res, Self::Err> {
        match input.chars().next() {
            Some(a @ '0') | Some(a @ '1') => input.split_at(a.len_utf8()).into(),
            _ => Ruled::Expected(Expected::Bin),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::apply::apply;

    #[test]
    fn bin() {
        assert_eq!(apply(super::bin(), "0"), Ruled::Match("0", ""));
        assert_eq!(apply(super::bin(), "1"), Ruled::Match("1", ""));
        assert_eq!(apply(super::bin(), "2"), Ruled::Expected(Expected::Bin));
    }
}
