macro_rules! impl_tuple {
    ( $t:ident, $($ts:ident),* ; $($vs:ident),+ ) => {
        impl<I, $t, $($ts,)* > Parse<I> for ( $t, $($ts,)* )
            where
                $t: Parse<I>,
                $($ts: Parse<I, Err=$t::Err>,)*
        {
            type Err = $t::Err;
            type Out = ( $t::Out, $($ts::Out,)* );

            fn parse(&self, rest: I) -> Result<(Self::Out, I), Self::Err> {
                let ($($vs,)+) = self;

                $(
                    let ($vs, rest) = $vs.parse(rest)?;
                )+

                Ok((( $($vs,)+ ), rest))
            }
        }
    };
}
