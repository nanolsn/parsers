use crate::prelude::*;

/// Match a whitespace character.
///
/// Match one of ` `, `\n`, `\r`, `\t`, `\r\n`.
///
/// # Examples
///
/// ```
/// # use parsers::{prelude::*, basic::white};
/// assert!(white().test(" "));
/// assert!(white().test("\n"));
/// assert!(!white().test("A"));
/// ```
#[derive(Copy, Clone, Debug)]
pub struct White;

/// Constructor of [`White`]
///
/// [`White`]: ./struct.White.html
pub fn white() -> White { White }

impl<'r, 'i> Rule<'r, &'i str> for White {
    type Mat = &'i str;
    type Exp = Failed<'static>;

    fn rule(&'r self, input: &'i str) -> Ruled<&'i str, Self::Mat, Self::Exp> {
        let nl = "\r\n";
        if input.starts_with(nl) {
            return input.split_at(nl.len()).into();
        }

        let c = match input.chars().next() {
            Some(c @ ' ') => c,
            Some(c @ '\n') => c,
            Some(c @ '\r') => c,
            Some(c @ '\t') => c,
            _ => return Expected(Failed::White),
        };

        input.split_at(c.len_utf8()).into()
    }
}

impl_ops!(White);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn white() {
        assert!(super::white().test(" "));
        assert!(super::white().test("\n"));
        assert!(super::white().test("\t"));
        assert!(!super::white().test("!"));
    }
}
