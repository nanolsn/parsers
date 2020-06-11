use crate::{
    rule::Rule,
    ruled::Ruled,
    rul::Rul,
    some_of::SomeOf,
};

#[derive(Copy, Clone, Debug)]
pub struct Latin;

pub fn latin() -> Rul<Latin> { Rul(Latin) }

impl<'i> Rule<&'i str> for Latin {
    type Exp = SomeOf<'static>;
    type Mat = &'i str;

    fn rule(self, input: &'i str) -> Ruled<&'i str, Self::Res, Self::Err> {
        let c = match input.chars().next() {
            Some(c @ 'a'..='z') => c,
            Some(c @ 'A'..='Z') => c,
            _ => return Ruled::Expected(SomeOf::Latin),
        };

        input.split_at(c.len_utf8()).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn latin() {
        assert!(super::latin().rule("a").is_match());
        assert!(super::latin().rule("b").is_match());
        assert!(super::latin().rule("A").is_match());
        assert!(super::latin().rule("B").is_match());
        assert!(super::latin().rule("q").is_match());
        assert!(super::latin().rule("Z").is_match());

        assert!(super::latin().rule("").is_expected());
        assert!(super::latin().rule("+").is_expected());
        assert!(super::latin().rule("0").is_expected());
        assert!(super::latin().rule("🐙").is_expected());
        assert!(super::latin().rule("Ш").is_expected());
    }
}
