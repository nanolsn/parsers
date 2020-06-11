use crate::{
    rule::Rule,
    ruled::Ruled,
    rul::Rul,
    some_of::SomeOf,
};

#[derive(Copy, Clone, Debug)]
pub struct Oct;

pub fn oct() -> Rul<Oct> { Rul(Oct) }

impl<'i> Rule<&'i str> for Oct {
    type Exp = SomeOf<'static>;
    type Mat = &'i str;

    fn rule(self, input: &'i str) -> Ruled<&'i str, Self::Res, Self::Err> {
        match input.chars().next() {
            Some(c @ '0'..='7') => input.split_at(c.len_utf8()).into(),
            _ => Ruled::Expected(SomeOf::Oct),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn oct() {
        assert_eq!(super::oct().rule("0"), Ruled::Match("0", ""));
        assert_eq!(super::oct().rule("7"), Ruled::Match("7", ""));
        assert_eq!(super::oct().rule("8"), Ruled::Expected(SomeOf::Oct));
        assert_eq!(super::oct().rule("a"), Ruled::Expected(SomeOf::Oct));
        assert_eq!(super::oct().rule("A"), Ruled::Expected(SomeOf::Oct));
    }
}
