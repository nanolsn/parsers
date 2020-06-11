use crate::{
    apply::Apply,
    ruled::Ruled,
    rule::Rule,
    expected::Expected,
};

#[derive(Copy, Clone, Debug)]
pub struct Hex;

pub fn hex() -> Rule<Hex> { Rule(Hex) }

impl<'i> Apply<&'i str> for Hex {
    type Err = Expected<'static>;
    type Res = &'i str;

    fn apply(self, input: &'i str) -> Ruled<&'i str, Self::Res, Self::Err> {
        let c = match input.chars().next() {
            Some(c @ '0'..='9') => c,
            Some(c @ 'a'..='f') => c,
            Some(c @ 'A'..='F') => c,
            _ => return Ruled::Expected(Expected::Hex),
        };

        input.split_at(c.len_utf8()).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::apply::apply;

    #[test]
    fn hex() {
        assert_eq!(apply(super::hex(), "0"), Ruled::Match("0", ""));
        assert_eq!(apply(super::hex(), "9"), Ruled::Match("9", ""));
        assert_eq!(apply(super::hex(), "a"), Ruled::Match("a", ""));
        assert_eq!(apply(super::hex(), "A"), Ruled::Match("A", ""));
        assert_eq!(apply(super::hex(), "f"), Ruled::Match("f", ""));
        assert_eq!(apply(super::hex(), "F"), Ruled::Match("F", ""));
        assert_eq!(apply(super::hex(), "g"), Ruled::Expected(Expected::Hex));
    }
}
