macro_rules! impl_tuple {
    ( $t:ident, $($ts:ident),* ; $($vs:ident),+ ) => {
        impl<'p, $t, $($ts,)* > Parse<'p> for ( $t, $($ts,)* )
            where
                $t: Parse<'p>,
                $($ts: Parse<'p, Err=$t::Err, On=$t::On>,)*
        {
            type Res = ( $t::Res, $($ts::Res,)* );
            type Err = $t::Err;
            type On = $t::On;

            fn parse(&self, rest: Self::On) -> Result<(Self::Res, Self::On), Self::Err> {
                let ($($vs,)+) = self;

                $(
                    let ($vs, rest) = $vs.parse(rest)?;
                )+

                Ok((( $($vs,)+ ), rest))
            }
        }
    };
}
