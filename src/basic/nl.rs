use crate::prelude::*;

#[derive(Copy, Clone, Debug)]
pub struct Nl;

pub fn nl() -> Nl { Nl }

impl<'r, 'i: 'r> Rule<'r, &'i str> for Nl {
    type Mat = &'i str;
    type Exp = Failed<'static>;

    fn rule(&'r self, input: &'i str) -> Ruled<&'i str, Self::Mat, Self::Exp> {
        let nl = "\r\n";
        if input.starts_with(nl) {
            return input.split_at(nl.len()).into();
        }

        match input.chars().next() {
            Some(c @ '\n') => input.split_at(c.len_utf8()).into(),
            Some(c @ '\r') => input.split_at(c.len_utf8()).into(),
            _ => return Expected(Failed::Nl),
        }
    }
}

impl_ops!(Nl);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nl() {
        assert!(super::nl().test("\n"));
        assert!(super::nl().test("\r"));
        assert!(super::nl().test("\r\n"));
        assert!(!super::nl().test("~"));
    }
}
