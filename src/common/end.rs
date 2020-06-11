use crate::{
    apply::Apply,
    ruled::Ruled,
    rule::Rule,
    expected::Expected,
};

#[derive(Copy, Clone, Debug)]
pub struct End;

pub fn end() -> Rule<End> { Rule(End) }

impl<'i> Apply<&'i str> for End {
    type Err = Expected<'static>;
    type Res = &'i str;

    fn apply(self, input: &'i str) -> Ruled<&'i str, Self::Res, Self::Err> {
        if input.is_empty() {
            Ruled::Match("", "")
        } else {
            Ruled::Expected(Expected::AnyChar)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        apply::apply,
        rule::rule,
    };

    #[test]
    fn end() {
        let r = rule('a') << super::end();
        assert_eq!(apply(r, "a"), Ruled::Match("a", ""));
        assert_eq!(apply(r, "aa"), Ruled::Expected(Expected::AnyChar));
    }
}
