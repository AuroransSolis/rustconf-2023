/// Parses a `<req></req>` section.
///
/// Expected initial call example:
/// ```
/// # trait Bar {}
/// trait_xml::trait_xml_parse_supertrait! {
///     @parse {
///         input: [
///                     Bar</req>
///                 </bounds>
///             </trait>
///         ],
///         tokens: [],
///         callback: [
///             name: trait_xml::trait_xml_parse_bounds,
///             rule: [@reqcallback],
///             args: [
///                 consts: [],
///                 lifetimes: [],
///                 types: [],
///                 reqs: [],
///                 callback: [
///                         name: trait_xml::trait_xml_inner,
///                         rule: [@boundscallback],
///                         args: [output: [[name Foo]],],
///                 ],
///             ],
///         ],
///     }
/// }
/// ```
#[macro_export]
macro_rules! trait_xml_parse_supertrait {
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
                "error parsing supertrait: ran out of tokens. caller: `",
                stringify!($callback),
                "`",
            )
        );
    };

    // End supertrait success
    (
        @parse {
            input: [</req>$($rest:tt)*],
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
                supertrait: [$bound],
            }
        }
    };
    (
        @parse {
            input: [</req>$($rest:tt)*],
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
                supertrait: [?$bound],
            }
        }
    };

    // End supertrait error - empty path
    (
        @parse {
            input: [</req>$($rest:tt)*],
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
                "error parsing supertrait: empty path. caller: `",
                stringify!($callback),
                "`",
            )
        );
    };

    // End supertrait error - input tokens do not form path
    (
        @parse {
            input: [</req>$($rest:tt)*],
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
                "error parsing supertrait: arguments form invalid path. caller: `",
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
                "error parsing supertrait: unknown start of end tag `",
                stringify!($unk),
                "`. caller: `",
                stringify!($callback),
                "`, rest: ",
                $(stringify!($rest)),*,
                ", cbargs: ",
                stringify!($argstoks),
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
        $crate::trait_xml_parse_supertrait! {
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
