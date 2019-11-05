pub trait Parse<I> {
    type Err;
    type Out;

    fn parse(&self, input: I) -> Result<(Self::Out, I), Self::Err>;
}

impl<'i> Parse<&'i str> for str {
    type Err = ();
    type Out = &'i str;

    fn parse<'s>(&'s self, input: &'i str) -> Result<(Self::Out, &'i str), Self::Err> {
        if input.starts_with(self) {
            let r = input.split_at(self.len());
            Ok(r)
        } else {
            Err(())
        }
    }
}

impl<'i> Parse<&'i str> for &str {
    type Err = ();
    type Out = &'i str;

    fn parse(&self, input: &'i str) -> Result<(Self::Out, &'i str), Self::Err> {
        Parse::parse(*self, input)
    }
}

impl<'i> Parse<&'i str> for String {
    type Err = ();
    type Out = &'i str;

    fn parse(&self, input: &'i str) -> Result<(Self::Out, &'i str), Self::Err> {
        Parse::parse(self.as_str(), input)
    }
}

impl<'i> Parse<&'i str> for char {
    type Err = ();
    type Out = &'i str;

    fn parse(&self, input: &'i str) -> Result<(Self::Out, &'i str), Self::Err> {
        match input.chars().next() {
            Some(c) if c == *self => {
                let (left, right) = input.split_at(c.len_utf8());
                Ok((left, right))
            }
            _ => Err(()),
        }
    }
}

impl<P1, P2, I, R1, R2, E> Parse<I> for (P1, P2)
    where
        P1: Parse<I, Out=R1, Err=E>,
        P2: Parse<I, Out=R2, Err=E>,
{
    type Err = E;
    type Out = (R1, R2);

    fn parse(&self, input: I) -> Result<((R1, R2), I), E> {
        self.0.parse(input).and_then(
            |(r1, rest)| self.1.parse(rest).map(
                |(r2, rest)| ((r1, r2), rest)
            )
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_str() {
        let t = "a";
        assert_eq!(Parse::parse(t, "a b"), Ok(("a", " b")));
    }

    #[test]
    fn test_parse_string() {
        let s = String::from("z");
        assert_eq!(Parse::parse(&s, "z_x"), Ok(("z", "_x")));
    }

    #[test]
    fn test_parse_char() {
        let f = 'f';
        assert_eq!(f.parse("fg"), Ok(("f", "g")));
    }
}
