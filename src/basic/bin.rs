use crate::{
    rule::Rule,
    ruled::Ruled,
    rul::Rul,
    some_of::SomeOf,
};

#[derive(Copy, Clone, Debug)]
pub struct Bin;

pub fn bin() -> Rul<Bin> { Rul(Bin) }

impl<'i> Rule<&'i str> for Bin {
    type Exp = SomeOf<'static>;
    type Mat = &'i str;

    fn rule(self, input: &'i str) -> Ruled<&'i str, Self::Res, Self::Err> {
        match input.chars().next() {
            Some(a @ '0') | Some(a @ '1') => input.split_at(a.len_utf8()).into(),
            _ => Ruled::Expected(SomeOf::Bin),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bin() {
        assert_eq!(super::bin().rule("0"), Ruled::Match("0", ""));
        assert_eq!(super::bin().rule("1"), Ruled::Match("1", ""));
        assert_eq!(super::bin().rule("2"), Ruled::Expected(SomeOf::Bin));
    }
}
