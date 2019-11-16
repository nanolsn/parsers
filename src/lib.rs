mod comply;
mod parser;
mod rule;
mod rule_type;
mod common;

mod rules {
    pub mod or;
    pub mod concat;
    pub mod not;
    pub mod ret;
    pub mod ret_err;
    pub mod string_res;
    pub mod first;
    pub mod second;
    pub mod and_then;
    pub mod or_else;
    pub mod any;
    pub mod any_pred;
    pub mod map;
    pub mod map_err;
    pub mod range;
    pub mod range_vec;
    pub mod opt;
    pub mod or_empty;
    pub mod boxed;
    pub mod pred;
    pub mod until;
    pub mod until_vec;

    pub use or::Or;
    pub use concat::Concat;
    pub use not::Not;
    pub use ret::Ret;
    pub use ret_err::RetErr;
    pub use string_res::{StringRes, string_res};
    pub use first::First;
    pub use second::Second;
    pub use and_then::AndThen;
    pub use or_else::OrElse;
    pub use any::{any, Any};
    pub use any_pred::{any_pred, AnyPred};
    pub use map::Map;
    pub use map_err::MapErr;
    pub use range::Range;
    pub use range_vec::RangeVec;
    pub use opt::Opt;
    pub use or_empty::OrEmpty;
    pub use boxed::{BoxedRule, boxed};
    pub use pred::Pred;
    pub use until::Until;
    pub use until_vec::UntilVec;
}

pub use rules::*;
pub use comply::Comply;
pub use parser::*;
pub use rule::{rule, Rule};
pub use common::*;

#[cfg(test)]
mod tests {
    mod obj;
}
