use crate::Parse;

#[derive(Copy, Clone, Debug)]
pub struct Pred<P, F>(pub(crate) P, pub(crate) F);

impl<P, F, A, I> Parse<I> for Pred<P, F>
    where
        P: Parse<I, Out=A>,
        F: Fn(&A) -> bool,
{
    type Err = ();
    type Out = P::Out;

    fn parse(&self, input: I) -> Result<(Self::Out, I), Self::Err> {
        match self.0.parse(input) {
            Ok((out, rest)) => {
                if (self.1)(&out) {
                    Ok((out, rest))
                } else {
                    Err(())
                }
            }
            Err(_) => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::par;

    #[test]
    fn until() {
        let u = (par("@") | "#").pred(|s: &String| s.as_str() == "@");

        assert_eq!(u.parse("@"), Ok(("@".to_string(), "")));
        assert_eq!(u.parse("#"), Err(()));
    }
}
