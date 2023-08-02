/// Parses a `<lifetime-clause></lifetime-clause>` section.
///
/// Expected initial call example:
/// ```
/// trait_xml::trait_xml_parse_lifetime_clause! {
///     @parse {
///         input: [
///                         <lifetime>'bar</lifetime>
///                         <lifetime-bound>'baz</lifetime-bound>
///                     </lifetime-clause>
///                 </where>
///             </trait>
///         ],
///         lifetime: ,
///         bounds: [],
///         callback: [
///             name: trait_xml::trait_xml_parse_where,
///             rule: [@lccallback],
///             args: [
///                 clauses: [],
///                 callback: [
///                     name: trait_xml::trait_xml_inner,
///                     rule: [@wherecallback],
///                     args: [
///                         output: [
///                             [name Foo]
///                             [
///                                 gparams ['bar:, 'baz:,]
///                                 reqs []
///                             ]
///                         ],
///                     ],
///                 ],
///             ],
///         ],
///     }
/// }
/// ```
#[macro_export]
macro_rules! trait_xml_parse_lifetime_clause {
    (
        @parse {
            input: [],
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
                "error parsing lifetime clause: ran out of tokens. caller: `",
                stringify!($callback),
                "`",
            )
        );
    };

    (
        @parse {
            input: [</lifetime-clause>$($rest:tt)*],
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
                "error parsing lifetime clause: no lifetime provided. caller: `",
                stringify!($callback),
                "`",
            )
        );
    };

    (
        @parse {
            input: [</lifetime-clause>$($rest:tt)*],
            lifetime: $lt:lifetime,
            bounds: [$($lt1:lifetime $($ltn:lifetime)*)?],
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
                lc: [$lt: $($lt1$( + $ltn)*)?],
            }
        }
    };

    // Parse LC lifetime - broken up so that proper error handling can be provided.
    // 1: found lifetime tag, lifetime already provided
    (
        @parse {
            input: [<lifetime>$($rest:tt)*],
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
                "error parsing lifetime clause: lifetime defined as `",
                stringify!($lt),
                "` but encountered another `<lifetime>`. caller: `",
                stringify!($callback),
                "`",
            )
        );
    };
    // 2: found lifetime tag, lifetime not yet provided
    (
        @parse {
            input: [<lifetime>$($rest:tt)*],
            lifetime: ,
            bounds: $boundstoks:tt,
            callback: $callbacktoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_lifetime_clause! {
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
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(
            concat!(
                "error parsing lifetime clause: expected lifetime, found end of input. ",
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
        $crate::trait_xml_parse_lifetime_clause! {
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
            input: [$inv:tt $($rest:tt)*],
            lifetime: ,
            bounds: $boundstoks:tt,
            callback: $callbacktoks:tt,
        }
    ) => {
        compile_error!(
            concat!(
                "error parsing `for` bound lifetime: invalid lifetime `",
                stringify!($inv),
                "`. ",
                "caller: `",
                stringify!($callback),
                "`",
            )
        );
    };
    // 3.1: found end lifetime tag
    (
        @parseltend {
            input: [</lifetime>$($rest:tt)*],
            lifetime: $lt:lifetime,
            bounds: $boundstoks:tt,
            callback: $callbacktoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_lifetime_clause! {
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
                "error parsing lifetime clause: expected `</lifetime>`, found end of input. ",
                "caller: `",
                stringify!($callback),
                "`",
            )
        );
    };
    // 3.3: unexpected token instead of </lifetime>
    (
        @parseltend {
            input: [$unx:tt $($rest:tt)*],
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
                "error parsing lifetime clause: expected `</lifetime>`, found `",
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
                    name: $crate::trait_xml_parse_lifetime_clause,
                    rule: [@callback],
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
                "error parsing lifetime clause: unknown end tag `",
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
                "error parsing lifetime clause: unknown start tag `",
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
                "error parsing lifetime clause: unexpected token `",
                stringify!($unx),
                "`. caller: `",
                stringify!($callback),
                "`",
            )
        );
    };

    (
        @callback {
            input: $inputtoks:tt,
            lifetime: [$($lt:lifetime)?],
            bounds: [$($bound:tt)*],
            callback: $callbacktoks:tt,
            ltbound: $ltbound:lifetime,
        }
    ) => {
        $crate::trait_xml_parse_lifetime_clause! {
            @parse {
                input: $inputtoks,
                lifetime: $($lt)?,
                bounds: [$($bound)* $ltbound],
                callback: $callbacktoks,
            }
        }
    };
}
