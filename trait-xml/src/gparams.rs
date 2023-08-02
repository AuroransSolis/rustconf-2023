#[macro_export]
/// Parses a `<gparams></gparams>` section.
///
/// Expected initial call example:
///
macro_rules! trait_xml_parse_gparams {
    // Empty input error
    (
        @parse {
            input: [],
            consts: $cgt:tt,
            lifetimes: $ltt:tt,
            types: $tyt:tt,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(concat!(
            "error parsing bounds: ran out of tokens. caller: `",
            stringify!($callback),
            "`",
        ));
    };

    // End bounds
    (
        @parse {
            input: [</gparams>$($rest:tt)*],
            consts: [],
            lifetimes: [],
            types: [],
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(concat!(
            "error parsing generic parameters: no generic parameters provided. caller: `",
            stringify!($callback),
            "`",
        ));
    };
    (
        @parse {
            input: [</gparams>$($rest:tt)*],
            consts: [$([congen $cgname:ident $cgtype:path])*],
            lifetimes: [$([lt $ltname:lifetime [$($ltbound:lifetime)*]])*],
            types: [$($tytoks:tt)*],
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
                gparams: [
                    $(
                        $ltname: $($ltbound +)*,
                    )*
                    $($tytoks)*
                    $(
                        const $cgname: $cgtype,
                    )*
                ],
            }
        }
    };

    // Parse const generic
    (
        @parse {
            input: [<const>$($rest:tt)*],
            consts: $cgt:tt,
            lifetimes: $ltt:tt,
            types: $tyt:tt,
            callback: $callbacktoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_const_generic! {
            @parse {
                input: [$($rest)*],
                name: ,
                type: ,
                callback: [
                    name: $crate::trait_xml_parse_gparams,
                    rule: [@constcallback],
                    args: [
                        consts: $cgt,
                        lifetimes: $ltt,
                        types: $tyt,
                        callback: $callbacktoks,
                    ],
                ],
            }
        }
    };
    // Parse lifetime
    (
        @parse {
            input: [<lifetime>$($rest:tt)*],
            consts: $cgt:tt,
            lifetimes: $ltt:tt,
            types: $tyt:tt,
            callback: $callbacktoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_lifetime! {
            @parse {
                input: [$($rest)*],
                lifetime:,
                bounds: [],
                callback: [
                    name: $crate::trait_xml_parse_gparams,
                    rule: [@lifetimecallback],
                    args: [
                        consts: $cgt,
                        lifetimes: $ltt,
                        types: $tyt,
                        callback: $callbacktoks,
                    ],
                ],
            }
        }
    };
    (
        @parse {
            input: [<type>$($rest:tt)*],
            consts: $cgt:tt,
            lifetimes: $ltt:tt,
            types: $tyt:tt,
            callback: $callbacktoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_type! {
            @parse {
                input: [$($rest)*],
                name: ,
                bounds: [],
                callback: [
                    name: $crate::trait_xml_parse_gparams,
                    rule: [@typecallback],
                    args: [
                        consts: $cgt,
                        lifetimes: $ltt,
                        types: $tyt,
                        callback: $callbacktoks,
                    ],
                ],
            }
        }
    };

    // Unknown end tag
    (
        @parse {
            input: [</$unk:tt$($rest:tt)*],
            consts: $cgt:tt,
            lifetimes: $ltt:tt,
            types: $tyt:tt,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(
            concat!(
                "error parsing bounds: unknown beginning of end tag `",
                stringify!($unk),
                "`. caller: `",
                stringify!($callback),
                "`",
            )
        );
    };
    // Unknown start tag
    (
        @parse {
            input: [<$unk:tt$($rest:tt)*],
            consts: $cgt:tt,
            lifetimes: $ltt:tt,
            types: $tyt:tt,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(
            concat!(
                "error parsing bounds: unknown beginning of start tag `",
                stringify!($unk),
                "`. caller: `",
                stringify!($callback),
                "`",
            )
        );
    };
    // Unexpected token
    (
        @parse {
            input: [$unx:tt$($rest:tt)*],
            consts: $cgt:tt,
            lifetimes: $ltt:tt,
            types: $tyt:tt,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(
            concat!(
                "error parsing bounds: unexpected token `",
                stringify!($unx),
                "`. caller: `",
                stringify!($callback),
                "`",
            )
        );
    };

    // Catch callbacks
    (
        @constcallback {
            input: $inputtoks:tt,
            consts: [$($ct:tt)*],
            lifetimes: $lft:tt,
            types: $tt:tt,
            callback: $callbacktoks:tt,
            congen: [congen $cgname:ident $type:path],
        }
    ) => {
        $crate::trait_xml_parse_gparams! {
            @parse {
                input: $inputtoks,
                consts: [$($ct)* [congen $cgname $type]],
                lifetimes: $lft,
                types: $tt,
                callback: $callbacktoks,
            }
        }
    };
    (
        @lifetimecallback {
            input: $inputtoks:tt,
            consts: $ct:tt,
            lifetimes: [$($ltt:tt)*],
            types: $tt:tt,
            callback: $callbacktoks:tt,
            lifetime: [lt $lt:lifetime [$($ltbound:lifetime)*]],
        }
    ) => {
        $crate::trait_xml_parse_gparams! {
            @parse {
                input: $inputtoks,
                consts: $ct,
                lifetimes: [$($ltt)* [lt $lt [$($ltbound)*]]],
                types: $tt,
                callback: $callbacktoks,
            }
        }
    };
    (
        @typecallback {
            input: $inputtoks:tt,
            consts: $ct:tt,
            lifetimes: $lft:tt,
            types: [$($tt:tt)*],
            callback: $callbacktoks:tt,
            type: [
                type $typename:ident [$(
                    [$($tybound1:tt)+]
                    $([$($tyboundn:tt)+])*
                )?]
            ],
        }
    ) => {
        $crate::trait_xml_parse_gparams! {
            @parse {
                input: $inputtoks,
                consts: $ct,
                lifetimes: $lft,
                types: [$($tt)* $typename: $($($tybound1)+$( + $($tyboundn)+)*)?,],
                callback: $callbacktoks,
            }
        }
    };
}
