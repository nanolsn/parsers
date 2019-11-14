mod comply;
mod rule;
mod parser;

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
    pub mod map;
    pub mod map_err;
    pub mod range;
    pub mod range_vec;
    pub mod opt;
    pub mod boxed;
    pub mod pred;

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
    pub use map::Map;
    pub use map_err::MapErr;
    pub use range::Range;
    pub use range_vec::RangeVec;
    pub use opt::Opt;
    pub use boxed::{BoxedRule, boxed};
    pub use pred::Pred;
}

pub use rules::*;
pub use comply::Comply;
pub use parser::*;
pub use rule::{rule, Rule};
