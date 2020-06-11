use crate::{
    apply::Apply,
    ruled::Ruled,
    rule::Rule,
    expected::Expected,
};

#[derive(Copy, Clone, Debug)]
pub struct Any;

pub fn any() -> Rule<Any> { Rule(Any) }

impl<'i> Apply<&'i str> for Any {
    type Err = Expected<'static>;
    type Res = &'i str;

    fn apply(self, input: &'i str) -> Ruled<&'i str, Self::Res, Self::Err> {
        match input.chars().next() {
            None => Ruled::Expected(Expected::AnyChar),
            Some(c) => input.split_at(c.len_utf8()).into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::apply::apply;

    #[test]
    fn any() {
        assert_eq!(apply(super::any(), "!@#$"), Ruled::Match("!", "@#$"));
        assert_eq!(apply(super::any(), ""), Ruled::Expected(Expected::AnyChar));
    }
}
