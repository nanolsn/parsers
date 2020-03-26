#[macro_use]
mod macros;

pub mod apply;
pub mod concat;
pub mod rule;
pub mod ruled;
pub mod tuple;

pub mod rules {
    pub mod and_then;
    pub mod boxed;
    pub mod cat;
    pub mod char_range;
    pub mod end;
    pub mod fst;
    pub mod map;
    pub mod map_err;
    pub mod not;
    pub mod opt;
    pub mod or;
    pub mod or_default;
    pub mod or_else;
    pub mod pred;
    pub mod range;
    pub mod snd;
}
