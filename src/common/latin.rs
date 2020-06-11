use crate::{
    apply::Apply,
    ruled::Ruled,
    rule::Rule,
    expected::Expected,
};

#[derive(Copy, Clone, Debug)]
pub struct Latin;

pub fn latin() -> Rule<Latin> { Rule(Latin) }

impl<'i> Apply<&'i str> for Latin {
    type Err = Expected<'static>;
    type Res = &'i str;

    fn apply(self, input: &'i str) -> Ruled<&'i str, Self::Res, Self::Err> {
        let c = match input.chars().next() {
            Some(c @ 'a'..='z') => c,
            Some(c @ 'A'..='Z') => c,
            _ => return Ruled::Expected(Expected::Latin),
        };

        input.split_at(c.len_utf8()).into()
    }
}

#[cfg(test)]
mod tests {
    use crate::apply::apply;

    #[test]
    fn latin() {
        assert!(apply(super::latin(), "a").is_ok());
        assert!(apply(super::latin(), "b").is_ok());
        assert!(apply(super::latin(), "A").is_ok());
        assert!(apply(super::latin(), "B").is_ok());
        assert!(apply(super::latin(), "q").is_ok());
        assert!(apply(super::latin(), "Z").is_ok());

        assert!(apply(super::latin(), "").is_err());
        assert!(apply(super::latin(), "+").is_err());
        assert!(apply(super::latin(), "0").is_err());
        assert!(apply(super::latin(), "🐙").is_err());
        assert!(apply(super::latin(), "Ш").is_err());
    }
}
