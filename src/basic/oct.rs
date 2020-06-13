use crate::prelude::*;

#[derive(Copy, Clone, Debug)]
pub struct Oct;

pub fn oct() -> Oct { Oct }

impl<'r, 'i> Rule<'r, &'i str> for Oct {
    type Mat = &'i str;
    type Exp = Failed<'static>;

    fn rule(&'r self, input: &'i str) -> Ruled<&'i str, Self::Mat, Self::Exp> {
        match input.chars().next() {
            Some(c @ '0'..='7') => input.split_at(c.len_utf8()).into(),
            _ => Expected(Failed::Oct),
        }
    }
}

impl_ops!(Oct);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn oct() {
        assert_eq!(super::oct().rule("0"), Match("0", ""));
        assert_eq!(super::oct().rule("7"), Match("7", ""));
        assert_eq!(super::oct().rule("8"), Expected(Failed::Oct));
        assert_eq!(super::oct().rule("a"), Expected(Failed::Oct));
        assert_eq!(super::oct().rule("A"), Expected(Failed::Oct));
    }
}
