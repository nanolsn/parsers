//! The library of parser combinators.
//!
//! Using [rules], you can express a grammar of any language.
//! Here is [basic] rules such as `latin` or `any`.
//! Also [compound] rules such as `or`, `cat`, `range` and etc.
//!
//! [rules]: ./trait.Rule.html
//! [basic]: ./basic/index.html
//! [compound]: ./compound/index.html

#[macro_use]
mod macros;

mod concat;
mod failed;
mod into_rule;
mod rule;
mod ruled;
mod tuple_impl;

pub use concat::*;
pub use failed::Failed;
pub use into_rule::*;
pub use rule::*;
pub use ruled::*;

pub mod prelude {
    #[doc(no_inline)]
    pub use super::{
        Rule,
        Ruled::{self, *},
        Failed,
        compound::rul,
    };
}

/// Basic rules such as numbers, letters and signs.
pub mod basic {
    // pub(crate) mod any;
    // pub(crate) mod bin;
    // pub(crate) mod dec;
    // pub(crate) mod end;
    // pub(crate) mod hex;
    // pub(crate) mod latin;
    // pub(crate) mod nl;
    // pub(crate) mod oct;
    // pub(crate) mod space;
    // pub(crate) mod white;
    // pub(crate) mod whites;
    //
    // pub use any::{any, Any};
    // pub use bin::{bin, Bin};
    // pub use dec::{dec, Dec};
    // pub use end::{end, End};
    // pub use hex::{hex, Hex};
    // pub use latin::{latin, Latin};
    // pub use nl::{nl, Nl};
    // pub use oct::{oct, Oct};
    // pub use space::{space, Space};
    // pub use white::{white, White};
    // pub use whites::{whites, Whites};
}

/// The compound rules to build complex rules.
pub mod compound {
    // mod and_then;
    mod cat;
    // mod char_range;
    // mod filter;
    mod fst;
    // mod into;
    // mod map;
    // mod map_err;
    // mod not;
    // mod one_of;
    // mod opt;
    mod or;
    // mod or_default;
    // mod or_else;
    // mod range;
    // mod ret;
    // mod ret_err;
    mod rul;
    mod snd;
    // mod until;

    // pub use and_then::*;
    pub use cat::*;
    // pub use char_range::{char_range, CharRange};
    // pub use filter::*;
    pub use fst::*;
    // pub use into::*;
    // pub use map::*;
    // pub use map_err::*;
    // pub use not::*;
    // pub use one_of::*;
    // pub use opt::*;
    pub use or::*;
    // pub use or_default::*;
    // pub use or_else::*;
    // pub use range::*;
    // pub use ret::{ret, Ret};
    // pub use ret_err::{ret_err, RetErr};
    pub use rul::{rul, Rul};
    pub use snd::*;
    // pub use until::*;
}
