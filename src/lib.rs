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
mod tuple;

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
}

pub mod rules {
    pub(crate) mod and_then;
    pub(crate) mod cat;
    pub(crate) mod char_range;
    pub(crate) mod fst;
    pub(crate) mod into;
    pub(crate) mod map;
    pub(crate) mod map_err;
    pub(crate) mod not;
    pub(crate) mod opt;
    pub(crate) mod or;
    pub(crate) mod or_default;
    pub(crate) mod or_else;
    pub(crate) mod pred;
    pub(crate) mod range;
    pub(crate) mod ret;
    pub(crate) mod ret_err;
    pub(crate) mod snd;
    pub(crate) mod until;

    pub use and_then::*;
    pub use cat::*;
    pub use char_range::*;
    pub use fst::*;
    pub use into::*;
    pub use map::*;
    pub use map_err::*;
    pub use not::*;
    pub use opt::*;
    pub use or::*;
    pub use or_default::*;
    pub use or_else::*;
    pub use pred::*;
    pub use range::*;
    pub use ret::*;
    pub use ret_err::*;
    pub use snd::*;
    pub use until::*;
}

pub use apply::*;
pub use concat::*;
pub use expected::*;
pub use rule::{rule, Rule};
pub use ruled::*;
