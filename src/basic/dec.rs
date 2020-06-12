use crate::prelude::*;

#[derive(Copy, Clone, Debug)]
pub struct Dec;

pub fn dec() -> Dec { Dec }

impl<'r, 'i: 'r> Rule<'r, &'i str> for Dec {
    type Mat = &'i str;
    type Exp = Failed<'static>;

    fn rule(&'r self, input: &'i str) -> Ruled<&'i str, Self::Mat, Self::Exp> {
        match input.chars().next() {
            Some(c @ '0'..='9') => input.split_at(c.len_utf8()).into(),
            _ => Expected(Failed::Dec),
        }
    }
}

impl_ops!(Dec);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dec() {
        assert_eq!(super::dec().rule("0"), Match("0", ""));
        assert_eq!(super::dec().rule("9"), Match("9", ""));
        assert_eq!(super::dec().rule("a"), Expected(Failed::Dec));
    }
}
