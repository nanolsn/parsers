use crate::{
    rule::Rule,
    ruled::Ruled,
    rul::Rul,
    some_of::SomeOf,
};

const SPACE: char = ' ';

#[derive(Copy, Clone, Debug)]
pub struct Space;

pub fn space() -> Rul<Space> { Rul(Space) }

impl<'i> Rule<&'i str> for Space {
    type Exp = SomeOf<'static>;
    type Mat = &'i str;

    fn rule(self, input: &'i str) -> Ruled<&'i str, Self::Res, Self::Err> {
        match input.chars().next() {
            Some(SPACE) => input.split_at(SPACE.len_utf8()).into(),
            _ => Ruled::Expected(SomeOf::Char(SPACE)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn space() {
        assert!(super::space().rule(" ").is_match());
        assert!(super::space().rule("q").is_expected());
    }
}
