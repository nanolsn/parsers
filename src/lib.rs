#[macro_use]
mod impl_tuple;
mod parse;
mod maps;

mod parsers {
    pub mod parser;
    pub mod list_parser;
    pub mod second;
    pub mod first;
    pub mod or_parser;
    pub mod repeat;
    pub mod range;
    pub mod range_vec;
    pub mod concat;
    pub mod any;
    pub mod until;
    pub mod until_vec;
}

pub use parsers::{
    parser::{par, Parser},
    list_parser::{ListParser, HeadParser},
    second::Second,
    first::First,
    or_parser::OrParser,
    repeat::Repeat,
    range::Range,
    range_vec::RangeVec,
    concat::Concat,
    any::{Any, ANY},
    until::Until,
    until_vec::UntilVec,
};

pub use parse::Parse;

#[macro_export]
macro_rules! pattern {
    ($p:pat) => {
        |a| match a {
            $p => true,
            _ => false,
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialOrd, PartialEq)]
    enum Var {
        Number(u32),
        Float(f32),
        Word(String),
    }

    use Var::*;

    #[test]
    fn test() {
        let space = par(' ') * ..;
        let word = (par(pattern!('a'..='z')) | pattern!('A'..='Z')) * (1..);
        let digit = par(pattern!('0'..='9'));
        let num = digit * (1..);
        let float = num & '.' & (digit * ..);

        let var = float.map(|f: String| Float(f.as_str().parse::<f32>().unwrap()))
            | num.map(|n: String| Number(n.as_str().parse::<u32>().unwrap()))
            | word.map(|w| Word(w));

        let code = (space >> var) ^ ..;

        assert_eq!(
            code.parse_result("  12. 42qwe"),
            Ok(vec![
                Float(12.0),
                Number(42),
                Word("qwe".to_string()),
            ])
        );
    }
}
