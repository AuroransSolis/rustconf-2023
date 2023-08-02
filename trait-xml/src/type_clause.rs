/// Parses a `<type-clause></type-clause>` section.
///
/// Expected initial call example:
/// ```
/// trait_xml::trait_xml_parse_type_clause! {
///     @parse {
///         input: [
///                         <type><Bar as Iterator>::Item</type>
///                         <type-bound>Clone</type-bound>
///                     </type-clause>
///                 </where>
///             </trait>
///         ],
///         type: ,
///         bounds: [],
///         callback: [
///             name: trait_xml::trait_xml_parse_where,
///             rule: [@tccallback],
///             args: [
///                 clauses: [],
///                 callback: [
///                     name: trait_xml::trait_xml_inner,
///                     rule: [@wherecallback],
///                     args: [
///                         output: [
///                             [name Foo]
///                             [
///                                 gparams [Bar: Iterator,]
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
macro_rules! trait_xml_parse_type_clause {
    (
        @parse {
            input: [],
            type: $($type:ty)?,
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
                "error parsing type clause: ran out of tokens. caller: `",
                stringify!($callback),
                "`",
            )
        );
    };

    (
        @parse {
            input: [</type-clause>$($rest:tt)*],
            type: $type:ty,
            bounds: [$([$($bound1:tt)+]$([$($boundn:tt)+])*)?],
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
                tc: [$type: $($($bound1)+$( + $($boundn)+)*)?],
            }
        }
    };

    (
        @parse {
            input: [</type-clause>$($rest:tt)*],
            type: ,
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
                "error parsing type clause: no type provided. caller: `",
                stringify!($callback),
                "`",
            )
        );
    };

    (
        @parse {
            input: [<type>$($rest:tt)*],
            type: $type:ty,
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
                "error parsing type clause: type already defined as `",
                stringify!($type),
                "` but encountered another `<type>`. caller: `",
                stringify!($callback),
                "`",
            )
        );
    };
    (
        @parse {
            input: [<type>$($rest:tt)*],
            type: ,
            bounds: $boundstoks:tt,
            callback: $callbacktoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_type_ty! {
            @parse {
                input: [$($rest)*],
                type: ,
                tokens: [],
                callback: [
                    name: $crate::trait_xml_parse_type_clause,
                    rule: [@typecallback],
                    args: [
                        bounds: $boundstoks,
                        callback: $callbacktoks,
                    ],
                ],
            }
        }
    };

    // Type bound
    (
        @parse {
            input: [<type-bound>$($rest:tt)*],
            type: $($type:ty)?,
            bounds: $boundstoks:tt,
            callback: $callbacktoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_type_bound! {
            @parse {
                input: [$($rest)*],
                tokens: [],
                callback: [
                    name: $crate::trait_xml_parse_type_clause,
                    rule: [@tbcallback],
                    args: [
                        type: [$($type)?],
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
            type: $($type:ty)?,
            bounds: $boundstoks:tt,
            callback: $callbacktoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_lifetime_bound! {
            @parse {
                input: [$($rest)*],
                lifetime: ,
                callback: [
                    name: $crate::trait_xml_parse_type_clause,
                    rule: [@lbcallback],
                    args: [
                        type: [$($type)?],
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
            type: $($type:ty)?,
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
                    name: $crate::trait_xml_parse_type_clause,
                    rule: [@fbcallback],
                    args: [
                        type: [$($type)?],
                        bounds: $boundstoks,
                        callback: $callbacktoks,
                    ],
                ],
            }
        }
    };

    // Catch callbacks
    (
        @typecallback {
            input: $inputtoks:tt,
            bounds: $boundstoks:tt,
            callback: $callbacktoks:tt,
            type: $type:ty,
        }
    ) => {
        $crate::trait_xml_parse_type_clause! {
            @parse {
                input: $inputtoks,
                type: $type,
                bounds: $boundstoks,
                callback: $callbacktoks,
            }
        }
    };
    (
        @tbcallback {
            input: $inputtoks:tt,
            type: [$($type:ty)?],
            bounds: [$($boundtok:tt)*],
            callback: $callbacktoks:tt,
            typebound: $tyboundtoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_type_clause! {
            @parse {
                input: $inputtoks,
                type: $($type)?,
                bounds: [$($boundtok)* $tyboundtoks],
                callback: $callbacktoks,
            }
        }
    };
    (
        @lbcallback {
            input: $inputtoks:tt,
            type: [$($type:ty)?],
            bounds: [$($boundtok:tt)*],
            callback: $callbacktoks:tt,
            ltbound: $ltbound:lifetime,
        }
    ) => {
        $crate::trait_xml_parse_type_clause! {
            @parse {
                input: $inputtoks,
                type: $($type)?,
                bounds: [$($boundtok)* [$ltbound]],
                callback: $callbacktoks,
            }
        }
    };
    (
        @fbcallback {
            input: $inputtoks:tt,
            type: [$($type:ty)?],
            bounds: [$($boundtok:tt)*],
            callback: $callbacktoks:tt,
            forbound: [for [$($lt:lifetime)+] $ltbound:path],
        }
    ) => {
        $crate::trait_xml_parse_type_clause! {
            @parse {
                input: $inputtoks,
                type: $($type)?,
                bounds: [$($boundtok)* [for<$($lt),+> $ltbound]],
                callback: $callbacktoks,
            }
        }
    };
}
