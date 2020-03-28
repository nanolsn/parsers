use crate::{
    apply::Apply,
    ruled::Ruled,
    rule::Rule,
};

#[derive(Copy, Clone, Debug)]
pub struct Hex;

pub fn hex() -> Rule<Hex> { Rule(Hex) }

impl<'i> Apply<&'i str> for Hex {
    type Err = ();
    type Res = &'i str;

    fn apply(&self, input: &'i str) -> Ruled<&'i str, Self::Res, Self::Err> {
        let c = match input.chars().next() {
            Some(c @ '0'..='9') => c,
            Some(c @ 'a'..='f') => c,
            Some(c @ 'A'..='F') => c,
            _ => return Ruled::Err(()),
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
        assert_eq!(apply(&super::hex(), "0"), Ruled::Ok("0", ""));
        assert_eq!(apply(&super::hex(), "9"), Ruled::Ok("9", ""));
        assert_eq!(apply(&super::hex(), "a"), Ruled::Ok("a", ""));
        assert_eq!(apply(&super::hex(), "A"), Ruled::Ok("A", ""));
        assert_eq!(apply(&super::hex(), "f"), Ruled::Ok("f", ""));
        assert_eq!(apply(&super::hex(), "F"), Ruled::Ok("F", ""));
        assert_eq!(apply(&super::hex(), "g"), Ruled::Err(()));
    }
}
