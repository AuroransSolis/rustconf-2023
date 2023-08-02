/// Parses a `<lifetime></lifetime>` section.
///
/// Expected initial call example:
/// ```
/// trait_xml::trait_xml_parse_lifetime! {
///     @parse {
///         input: [
///                         <name>'bar</name>
///                     </lifetime>
///                 </bounds>
///             </trait>
///         ],
///         lifetime: ,
///         bounds: [],
///         callback: [
///             name: trait_xml::trait_xml_parse_bounds,
///             rule: [@lifetimecallback],
///             args: [
///                 consts: [],
///                 lifetimes: [],
///                 types: [],
///                 reqs: [],
///                 callback: [
///                     name: trait_xml::trait_xml_inner,
///                     rule: [@boundscallback],
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
macro_rules! trait_xml_parse_lifetime {
    // Empty input error
    (
        @parse {
            input: [],
            lifetime: $($lt:lifetime)?,
            bounds: $boundstoks:tt,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argtoks:tt,
            ],
        }
    ) => {
        compile_error!(
            concat!(
                "error parsing lifetime parameter: ran out of tokens. caller: `",
                stringify!($callback),
                "`",
            )
        );
    };

    // End of input - no lifetime
    (
        @parse {
            input: [</lifetime>$($rest:tt)*],
            lifetime: ,
            bounds: $boundstoks:tt,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(
            concat!(
                "error parsing lifetime parameter: missing lifetime. caller: `",
                stringify!($callback),
                "`",
            )
        );
    };

    // End of input - success
    (
        @parse {
            input: [</lifetime>$($rest:tt)*],
            lifetime: $lt:lifetime,
            bounds: $boundstoks:tt,
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
                lifetime: [lt $lt $boundstoks],
            }
        }
    };

    // Parse LT name - broken up so that proper error handling can be provided.
    // 1: found lifetime, lifetime already provided
    (
        @parse {
            input: [<name>$($rest:tt)*],
            lifetime: $lt:lifetime,
            bounds: $boundstoks:tt,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(
            concat!(
                "error parsing lifetime parameter: multiple `<lifetime>`s defined. ",
                "first defined as: `",
                stringify!($lt),
                "`, caller: `",
                stringify!($callback),
                "`",
            )
        );
    };
    // 2: found lifetime tag, no lifetime defined yet
    (
        @parse {
            input: [<name>$($rest:tt)*],
            lifetime: ,
            bounds: $boundstoks:tt,
            callback: $callbacktoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_lifetime! {
            @parselt {
                input: [$($rest)*],
                lifetime: ,
                bounds: $boundstoks,
                callback: $callbacktoks,
            }
        }
    };
    // 2.1: unexpected end of input
    (
        @parselt {
            input: [],
            lifetime: ,
            bounds: $boundstoks:tt,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argtoks:tt,
            ],
        }
    ) => {
        compile_error!(
            concat!(
                "error parsing lifetime parameter: expected lifetime, found end of input. ",
                "caller: `",
                stringify!($callback),
                "`",
            )
        );
    };
    // 2.2: valid lifetime
    (
        @parselt {
            input: [$lt:lifetime $($rest:tt)*],
            lifetime: ,
            bounds: $boundstoks:tt,
            callback: $callbacktoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_lifetime! {
            @parseltend {
                input: [$($rest)*],
                lifetime: $lt,
                bounds: $boundstoks,
                callback: $callbacktoks,
            }
        }
    };
    // 2.3: invalid lifetime
    (
        @parselt {
            input: [$unx:tt$($rest:tt)*],
            lifetime: ,
            bounds: $boundstoks:tt,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argtoks:tt,
            ],
        }
    ) => {
        compile_error!(
            concat!(
                "error parsing lifetime parameter: expected lifetime, found `",
                stringify!($unx),
                "`. caller: `",
                stringify!($callback),
                "`",
            )
        );
    };
    // 3.1: found end name tag
    (
        @parseltend {
            input: [</name>$($rest:tt)*],
            lifetime: $lt:lifetime,
            bounds: $boundstoks:tt,
            callback: $callbacktoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_lifetime! {
            @parse {
                input: [$($rest)*],
                lifetime: $lt,
                bounds: $boundstoks,
                callback: $callbacktoks,
            }
        }
    };
    // 3.2: unexpected end of input
    (
        @parseltend {
            input: [],
            lifetime: $lt:lifetime,
            bounds: $boundstoks:tt,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(
            concat!(
                "error parsing lifetime parameter: expected `</lifetime>`, found end of input. ",
                "caller: `",
                stringify!($callback),
                "`",
            )
        );
    };
    // 3.3: unexpected token instead of </name>
    (
        @parsenameend {
            input: [$unx:tt $($rest:tt)*]
            lifetime: $lt:lifetime,
            bounds: $boundstoks:tt,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(
            concat!(
                "error parsing lifetime parameter: expected `</name>`, found `",
                stringify!($unx),
                "`. caller: `",
                stringify!($callback),
                "`",
            )
        );
    };

    (
        @parse {
            input: [<lifetime-bound>$($rest:tt)*],
            lifetime: $($lt:lifetime)?,
            bounds: $boundstoks:tt,
            callback: $callbacktoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_lifetime_bound! {
            @parse {
                input: [$($rest)*],
                lifetime: ,
                callback: [
                    name: $crate::trait_xml_parse_lifetime,
                    rule: [@lbcallback],
                    args: [
                        lifetime: [$($lt)?],
                        bounds: $boundstoks,
                        callback: $callbacktoks,
                    ],
                ],
            }
        }
    };

    (
        @parse {
            input: [</$unk:tt$($rest:tt)*],
            lifetime: $($lt:lifetime)?,
            bounds: $boundstoks:tt,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(
            concat!(
                "error parsing lifetime parameter: start of unknown end tag `",
                stringify!($unk),
                "`. caller: `",
                stringify!($callback),
                "`",
            )
        );
    };
    (
        @parse {
            input: [<$unk:tt$($rest:tt)*],
            lifetime: $($lt:lifetime)?,
            bounds: $boundstoks:tt,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(
            concat!(
                "error parsing lifetime parameter: start of unknown start tag `",
                stringify!($unk),
                "`. caller: `",
                stringify!($callback),
                "`",
            )
        );
    };
    (
        @parse {
            input: [$unx:tt$($rest:tt)*],
            lifetime: $($lt:lifetime)?,
            bounds: $boundstoks:tt,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(
            concat!(
                "error parsing lifetime parameter: unexpected token `",
                stringify!($unx),
                "`. caller: `",
                stringify!($callback),
                "`",
            )
        );
    };

    (
        @lbcallback {
            input: $inputtoks:tt,
            lifetime: [$($lt:lifetime)?],
            bounds: [$($bound:lifetime)*],
            callback: $callbacktoks:tt,
            ltbound: $ltbound:lifetime,
        }
    ) => {
        $crate::trait_xml_parse_lifetime! {
            @parse {
                input: $inputtoks,
                lifetime: $($lt)?,
                bounds: [$($bound)* $ltbound],
                callback: $callbacktoks,
            }
        }
    };
}
