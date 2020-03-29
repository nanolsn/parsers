use crate::{
    apply::Apply,
    ruled::Ruled,
    rule::Rule,
};

const SPACE: char = ' ';

#[derive(Copy, Clone, Debug)]
pub struct Space;

pub fn space() -> Rule<Space> { Rule(Space) }

impl<'i> Apply<&'i str> for Space {
    type Err = ();
    type Res = &'i str;

    fn apply(self, input: &'i str) -> Ruled<&'i str, Self::Res, Self::Err> {
        match input.chars().next() {
            Some(SPACE) => input.split_at(SPACE.len_utf8()).into(),
            _ => Ruled::Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::apply::apply;

    #[test]
    fn space() { assert!(apply(super::space(), " ").is_ok()) }
}
