use crate::{
    apply::Apply,
    ruled::Ruled,
    rule::Rule,
};

#[derive(Copy, Clone, Debug)]
pub struct Any;

pub fn any() -> Rule<Any> { Rule(Any) }

impl<'i> Apply<&'i str> for Any {
    type Err = ();
    type Res = &'i str;

    fn apply(self, input: &'i str) -> Ruled<&'i str, Self::Res, Self::Err> {
        match input.chars().next() {
            None => Ruled::Err(()),
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
        assert_eq!(apply(super::any(), "!@#$"), Ruled::Ok("!", "@#$"));
        assert_eq!(apply(super::any(), ""), Ruled::Err(()));
    }
}
