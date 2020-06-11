use crate::{
    rule::Rule,
    ruled::Ruled,
    rul::Rul,
    some_of::SomeOf,
};

#[derive(Copy, Clone, Debug)]
pub struct Any;

pub fn any() -> Rul<Any> { Rul(Any) }

impl<'i> Rule<&'i str> for Any {
    type Exp = SomeOf<'static>;
    type Mat = &'i str;

    fn rule(self, input: &'i str) -> Ruled<&'i str, Self::Res, Self::Err> {
        match input.chars().next() {
            None => Ruled::Expected(SomeOf::AnyChar),
            Some(c) => input.split_at(c.len_utf8()).into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn any() {
        assert_eq!(super::any().rule("!@#$"), Ruled::Match("!", "@#$"));
        assert_eq!(super::any().rule(""), Ruled::Expected(SomeOf::AnyChar));
    }
}
