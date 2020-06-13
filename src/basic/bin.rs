use crate::prelude::*;

#[derive(Copy, Clone, Debug)]
pub struct Bin;

pub fn bin() -> Bin { Bin }

impl<'r, 'i> Rule<'r, &'i str> for Bin {
    type Mat = &'i str;
    type Exp = Failed<'static>;

    fn rule(&'r self, input: &'i str) -> Ruled<&'i str, Self::Mat, Self::Exp> {
        match input.chars().next() {
            Some(a @ '0') | Some(a @ '1') => input.split_at(a.len_utf8()).into(),
            _ => Expected(Failed::Bin),
        }
    }
}

impl_ops!(Bin);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bin() {
        assert!(super::bin().test("0"));
        assert_eq!(super::bin().rule("0"), Match("0", ""));
        assert_eq!(super::bin().rule("1"), Match("1", ""));
        assert_eq!(super::bin().rule("2"), Expected(Failed::Bin));
    }
}
