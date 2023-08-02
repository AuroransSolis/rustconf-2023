/// Parses a `<type-bound></type-bound>` section.
///
/// Expected initial call example:
/// ```
/// trait_xml::trait_xml_parse_type_bound! {
///     @parse {
///         input: [
///                         Clone</type-bound>
///                     </type>
///                 </bounds>
///             </trait>
///         ],
///         tokens: [],
///         callback: [
///             name: trait_xml::trait_xml_parse_type,
///             rule: [@tbcallback],
///             args: [
///                 name: [Bar],
///                 bounds: [],
///                 callback: [
///                     name: trait_xml::trait_xml_parse_bounds,
///                     rule: [@typecallback],
///                     args: [
///                         consts: [],
///                         lifetimes: [],
///                         types: [],
///                         reqs: [],
///                         callback: [
///                             name: trait_xml::trait_xml_inner,
///                             rule: [@boundscallback],
///                             args: [
///                                 output: [[name Foo]],
///                             ],
///                         ],
///                     ],
///                 ],
///             ],
///         ],
///     }
/// }
/// ```
#[macro_export]
macro_rules! trait_xml_parse_type_bound {
    // Empty input error
    (
        @parse {
            input: [],
            tokens: [$($tok:tt)*],
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(
            concat!(
                "error parsing type bound: ran out of tokens. caller: `",
                stringify!($callback),
                "`",
            )
        );
    };

    // End type bound success
    (
        @parse {
            input: [</type-bound>$($rest:tt)*],
            tokens: [?$bound:path],
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
                typebound: [?$bound],
            }
        }
    };
    (
        @parse {
            input: [</type-bound>$($rest:tt)*],
            tokens: [$bound:path],
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
                typebound: [$bound],
            }
        }
    };

    // End type bound error - empty path
    (
        @parse {
            input: [</type-bound>$($rest:tt)*],
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
                "error parsing type bound: empty path. caller: `",
                stringify!($callback),
                "`",
            )
        );
    };

    // End type bound error - input tokens do not form path
    (
        @parse {
            input: [</type-bound>$($rest:tt)*],
            tokens: [$($token:tt)+],
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(
            concat!(
                "error parsing type bound: arguments form invalid path. caller: `",
                stringify!($callback),
                "`, args: `",
                $(stringify!($token)),+,
                "`",
            )
        );
    };

    // Unknown end tag
    (
        @parse {
            input: [</$unk:tt$($rest:tt)*],
            tokens: [$($token:tt)+],
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(
            concat!(
                "error parsing type bound: unknown start of end tag `",
                stringify!($unk),
                "`. caller: `",
                stringify!($callback),
                "`",
            )
        );
    };

    // Munch first token
    (
        @parse {
            input: [$first:tt $($rest:tt)*],
            tokens: [$($tok:tt)*],
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        $crate::trait_xml_parse_type_bound! {
            @parse {
                input: [$($rest)*],
                tokens: [$($tok)* $first],
                callback: [
                    name: $callback,
                    rule: $ruletoks,
                    args: $argstoks,
                ],
            }
        }
    };
}
