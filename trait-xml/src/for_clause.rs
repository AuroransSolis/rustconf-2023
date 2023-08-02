/// Parses a `<for-clause></for-clause>` section.
///
/// Expected initial call example:
/// ```
/// trait_xml::trait_xml_parse_for_clause! {
///     @parse {
///         input: [
///                         <lifetime>'baz</lifetime>
///                         <type-clause>
///                             <type>Bar</type>
///                             <type-bound>std::ops::Fn(&'baz u8)</type-bound>
///                         </type-clause>
///                     </for-clause>
///                 </where>
///             </trait>
///         ],
///         lifetimes: [],
///         bound: [],
///         callback: [
///             name: trait_xml::trait_xml_parse_where,
///             rule: [@fccallback],
///             args: [
///                 clauses: [],
///                 callback: [
///                     name: trait_xml::trait_xml_inner,
///                     rule: [@wherecallback],
///                     args: [
///                         output: [[name Foo] [gparams [Bar:,] reqs []]],
///                     ],
///                 ],
///             ],
///         ],
///     }
/// }
/// ```
#[macro_export]
macro_rules! trait_xml_parse_for_clause {
    (
        @parse {
            input: [],
            lifetimes: $lifetimetoks:tt,
            bound: $boundtoks:tt,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(
            concat!(
                "error parsing `for` clause: ran out of tokens. caller: `",
                stringify!($callback),
                "`",
            )
        );
    };

    (
        @parse {
            input: [</for-clause>$($rest:tt)*],
            lifetimes: [],
            bound: [],
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(
            concat!(
                "error parsing `for` clause: no lifetimes or bound provided. caller: `",
                stringify!($callback),
                "`",
            )
        );
    };

    (
        @parse {
            input: [</for-clause>$($rest:tt)*],
            lifetimes: [],
            bound: [$($boundtok:tt)+],
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(
            concat!(
                "error parsing `for` clause: no lifetimes provided. caller: `",
                stringify!($callback),
                "`",
            )
        );
    };

    (
        @parse {
            input: [</for-clause>$($rest:tt)*],
            lifetimes: $lifetimetoks:tt,
            bound: [],
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(
            concat!(
                "error parsing `for` clause: no bound provided. caller: `",
                stringify!($callback),
                "`",
            )
        );
    };

    (
        @parse {
            input: [</for-clause>$($rest:tt)*],
            lifetimes: [$($forlt:lifetime)+],
            bound: [$($boundtok:tt)+],
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
                fc: [for<$($forlt,)+> $($boundtok)+],
            }
        }
    };

    // Parse FC lifetime - broken up so that proper error handling can be provided.
    // 1: found lifetime tag
    (
        @parse {
            input: [<lifetime>$($rest:tt)*],
            lifetimes: $lifetimetoks:tt,
            bound: $boundtoks:tt,
            callback: $callbacktoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_for_clause! {
            @parselt {
                input: [$($rest)*],
                lifetimes: $lifetimetoks,
                bound: $boundtoks,
                callback: $callbacktoks,
            }
        }
    };
    // 1.1: unexpected end of input
    (
        @parselt {
            input: [],
            lifetimes: $lifetimetoks:tt,
            bound: $boundtoks:tt,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(
            concat!(
                "error parsing `for` bound: expected lifetime, found end of input. ",
                "caller: `",
                stringify!($callback),
                "`",
            )
        );
    };
    // 1.2: valid lifetime
    (
        @parselt {
            input: [$lt:lifetime $($rest:tt)*],
            lifetimes: [$($forlt:lifetime)*],
            bound: $boundtoks:tt,
            callback: $callbacktoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_for_clause! {
            @parseltend {
                input: [$($rest)*],
                lifetimes: [$($forlt)* $lt],
                bound: $boundtoks,
                callback: $callbacktoks,
            }
        }
    };
    // 1.3: invalid lifetime
    (
        @parselt {
            input: [$inv:tt $($rest:tt)*],
            lifetimes: $lifetimetoks:tt,
            bound: $boundtoks:tt,
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
    // 2.1: found end lifetime tag
    (
        @parseltend {
            input: [</lifetime>$($rest:tt)*],
            lifetimes: $lifetimetoks:tt,
            bound: $boundtoks:tt,
            callback: $callbacktoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_for_clause! {
            @parse {
                input: [$($rest)*],
                lifetimes: $lifetimetoks,
                bound: $boundtoks,
                callback: $callbacktoks,
            }
        }
    };
    // 2.2: unexpected end of input
    (
        @parseltend {
            input: [],
            lifetimes: $lifetimetoks:tt,
            bound: $boundtoks:tt,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(
            concat!(
                "error parsing `for` bound lifetime: expected `</lifetime>`, found end of input. ",
                "caller: `",
                stringify!($callback),
                "`",
            )
        );
    };
    // 2.3: unexpected token instead of </lifetime>
    (
        @parseltend {
            input: [$unx:tt $($rest:tt)*],
            lifetimes: $lifetimetoks:tt,
            bound: $boundtoks:tt,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(
            concat!(
                "error parsing `for` bound lifetime: expected `</lifetime>`, found `",
                stringify!($unx),
                "`. caller: `",
                stringify!($callback),
                "`",
            )
        );
    };

    // Type clause
    (
        @parse {
            input: [<type-clause>$($rest:tt)*],
            lifetimes: $lifetimetoks:tt,
            bound: [$($boundtok:tt)+],
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(
            concat!(
                "error parsing `for` clause: type clause already defined as `",
                $(stringify!($boundtok)),+,
                "` but encountered another `<type-clause>`. caller: `",
                stringify!($callback),
                "`",
            )
        );
    };
    (
        @parse {
            input: [<type-clause>$($rest:tt)*],
            lifetimes: $lifetimetoks:tt,
            bound: [],
            callback: $callbacktoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_type_clause! {
            @parse {
                input: [$($rest)*],
                type: ,
                bounds: [],
                callback: [
                    name: $crate::trait_xml_parse_for_clause,
                    rule: [@tccallback],
                    args: [
                        lifetimes: $lifetimetoks,
                        bound: [],
                        callback: $callbacktoks,
                    ],
                ],
            }
        }
    };

    // Catch callbacks
    (
        @tccallback {
            input: $inputtoks:tt,
            lifetimes: $lifetimetoks:tt,
            bound: [],
            callback: $callbacktoks:tt,
            tc: $tct:tt,
        }
    ) => {
        $crate::trait_xml_parse_for_clause! {
            @parse {
                input: $inputtoks,
                lifetimes: $lifetimetoks,
                bound: $tct,
                callback: $callbacktoks,
            }
        }
    };
}
