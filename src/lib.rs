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
    pub mod concat;
    pub mod any;
    pub mod until;
}

pub use parsers::{
    parser::{par, Parser},
    list_parser::{ListParser, HeadParser},
    second::Second,
    first::First,
    or_parser::OrParser,
    repeat::Repeat,
    range::Range,
    concat::Concat,
    any::{Any, ANY},
    until::Until,
};

pub use parse::Parse;

#[macro_export]
macro_rules! match_this {
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

    #[test]
    fn test() {
        let digit = par(match_this!('0'..='9'));
        let space = par(' ') * ..;
        let float = (space >> (digit * (1..)) & '.' & (digit * ..))
            .map(|s: String| s.as_str().parse::<f32>().unwrap());

        assert_eq!(float.parse_result("  12.45"), Ok(12.45));
    }
}
