//! The library of parser combinators.
//!
//! Using [rules], you can express a grammar of any language.
//! Here is [common] rules such as `latin` or `any`.
//! Also rule [combinators] such as `or`, `cat`, `range` and etc.
//!
//! [rules]: ./trait.Apply.html
//! [common]: ./common/index.html
//! [combinators]: ./rules/index.html

#[macro_use]
mod macros;

#[cfg(test)]
mod tests {
    mod json;
}

mod apply;
mod concat;
mod expected;
mod rule;
mod ruled;
mod tuple_impl;

/// The common rules such as numbers, letters and signs
pub mod common {
    pub(crate) mod any;
    pub(crate) mod bin;
    pub(crate) mod dec;
    pub(crate) mod end;
    pub(crate) mod good_boy;
    pub(crate) mod hex;
    pub(crate) mod latin;
    pub(crate) mod nl;
    pub(crate) mod oct;
    pub(crate) mod space;
    pub(crate) mod white;
    pub(crate) mod whites;

    pub use any::{any, Any};
    pub use bin::{bin, Bin};
    pub use dec::{dec, Dec};
    pub use end::{end, End};
    pub use good_boy::{good_boy, GoodBoy, Gender};
    pub use hex::{hex, Hex};
    pub use latin::{latin, Latin};
    pub use nl::{nl, Nl};
    pub use oct::{oct, Oct};
    pub use space::{space, Space};
    pub use white::{white, White};
    pub use whites::{whites, Whites};
}

/// The rule combinators to build complex rules.
pub mod rules {
    mod and_then;
    mod cat;
    mod char_range;
    mod fst;
    mod into;
    mod map;
    mod map_err;
    mod not;
    mod one_of;
    mod opt;
    mod or;
    mod or_default;
    mod or_else;
    mod pred;
    mod range;
    mod ret;
    mod ret_err;
    mod snd;
    mod until;

    pub use and_then::*;
    pub use cat::*;
    pub use char_range::{char_range, CharRange};
    pub use fst::*;
    pub use into::*;
    pub use map::*;
    pub use map_err::*;
    pub use not::*;
    pub use one_of::*;
    pub use opt::*;
    pub use or::*;
    pub use or_default::*;
    pub use or_else::*;
    pub use pred::*;
    pub use range::*;
    pub use ret::{ret, Ret};
    pub use ret_err::{ret_err, RetErr};
    pub use snd::*;
    pub use until::*;
}

pub use apply::*;
pub use concat::*;
pub use expected::*;
pub use rule::{rule, Rule};
pub use ruled::*;
