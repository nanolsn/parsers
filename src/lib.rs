#[macro_use]
mod impl_tuple;
mod parse;
mod maps;
mod template;

mod parsers {
    pub mod parser;
    pub mod list_parser;
    pub mod second;
    pub mod first;
    pub mod or_parser;
    pub mod repeat;
    pub mod repeat_vec;
    pub mod range;
    pub mod range_vec;
    pub mod concat;
    pub mod any;
    pub mod until;
    pub mod until_vec;
    pub mod pred;
    pub mod not;
    pub mod opt;
    pub mod boxed;
}

pub use parsers::{
    parser::{par, stringed_par, Parser},
    list_parser::{ListParser, HeadParser},
    second::Second,
    first::First,
    or_parser::OrParser,
    repeat::Repeat,
    repeat_vec::RepeatVec,
    range::Range,
    range_vec::RangeVec,
    concat::Concat,
    any::{Any, ANY, any},
    until::Until,
    until_vec::UntilVec,
    pred::Pred,
    not::Not,
    opt::Opt,
    boxed::Boxed,
};

pub use parse::Parse;
pub use parse::{PredFn, pred_fn};
pub use template::{
    Template,
    template,
    BoundTemplate,
};

#[macro_export]
macro_rules! pattern {
    ($p:pat) => {
        |a| match a {
            $p => true,
            _ => false,
        }
    };
}

mod tests;
