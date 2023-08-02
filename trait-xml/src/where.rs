/// Parses a `<where></where>` section.
///
/// Expected initial call example:
/// ```
/// trait_xml::trait_xml_parse_where! {
///     @parse {
///         input: [
///                     <type-clause>
///                         <type><Bar as Iterator>::Item</type>
///                         <type-bound>Clone</type-bound>
///                     </type-clause>
///                 </where>
///             </trait>
///         ],
///         clauses: [],
///         callback: [
///             name: trait_xml::trait_xml_inner,
///             rule: [@wherecallback],
///             args: [
///                 output: [
///                     [name Foo]
///                     [
///                         gparams [Bar: Iterator,]
///                         reqs []
///                     ]
///                 ],
///             ],
///         ],
///     }
/// }
/// ```
#[macro_export]
macro_rules! trait_xml_parse_where {
    (
        @parse {
            input: [],
            clauses: $clausetoks:tt,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(
            concat!(
                "error parsing where clause: ran out of tokens. caller: `",
                stringify!($callback),
                "`",
            )
        );
    };

    (
        @parse {
            input: [</where>$($rest:tt)*],
            clauses: [],
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(concat!(
            "error parsing `where` clause: no clauses provided. caller: `",
            stringify!($callback),
            "`",
        ));
    };
    (
        @parse {
            input: [</where>$($rest:tt)*],
            clauses: $clausestoks:tt,
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
                where: $clausestoks,
            }
        }
    };

    (
        @parse {
            input: [<lifetime-clause>$($rest:tt)*],
            clauses: $clausetoks:tt,
            callback: $callbacktoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_lifetime_clause! {
            @parse {
                input: [$($rest)*],
                lifetime: ,
                bounds: [],
                callback: [
                    name: $crate::trait_xml_parse_where,
                    rule: [@lccallback],
                    args: [
                        clauses: $clausetoks,
                        callback: $callbacktoks,
                    ],
                ],
            }
        }
    };
    (
        @parse {
            input: [<type-clause>$($rest:tt)*],
            clauses: $clausetoks:tt,
            callback: $callbacktoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_type_clause! {
            @parse {
                input: [$($rest)*],
                type: ,
                bounds: [],
                callback: [
                    name: $crate::trait_xml_parse_where,
                    rule: [@tccallback],
                    args: [
                        clauses: $clausetoks,
                        callback: $callbacktoks,
                    ],
                ],
            }
        }
    };
    (
        @parse {
            input: [<for-clause>$($rest:tt)*],
            clauses: $clausetoks:tt,
            callback: $callbacktoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_for_clause! {
            @parse {
                input: [$($rest)*],
                lifetimes: [],
                bound: [],
                callback: [
                    name: $crate::trait_xml_parse_where,
                    rule: [@fccallback],
                    args: [
                        clauses: $clausetoks,
                        callback: $callbacktoks,
                    ],
                ],
            }
        }
    };

    (
        @parse {
            input: [</$unk:tt$($rest:tt)*],
            clauses: $clausestoks:tt,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(
            concat!(
                "error parsing where clause: unknown end tag `",
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
            clauses: $clausestoks:tt,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(
            concat!(
                "error parsing where clause: unknown start tag `",
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
            clauses: $clausestoks:tt,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(
            concat!(
                "error parsing where clause: unexpected token `",
                stringify!($unx),
                "`. caller: `",
                stringify!($callback),
                "`",
            )
        );
    };

    (
        @lccallback {
            input: $inputtoks:tt,
            clauses: [$($clause:tt)*],
            callback: $callbacktoks:tt,
            lc: $lct:tt,
        }
    ) => {
        $crate::trait_xml_parse_where! {
            @parse {
                input: $inputtoks,
                clauses: [$($clause)* $lct],
                callback: $callbacktoks,
            }
        }
    };
    (
        @tccallback {
            input: $inputtoks:tt,
            clauses: [$($clause:tt)*],
            callback: $callbacktoks:tt,
            tc: $tct:tt,
        }
    ) => {
        $crate::trait_xml_parse_where! {
            @parse {
                input: $inputtoks,
                clauses: [$($clause)* $tct],
                callback: $callbacktoks,
            }
        }
    };
    (
        @fccallback {
            input: $inputtoks:tt,
            clauses: [$($clause:tt)*],
            callback: $callbacktoks:tt,
            fc: $fct:tt,
        }
    ) => {
        $crate::trait_xml_parse_where! {
            @parse {
                input: $inputtoks,
                clauses: [$($clause)* $fct],
                callback: $callbacktoks,
            }
        }
    };
}
