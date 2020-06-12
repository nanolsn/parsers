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

mod char_range;
mod concat;
mod failed;
mod into_rule;
mod rule;
mod ruled;
mod tuple_impl;

pub use char_range::*;
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
        char_range::char_range,
    };
}

/// Basic rules such as numbers, letters and signs.
pub mod basic {
    // mod any;
    // mod bin;
    // mod dec;
    // mod end;
    // mod hex;
    // mod latin;
    // mod nl;
    // mod oct;
    // mod space;
    // mod white;
    // mod whites;
    //
    // pub use any::*;
    // pub use bin::*;
    // pub use dec::*;
    // pub use end::*;
    // pub use hex::*;
    // pub use latin::*;
    // pub use nl::*;
    // pub use oct::*;
    // pub use space::*;
    // pub use white::*;
    // pub use whites::*;
}

/// The compound rules to build complex rules.
pub mod compound {
    mod cat;
    mod filter;
    mod fst;
    mod map;
    mod map_exp;
    mod not;
    mod one_of;
    mod opt;
    mod or;
    mod or_default;
    mod range;
    mod ret;
    mod ret_exp;
    mod rul;
    mod snd;
    mod to;
    mod until;

    pub use cat::*;
    pub use filter::*;
    pub use fst::*;
    pub use map::*;
    pub use map_exp::*;
    pub use not::*;
    pub use one_of::*;
    pub use opt::*;
    pub use or::*;
    pub use or_default::*;
    pub use range::*;
    pub use ret::{ret, Ret};
    pub use ret_exp::{ret_exp, RetExp};
    pub use rul::{rul, Rul};
    pub use snd::*;
    pub use to::*;
    pub use until::*;
}
