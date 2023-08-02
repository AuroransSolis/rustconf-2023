/// Parses a `<for-bound></for-bound>` section.
///
/// Expected initial call example:
/// ```
/// trait_xml::trait_xml_parse_for_bound! {
///     @parse {
///         input: [
///                             <lifetime>'baz</lifetime>
///                             <type-bound>std::ops::Fn(&'baz u8)</type-bound>
///                         </for-bound>
///                     </type>
///                 </bounds>
///             </trait>
///         ],
///         lifetimes: [],
///         bound: ,
///         callback: [
///             name: trait_xml::trait_xml_parse_type,
///             rule: [@fbcallback],
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
macro_rules! trait_xml_parse_for_bound {
    (
        @parse {
            input: [],
            lifetimes: $lifetimetoks:tt,
            bound: $($bound:path)?,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(
            concat!(
                "error parsing `for` bound: ran out of tokens. caller: `",
                stringify!($callback),
                "`",
            )
        );
    };

    (
        @parse {
            input: [</for-bound>$($rest:tt)*],
            lifetimes: [$($forlt:lifetime)+],
            bound: $bound:path,
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
                forbound: [for [$($forlt)+] $bound],
            }
        }
    };

    (
        @parse {
            input: [</for-bound>$($rest:tt)*],
            lifetimes: [],
            bound: ,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(
            concat!(
                "error parsing `for` bound: no lifetimes or bound provided. caller: `",
                stringify!($callback),
                "`",
            )
        );
    };

    (
        @parse {
            input: [</for-bound>$($rest:tt)*],
            lifetimes: [$($forlt:lifetime)+],
            bound: ,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(
            concat!(
                "error parsing `for` bound: no bound provided. caller: `",
                stringify!($callback),
                "`",
            )
        );
    };

    (
        @parse {
            input: [</for-bound>$($rest:tt)*],
            lifetimes: [],
            bound: $bound:path,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(
            concat!(
                "error parsing `for` bound: no lifetimes provided. caller: `",
                stringify!($callback),
                "`",
            )
        );
    };

    // Parse FB lifetime - broken up so that proper error handling can be provided.
    // 1: found lifetime tag
    (
        @parse {
            input: [<lifetime>$($rest:tt)*],
            lifetimes: $lifetimetoks:tt,
            bound: $($bound:path)?,
            callback: $callbacktoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_for_bound! {
            @parselt {
                input: [$($rest)*],
                lifetimes: $lifetimetoks,
                bound: $($bound)?,
                callback: $callbacktoks,
            }
        }
    };
    // 1.1: unexpected end of input
    (
        @parselt {
            input: [],
            lifetimes: $lifetimetoks:tt,
            bound: $($bound:path)?,
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
            bound: $($bound:path)?,
            callback: $callbacktoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_for_bound! {
            @parseltend {
                input: [$($rest)*],
                lifetimes: [$($forlt)* $lt],
                bound: $($bound)?,
                callback: $callbacktoks,
            }
        }
    };
    // 1.3: invalid lifetime
    (
        @parselt {
            input: [$inv:tt $($rest:tt)*],
            lifetimes: $lifetimetoks:tt,
            bound: $($bound:path)?,
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
            bound: $($bound:path)?,
            callback: $callbacktoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_for_bound! {
            @parse {
                input: [$($rest)*],
                lifetimes: $lifetimetoks,
                bound: $($bound)?,
                callback: $callbacktoks,
            }
        }
    };
    // 2.2: unexpected end of input
    (
        @parseltend {
            input: [],
            lifetimes: $lifetimetoks:tt,
            bound: $($bound:path)?,
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
            bound: $($bound:path)?,
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

    (
        @parse {
            input: [<type-bound>$($rest:tt)*],
            lifetimes: $lifetimetoks:tt,
            bound: $bound:path,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(
            concat!(
                "error parsing `for` bound: multiple `<bound>`s defined. caller: `",
                stringify!($callback),
                "`",
            )
        );
    };
    (
        @parse {
            input: [<type-bound>$($rest:tt)*],
            lifetimes: $lifetimetoks:tt,
            bound: ,
            callback: $callbacktoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_type_bound! {
            @parse {
                input: [$($rest)*],
                tokens: [],
                callback: [
                    name: $crate::trait_xml_parse_for_bound,
                    rule: [@tbcallback],
                    args: [
                        lifetimes: $lifetimetoks,
                        callback: $callbacktoks,
                    ],
                ],
            }
        }
    };

    (
        @tbcallback {
            input: $inputtoks:tt,
            lifetimes: $lifetimetoks:tt,
            callback: $callbacktoks:tt,
            typebound: [$bound:path],
        }
    ) => {
        $crate::trait_xml_parse_for_bound! {
            @parse {
                input: $inputtoks,
                lifetimes: $lifetimetoks,
                bound: $bound,
                callback: $callbacktoks,
            }
        }
    };
}
