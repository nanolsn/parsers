use crate::{
    rule::Rule,
    ruled::Ruled,
    rul::Rul,
    some_of::SomeOf,
};

#[derive(Copy, Clone, Debug)]
pub struct Hex;

pub fn hex() -> Rul<Hex> { Rul(Hex) }

impl<'i> Rule<&'i str> for Hex {
    type Exp = SomeOf<'static>;
    type Mat = &'i str;

    fn rule(self, input: &'i str) -> Ruled<&'i str, Self::Res, Self::Err> {
        let c = match input.chars().next() {
            Some(c @ '0'..='9') => c,
            Some(c @ 'a'..='f') => c,
            Some(c @ 'A'..='F') => c,
            _ => return Ruled::Expected(SomeOf::Hex),
        };

        input.split_at(c.len_utf8()).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex() {
        assert_eq!(super::hex().rule("0"), Ruled::Match("0", ""));
        assert_eq!(super::hex().rule("9"), Ruled::Match("9", ""));
        assert_eq!(super::hex().rule("a"), Ruled::Match("a", ""));
        assert_eq!(super::hex().rule("A"), Ruled::Match("A", ""));
        assert_eq!(super::hex().rule("f"), Ruled::Match("f", ""));
        assert_eq!(super::hex().rule("F"), Ruled::Match("F", ""));
        assert_eq!(super::hex().rule("g"), Ruled::Expected(SomeOf::Hex));
    }
}
