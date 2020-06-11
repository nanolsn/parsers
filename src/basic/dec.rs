use crate::{
    rule::Rule,
    ruled::Ruled,
    rul::Rul,
    some_of::SomeOf,
};

#[derive(Copy, Clone, Debug)]
pub struct Dec;

pub fn dec() -> Rul<Dec> { Rul(Dec) }

impl<'i> Rule<&'i str> for Dec {
    type Exp = SomeOf<'static>;
    type Mat = &'i str;

    fn rule(self, input: &'i str) -> Ruled<&'i str, Self::Res, Self::Err> {
        match input.chars().next() {
            Some(c @ '0'..='9') => input.split_at(c.len_utf8()).into(),
            _ => Ruled::Expected(SomeOf::Dec),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dec() {
        assert_eq!(super::dec().rule("0"), Ruled::Match("0", ""));
        assert_eq!(super::dec().rule("9"), Ruled::Match("9", ""));
        assert_eq!(super::dec().rule("a"), Ruled::Expected(SomeOf::Dec));
    }
}
