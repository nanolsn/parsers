use crate::prelude::*;

#[derive(Copy, Clone, Debug)]
pub struct Hex;

pub fn hex() -> Hex { Hex }

impl<'r, 'i> Rule<'r, &'i str> for Hex {
    type Mat = &'i str;
    type Exp = Failed<'static>;

    fn rule(&'r self, input: &'i str) -> Ruled<&'i str, Self::Mat, Self::Exp> {
        let c = match input.chars().next() {
            Some(c @ '0'..='9') => c,
            Some(c @ 'a'..='f') => c,
            Some(c @ 'A'..='F') => c,
            _ => return Expected(Failed::Hex),
        };

        input.split_at(c.len_utf8()).into()
    }
}

impl_ops!(Hex);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex() {
        assert_eq!(super::hex().rule("0"), Match("0", ""));
        assert_eq!(super::hex().rule("9"), Match("9", ""));
        assert_eq!(super::hex().rule("a"), Match("a", ""));
        assert_eq!(super::hex().rule("A"), Match("A", ""));
        assert_eq!(super::hex().rule("f"), Match("f", ""));
        assert_eq!(super::hex().rule("F"), Match("F", ""));
        assert_eq!(super::hex().rule("g"), Expected(Failed::Hex));
    }
}
