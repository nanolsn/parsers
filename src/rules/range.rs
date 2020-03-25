use crate::{
    apply::Apply,
    ruled::Ruled,
};

#[derive(Copy, Clone, Debug)]
pub struct Range<R> {
    pub(crate) rule: R,
    pub(crate) from: usize,
    pub(crate) to: Option<usize>,
}

impl<R, I> Apply<I> for Range<R>
    where
        R: Apply<I>,
        I: Copy,
        R::Res: AsRef<str>,
{
    type Err = R::Err;
    type Res = String;

    fn apply(&self, mut input: I) -> Ruled<I, Self::Res, Self::Err> {
        let mut count = 0;
        let mut res = String::new();

        loop {
            if self.to.is_some() && count >= self.to.unwrap() {
                break Ruled::Ok(res, input);
            }

            match self.rule.apply(input) {
                Ruled::Ok(r, i) => {
                    count += 1;
                    input = i;
                    res.push_str(r.as_ref());
                }
                Ruled::Err(e) => {
                    break if count >= self.from {
                        Ruled::Ok(res, input)
                    } else {
                        Ruled::Err(e)
                    };
                }
            }
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
    fn range() {
        let r = rule("a") * (1..3);
        assert_eq!(apply(r, "~"), Ruled::Err(()));
        assert_eq!(apply(r, "a"), Ruled::Ok("a".to_owned(), ""));
        assert_eq!(apply(r, "aa"), Ruled::Ok("aa".to_owned(), ""));
        assert_eq!(apply(r, "aaa"), Ruled::Ok("aa".to_owned(), "a"));

        let r = rule("a") * (0..3);
        assert_eq!(apply(r, "~"), Ruled::Ok("".to_owned(), "~"));
        assert_eq!(apply(r, "a"), Ruled::Ok("a".to_owned(), ""));
        assert_eq!(apply(r, "aa"), Ruled::Ok("aa".to_owned(), ""));
        assert_eq!(apply(r, "aaa"), Ruled::Ok("aa".to_owned(), "a"));
    }

    #[test]
    fn range_inclusive() {
        let r = rule("a") * (0..=0);
        assert_eq!(apply(r, "."), Ruled::Ok("".to_owned(), "."));
        assert_eq!(apply(r, "a"), Ruled::Ok("".to_owned(), "a"));

        let r = rule("a") * (0..=2);
        assert_eq!(apply(r, "~"), Ruled::Ok("".to_owned(), "~"));
        assert_eq!(apply(r, "a"), Ruled::Ok("a".to_owned(), ""));
        assert_eq!(apply(r, "aa"), Ruled::Ok("aa".to_owned(), ""));
        assert_eq!(apply(r, "aaa"), Ruled::Ok("aa".to_owned(), "a"));
    }

    #[test]
    fn range_to() {
        let r = rule("a") * ..2;
        assert_eq!(apply(r, "~"), Ruled::Ok("".to_owned(), "~"));
        assert_eq!(apply(r, "a"), Ruled::Ok("a".to_owned(), ""));
        assert_eq!(apply(r, "aa"), Ruled::Ok("a".to_owned(), "a"));
    }

    #[test]
    fn range_to_inclusive() {
        let r = rule("a") * ..=1;
        assert_eq!(apply(r, "~"), Ruled::Ok("".to_owned(), "~"));
        assert_eq!(apply(r, "a"), Ruled::Ok("a".to_owned(), ""));
        assert_eq!(apply(r, "aa"), Ruled::Ok("a".to_owned(), "a"));
    }

    #[test]
    fn range_from() {
        let r = rule("a") * (2..);
        assert_eq!(apply(r, ""), Ruled::Err(()));
        assert_eq!(apply(r, "a"), Ruled::Err(()));
        assert_eq!(apply(r, "aa"), Ruled::Ok("aa".to_owned(), ""));
        assert_eq!(apply(r, "aaa"), Ruled::Ok("aaa".to_owned(), ""));
    }

    #[test]
    fn range_full() {
        let r = rule("a") * ..;
        assert_eq!(apply(r, ""), Ruled::Ok("".to_owned(), ""));
        assert_eq!(apply(r, "~"), Ruled::Ok("".to_owned(), "~"));
        assert_eq!(apply(r, "a"), Ruled::Ok("a".to_owned(), ""));
        assert_eq!(apply(r, "aa"), Ruled::Ok("aa".to_owned(), ""));
        assert_eq!(apply(r, "aaa"), Ruled::Ok("aaa".to_owned(), ""));
    }
}
