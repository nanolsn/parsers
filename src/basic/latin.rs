use crate::prelude::*;

#[derive(Copy, Clone, Debug)]
pub struct Latin;

pub fn latin() -> Latin { Latin }

impl<'r, 'i: 'r> Rule<'r, &'i str> for Latin {
    type Mat = &'i str;
    type Exp = Failed<'static>;

    fn rule(&'r self, input: &'i str) -> Ruled<&'i str, Self::Mat, Self::Exp> {
        let c = match input.chars().next() {
            Some(c @ 'a'..='z') => c,
            Some(c @ 'A'..='Z') => c,
            _ => return Ruled::Expected(Failed::Latin),
        };

        input.split_at(c.len_utf8()).into()
    }
}

impl_ops!(Latin);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn latin() {
        assert!(super::latin().test("a"));
        assert!(super::latin().test("b"));
        assert!(super::latin().test("A"));
        assert!(super::latin().test("B"));
        assert!(super::latin().test("q"));
        assert!(super::latin().test("Z"));

        assert!(!super::latin().test(""));
        assert!(!super::latin().test("+"));
        assert!(!super::latin().test("0"));
        assert!(!super::latin().test("🐙"));
        assert!(!super::latin().test("Ш"));
    }
}
