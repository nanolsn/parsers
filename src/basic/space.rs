use crate::prelude::*;

const SPACE: char = ' ';

#[derive(Copy, Clone, Debug)]
pub struct Space;

pub fn space() -> Space { Space }

impl<'r, 'i> Rule<'r, &'i str> for Space {
    type Mat = &'i str;
    type Exp = Failed<'static>;

    fn rule(&'r self, input: &'i str) -> Ruled<&'i str, Self::Mat, Self::Exp> {
        match input.chars().next() {
            Some(SPACE) => input.split_at(SPACE.len_utf8()).into(),
            _ => Expected(Failed::Char(SPACE)),
        }
    }
}

impl_ops!(Space);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn space() {
        assert!(super::space().test(" "));
        assert!(!super::space().test("q"));
    }
}
