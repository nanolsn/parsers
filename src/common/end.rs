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
            Ruled::Ok("", "")
        } else {
            Ruled::Err(Expected::AnyChar)
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
        assert_eq!(apply(r, "a"), Ruled::Ok("a", ""));
        assert_eq!(apply(r, "aa"), Ruled::Err(Expected::AnyChar));
    }
}
