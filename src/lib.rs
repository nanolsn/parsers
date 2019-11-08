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
    pub mod pred;
}

pub use parsers::{
    parser::{par, stringed_par, Parser},
    list_parser::{ListParser, HeadParser},
    second::Second,
    first::First,
    or_parser::OrParser,
    repeat::Repeat,
    range::Range,
    range_vec::RangeVec,
    concat::Concat,
    any::{Any, ANY, any},
    until::Until,
    until_vec::UntilVec,
    pred::Pred,
};

pub use parse::Parse;
pub use parse::PredWrapper;

#[macro_export]
macro_rules! pattern {
    ($p:pat) => {
        PredWrapper(|a| match a {
            $p => true,
            _ => false,
        })
    };
}

mod tests;
