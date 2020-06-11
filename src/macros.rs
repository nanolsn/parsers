macro_rules! impl_ops {
    ($t:ident< $($g:ident),+ >) => {
        impl< $($g),+ , T> std::ops::BitAnd<T> for $t< $($g),+ > {
            type Output = crate::compound::Cat<$t< $($g),+ >, T, &'static str>;

            fn bitand(self, rhs: T) -> Self::Output { crate::compound::Cat::new(self, rhs) }
        }

        impl< $($g),+ , T> std::ops::Add<T> for $t< $($g),+ > {
            type Output = crate::compound::Cat<$t< $($g),+ >, T, String>;

            fn add(self, rhs: T) -> Self::Output { crate::compound::Cat::new(self, rhs) }
        }

        impl_or!($t< $($g),+ >);

        impl_shifts!($t< $($g),+ >);
    };
}

macro_rules! impl_or {
    ($t:ident< $($g:ident),+ >) => {
        impl< $($g),+ , T> std::ops::BitOr<T> for $t< $($g),+ > {
            type Output = crate::compound::Or<$t< $($g),+ >, T>;

            fn bitor(self, rhs: T) -> Self::Output { crate::compound::Or(self, rhs) }
        }
    };
}

macro_rules! impl_shifts {
    ($t:ident< $($g:ident),+ >) => {
        impl< $($g),+ , T> std::ops::Shl<T> for $t< $($g),+ > {
            type Output = crate::compound::Fst<$t< $($g),+ >, T>;

            fn shl(self, rhs: T) -> Self::Output { crate::compound::Fst(self, rhs) }
        }

        impl< $($g),+ , T> std::ops::Shr<T> for $t< $($g),+ > {
            type Output = crate::compound::Snd<$t< $($g),+ >, T>;

            fn shr(self, rhs: T) -> Self::Output { crate::compound::Snd(self, rhs) }
        }
    };
}
