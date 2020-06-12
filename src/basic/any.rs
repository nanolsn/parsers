use crate::prelude::*;

#[derive(Copy, Clone, Debug)]
pub struct Any;

pub fn any() -> Any { Any }

impl<'r, 'i: 'r> Rule<'r, &'i str> for Any {
    type Mat = &'i str;
    type Exp = Failed<'static>;

    fn rule(&'r self, input: &'i str) -> Ruled<&'i str, Self::Mat, Self::Exp> {
        match input.chars().next() {
            None => Expected(Failed::AnyChar),
            Some(c) => input.split_at(c.len_utf8()).into(),
        }
    }
}

impl_ops!(Any);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn any() {
        assert!(super::any().test("q"));
        assert_eq!(super::any().rule("!@#$"), Match("!", "@#$"));
        assert_eq!(super::any().rule(""), Expected(Failed::AnyChar));
    }
}
