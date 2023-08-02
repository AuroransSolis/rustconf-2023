/// Parses a `<type></type>` section where a `:ty` token is expected to be in the middle.
///
/// Expected initial call example:
/// ```
/// trait_xml::trait_xml_parse_type_ty! {
///     @parse {
///         input: [
///                     u8</type>
///                 </assocconst>
///             </trait>
///         ],
///         type: ,
///         tokens: [],
///         callback: [
///             name: trait_xml::trait_xml_parse_assoc_const,
///             rule: [@typecallback],
///             args: [
///                 name: [Foo],
///                 default: [],
///                 callback: [
///                     name: trait_xml::trait_xml_inner,
///                     rule: [@accallback],
///                     args: [
///                         output: [[name Foo]],
///                     ],
///                 ],
///             ],
///         ],
///     }
/// }
/// ```
#[macro_export]
macro_rules! trait_xml_parse_type_ty {
    (
        @parse {
            input: [],
            type: ,
            tokens: $typetoks:tt,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(
            concat!(
                "error parsing type: ran out of tokens. caller: `",
                stringify!($callback),
                "`",
            )
        );
    };
    (
        @parse {
            input: [</type>$($rest:tt)*],
            type: ,
            tokens: [],
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(
            concat!(
                "error parsing type: empty type tags. caller: `",
                stringify!($callback),
                "`",
            )
        );
    };
    (
        @parse {
            input: [</type>$($rest:tt)*],
            type: ,
            tokens: [$type:ty],
            callback: [
                name: $callback:path,
                rule: [$($rule:tt)+],
                args: [$($field:tt: $fieldtokens:tt,)+],
            ],
        }
    ) => {
        $callback! {
            $($rule)+ {
                input: [$($rest)*],
                $($field: $fieldtokens,)+
                type: $type,
            }
        }
    };
    (
        @parse {
            input: [</type>$($rest:tt)*],
            type: ,
            tokens: [$($inv:tt)+],
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(
            concat!(
                "error parsing type: tokens `",
                $(stringify!($inv)),+,
                "` form invalid path. caller: `",
                stringify!($callback),
                "`",
            )
        );
    };
    (
        @parse {
            input: [$first:tt$($rest:tt)*],
            type: ,
            tokens: [$($tytok:tt)*],
            callback: $callbacktoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_type_ty! {
            @parse {
                input: [$($rest)*],
                type: ,
                tokens: [$($tytok)* $first],
                callback: $callbacktoks,
            }
        }
    };
}
