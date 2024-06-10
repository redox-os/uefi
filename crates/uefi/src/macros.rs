macro_rules! enum_try_from_int {
    (
        #[repr($T: ident)]
        $( #[$meta: meta] )*
        $vis: vis enum $Name: ident {
            $(
                $Variant: ident = $value: expr
            ),*
            $( , )?
        }
    ) => {
        #[repr($T)]
        $( #[$meta] )*
        $vis enum $Name {
            $(
                $Variant = $value
            ),*
        }

        impl core::convert::TryFrom<$T> for $Name {
            type Error = ();

            fn try_from(value: $T) -> core::result::Result<$Name, ()> {
                match value {
                    $(
                        $value => Ok($Name::$Variant),
                    )*
                    _ => Err(())
                }
            }
        }
    }
}
