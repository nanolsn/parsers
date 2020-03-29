#[macro_use]
mod macros;

#[cfg(test)]
mod tests {
    // mod json;
}

pub mod apply;
pub mod concat;
pub mod expected;
pub mod rule;
pub mod ruled;
pub mod tuple;

pub mod common {
    pub mod any;
    pub mod bin;
    pub mod dec;
    pub mod end;
    pub mod good_boy;
    pub mod hex;
    pub mod latin;
    pub mod nl;
    pub mod oct;
    pub mod space;
    pub mod white;
}

pub mod rules {
    pub mod and_then;
    pub mod cat;
    pub mod char_range;
    pub mod fst;
    pub mod into;
    pub mod map;
    pub mod map_err;
    pub mod not;
    pub mod opt;
    pub mod or;
    pub mod or_default;
    pub mod or_else;
    pub mod pred;
    pub mod range;
    pub mod ret;
    pub mod ret_err;
    pub mod snd;
    pub mod until;
}
