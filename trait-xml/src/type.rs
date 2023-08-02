/// Parses a `<type></type>` section.
///
/// Expected initial call example:
/// ```
/// trait_xml::trait_xml_parse_type! {
///     @parse {
///         input: [
///                         <name>Bar</name>
///                     </type>
///                 </bounds>
///             </trait>
///         ],
///         name: ,
///         bounds: [],
///         callback: [
///             name: trait_xml::trait_xml_parse_bounds,
///             rule: [@typecallback],
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
macro_rules! trait_xml_parse_type {
    (
        @parse {
            input: [],
            name: $($name:ident)?,
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
                "error parsing generic type: ran out of tokens. caller: `",
                stringify!($callback),
                "`",
            )
        );
    };

    // Parse GT name - broken up so that proper error handling can be provided.
    // 1: found name tag, name already provided
    (
        @parse {
            input: [<name>$($rest:tt)*],
            name: $name:ident,
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
                "error parsing generic type: multiple `<name>`s defined. ",
                "first defined as: `",
                stringify!($name),
                "`, caller: `",
                stringify!($callback),
                "`",
            )
        );
    };
    // 2: found name tag, no name defined yet
    (
        @parse {
            input: [<name>$($rest:tt)*],
            name: ,
            bounds: $boundstoks:tt,
            callback: $callbacktoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_type! {
            @parsename {
                input: [$($rest)*],
                name: ,
                bounds: $boundstoks,
                callback: $callbacktoks,
            }
        }
    };
    // 2.1: unexpected end of input
    (
        @parsename {
            input: [],
            name: ,
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
                "error parsing generic type: expected identifier, found end of input. ",
                "caller: `",
                stringify!($callback),
                "`",
            )
        );
    };
    // 2.2: valid identifier
    (
        @parsename {
            input: [$name:ident $($rest:tt)*],
            name: ,
            bounds: $boundstoks:tt,
            callback: $callbacktoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_type! {
            @parsenameend {
                input: [$($rest)*],
                name: $name,
                bounds: $boundstoks,
                callback: $callbacktoks,
            }
        }
    };
    // 2.3: invalid identifier
    (
        @parsename {
            input: [$inv:tt $($rest:tt)*],
            name: ,
            bounds: $boundstoks:tt,
            callback: $callbacktoks:tt,
        }
    ) => {
        compile_error!(
            concat!(
                "error parsing generic type: invalid identifier `",
                stringify!($inv),
                "`. ",
                "caller: `",
                stringify!($callback),
                "`",
            )
        );
    };
    // 3.1: found end name tag
    (
        @parsenameend {
            input: [</name>$($rest:tt)*],
            name: $name:ident,
            bounds: $boundstoks:tt,
            callback: $callbacktoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_type! {
            @parse {
                input: [$($rest)*],
                name: $name,
                bounds: $boundstoks,
                callback: $callbacktoks,
            }
        }
    };
    // 3.2: unexpected end of input
    (
        @parsenameend {
            input: [],
            name: ,
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
                "error parsing generic type: expected `</name>`, found end of input. ",
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
            name: $name:ident,
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
                "error parsing generic type: expected `</name>`, found `",
                stringify!($unx),
                "`. caller: `",
                stringify!($callback),
                "`",
            )
        );
    };

    // Type bound
    (
        @parse {
            input: [<type-bound>$($rest:tt)*],
            name: $($name:ident)?,
            bounds: $boundstoks:tt,
            callback: $callbacktoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_type_bound! {
            @parse {
                input: [$($rest)*],
                tokens: [],
                callback: [
                    name: $crate::trait_xml_parse_type,
                    rule: [@tbcallback],
                    args: [
                        name: [$($name)?],
                        bounds: $boundstoks,
                        callback: $callbacktoks,
                    ],
                ],
            }
        }
    };
    // Lifetime bound
    (
        @parse {
            input: [<lifetime-bound>$($rest:tt)*],
            name: $($name:ident)?,
            bounds: $boundstoks:tt,
            callback: $callbacktoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_lifetime_bound! {
            @parse {
                input: [$($rest)*],
                lifetime: ,
                callback: [
                    name: $crate::trait_xml_parse_type,
                    rule: [@lbcallback],
                    args: [
                        name: [$($name)?],
                        bounds: $boundstoks,
                        callback: $callbacktoks,
                    ],
                ],
            }
        }
    };
    // `for` bound
    (
        @parse {
            input: [<for-bound>$($rest:tt)*],
            name: $($name:ident)?,
            bounds: $boundstoks:tt,
            callback: $callbacktoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_for_bound! {
            @parse {
                input: [$($rest)*],
                lifetimes: [],
                bound: ,
                callback: [
                    name: $crate::trait_xml_parse_type,
                    rule: [@fbcallback],
                    args: [
                        name: [$($name)?],
                        bounds: $boundstoks,
                        callback: $callbacktoks,
                    ],
                ],
            }
        }
    };

    (
        @parse {
            input: [</type>$($rest:tt)*],
            name: ,
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
                "error parsing generic type: no name given. caller: `",
                stringify!($callback),
                "`",
            )
        );
    };

    (
        @parse {
            input: [</type>$($rest:tt)*],
            name: $name:ident,
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
                type: [type $name $boundstoks],
            }
        }
    };

    // Catch callbacks
    (
        @tbcallback {
            input: $inputtoks:tt,
            name: [$($name:ident)?],
            bounds: [$($boundtok:tt)*],
            callback: $callbacktoks:tt,
            typebound: $boundtoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_type! {
            @parse {
                input: $inputtoks,
                name: $($name)?,
                bounds: [$($boundtok)* $boundtoks],
                callback: $callbacktoks,
            }
        }
    };
    (
        @lbcallback {
            input: $inputtoks:tt,
            name: [$($name:ident)?],
            bounds: [$($boundtok:tt)*],
            callback: $callbacktoks:tt,
            ltbound: $ltbound:lifetime,
        }
    ) => {
        $crate::trait_xml_parse_type! {
            @parse {
                input: $inputtoks,
                name: $($name)?,
                bounds: [$($boundtok)* [$ltbound]],
                callback: $callbacktoks,
            }
        }
    };
    (
        @fbcallback {
            input: $inputtoks:tt,
            name: [$($name:ident)?],
            bounds: [$($boundtok:tt)*],
            callback: $callbacktoks:tt,
            forbound: [for [$($lt:lifetime)+] $ltbound:path],
        }
    ) => {
        $crate::trait_xml_parse_type! {
            @parse {
                input: $inputtoks,
                name: $($name)?,
                bounds: [$($boundtok)* [for<$($lt),+> $ltbound]],
                callback: $callbacktoks,
            }
        }
    };
}
