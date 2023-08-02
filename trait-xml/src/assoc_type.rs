/// Parses a `<assoctype></assoctype>` section.
///
/// Expected initial call example:
/// ```
/// # trait Foo {}
/// trait_xml::trait_xml_parse_assoc_type! {
///     @parse {
///         input: [
///                     <name>Baz</name>
///                     <bounds>
///                         <req>Foo</req>
///                         <type>
///                             <name>Baq</name>
///                         </type>
///                     </bounds>
///                     <where>
///                         <type-clause>
///                             <type>Self::Baz<Baq></type>
///                             <type-bound>Clone</type-bound>
///                         </type-clause>
///                     </where>
///                 </assoctype>
///             </trait>
///         ],
///         name: ,
///         gparams: [],
///         bounds: [],
///         hasbounds: [],
///         boundswhere: [],
///         hasboundswhere: [],
///         callback: [
///             name: trait_xml::trait_xml_inner,
///             rule: [@atcallback],
///             args: [
///                 output: [[name Bar]],
///             ],
///         ],
///     }
/// }
/// ```
#[macro_export]
macro_rules! trait_xml_parse_assoc_type {
    // Empty input error
    (
        @parse {
            input: [],
            name: $($name:ident)?,
            gparams: $gpt:tt,
            bounds: $boundstoks:tt,
            hasbounds: $hbt:tt,
            boundswhere: $bwtoks:tt,
            hasboundswhere: $hbwt:tt,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(concat!(
            "error parsing associated type: ran out of tokens. caller: `",
            stringify!($callback),
            "`",
        ));
    };

    // Successfully parsed associated type
    (
        @parse {
            input: [</assoctype>$($rest:tt)*],
            name: $name:ident,
            gparams: $gpt:tt,
            bounds: $boundstoks:tt,
            hasbounds: $hbt:tt,
            boundswhere: $bwtoks:tt,
            hasboundswhere: $hbwt:tt,
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
                assoc type: [
                    at
                    [$name]
                    $gpt
                    $boundstoks
                    $bwtoks
                ],
            }
        }
    };
    // Missing name error
    (
        @parse {
            input: [</assoctype>$($rest:tt)*],
            name: ,
            gparams: $gpt:tt,
            bounds: $boundstoks:tt,
            hasbounds: $hbt:tt,
            boundswhere: $bwtoks:tt,
            hasboundswhere: $hbwt:tt,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(concat!(
            "error parsing associated type: missing name. caller: `",
            stringify!($callback),
            "`",
        ));
    };

    // Multiple names defined error
    (
        @parse {
            input: [<name>$($rest:tt)*],
            name: $name:ident,
            gparams: $gpt:tt,
            bounds: $boundstoks:tt,
            hasbounds: $hbt:tt,
            boundswhere: $bwtoks:tt,
            hasboundswhere: $hbwt:tt,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(concat!(
            "error parsing associated type: name already defined as `",
            stringify!($name),
            "`, but encountered a second `<name>` tag. caller: `",
            stringify!($callback),
            "`",
        ));
    };
    // Parse associated type name
    (
        @parse {
            input: [<name>$($rest:tt)*],
            name: ,
            gparams: $gpt:tt,
            bounds: $boundstoks:tt,
            hasbounds: $hbt:tt,
            boundswhere: $bwtoks:tt,
            hasboundswhere: $hbwt:tt,
            callback: $callbacktoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_name_ident! {
            @parse {
                input: [$($rest)*],
                name: ,
                callback: [
                    name: $crate::trait_xml_parse_assoc_type,
                    rule: [@namecallback],
                    args: [
                        gparams: $gpt,
                        bounds: $boundstoks,
                        hasbounds: $hbt,
                        boundswhere: $bwtoks,
                        hasboundswhere: $hbwt,
                        callback: $callbacktoks,
                    ],
                ],
            }
        }
    };
    // Multiple bounds sections error
    (
        @parse {
            input: [<bounds>$($rest:tt)*],
            name: $($name:ident)?,
            gparams: $gpt:tt,
            bounds: $boundstoks:tt,
            hasbounds: [[]],
            boundswhere: $bwtoks:tt,
            hasboundswhere: $hbwt:tt,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(concat!(
            "error parsing associated type: bounds already defined, but encountered a second ",
            "`<bounds>` tag. caller: `",
            stringify!($callback),
            "`",
        ));
    };
    // Parse associated type bounds
    (
        @parse {
            input: [<bounds>$($rest:tt)*],
            name: $($name:ident)?,
            gparams: $gpt:tt,
            bounds: $boundstoks:tt,
            hasbounds: [],
            boundswhere: $bwtoks:tt,
            hasboundswhere: $hbwt:tt,
            callback: $callbacktoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_bounds! {
            @parse {
                input: [$($rest)*],
                consts: [],
                lifetimes: [],
                types: [],
                reqs: [],
                callback: [
                    name: $crate::trait_xml_parse_assoc_type,
                    rule: [@boundscallback],
                    args: [
                        name: [$($name)?],
                        hasbounds: [[]],
                        boundswhere: $bwtoks,
                        hasboundswhere: $hbwt,
                        callback: $callbacktoks,
                    ],
                ],
            }
        }
    };
    // Multiple `where` sections error
    (
        @parse {
            input: [<where>$($rest:tt)*],
            name: $($name:ident)?,
            gparams: $gpt:tt,
            bounds: $boundstoks:tt,
            hasbounds: $hbt:tt,
            boundswhere: $bwtoks:tt,
            hasboundswhere: [[]],
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(concat!(
            "error parsing associated type: multiple `<where></where>` sections. caller: `",
            stringify!($callback),
            "`",
        ));
    };
    // Parse associated type `where` clause
    (
        @parse {
            input: [<where>$($rest:tt)*],
            name: $($name:ident)?,
            gparams: $gpt:tt,
            bounds: $boundstoks:tt,
            hasbounds: $hbt:tt,
            boundswhere: $bwtoks:tt,
            hasboundswhere: [],
            callback: $callbacktoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_where! {
            @parse {
                input: [$($rest)*],
                clauses: [],
                callback: [
                    name: $crate::trait_xml_parse_assoc_type,
                    rule: [@wherecallback],
                    args: [
                        name: [$($name)?],
                        gparams: $gpt,
                        bounds: $boundstoks,
                        hasbounds: $hbt,
                        hasboundswhere: [[]],
                        callback: $callbacktoks,
                    ],
                ],
            }
        }
    };
    (
        @parse {
            input: [</$unx:tt$($rest:tt)*],
            name: $($name:ident)?,
            gparams: $gpt:tt,
            bounds: $boundstoks:tt,
            hasbounds: $hbt:tt,
            boundswhere: $bwtoks:tt,
            hasboundswhere: $hbwt:tt,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(concat!(
            "error parsing associated type: encountered unexpected end tag `",
            stringify!($unx),
            "`. caller: `",
            stringify!($callback),
            "`",
        ));
    };
    (
        @parse {
            input: [<$unk:tt$($rest:tt)*],
            name: $($name:ident)?,
            gparams: $gpt:tt,
            bounds: $boundstoks:tt,
            hasbounds: $hbt:tt,
            boundswhere: $bwtoks:tt,
            hasboundswhere: $hbwt:tt,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(concat!(
            "error parsing associated type: encountered unknown start tag `",
            stringify!($unk),
            "`. caller: `",
            stringify!($callback),
            "`",
        ));
    };
    (
        @parse {
            input: [$unx:tt$($rest:tt)*],
            name: $($name:ident)?,
            gparams: $gpt:tt,
            bounds: $boundstoks:tt,
            hasbounds: $hbt:tt,
            boundswhere: $bwtoks:tt,
            hasboundswhere: $hbwt:tt,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(concat!(
            "error parsing associated type: encountered unexpected token `",
            stringify!($unx),
            "`. caller: `",
            stringify!($callback),
            "`",
        ));
    };

    // Catch callbacks
    (
        @namecallback {
            input: $inputtoks:tt,
            gparams: $gpt:tt,
            bounds: $boundstoks:tt,
            hasbounds: $hbt:tt,
            boundswhere: $bwtoks:tt,
            hasboundswhere: $hbwt:tt,
            callback: $callbacktoks:tt,
            name: $name:ident,
        }
    ) => {
        $crate::trait_xml_parse_assoc_type! {
            @parse {
                input: $inputtoks,
                name: $name,
                gparams: $gpt,
                bounds: $boundstoks,
                hasbounds: $hbt,
                boundswhere: $bwtoks,
                hasboundswhere: $hbwt,
                callback: $callbacktoks,
            }
        }
    };
    (
        @boundscallback {
            input: $inputtoks:tt,
            name: [$($name:ident)?],
            hasbounds: $hbt:tt,
            boundswhere: $bwtoks:tt,
            hasboundswhere: $hbwt:tt,
            callback: $callbacktoks:tt,
            gparams: $gpt:tt,
            reqs: $rt:tt,
        }
    ) => {
        $crate::trait_xml_parse_assoc_type! {
            @parse {
                input: $inputtoks,
                name: $($name)?,
                gparams: $gpt,
                bounds: $rt,
                hasbounds: $hbt,
                boundswhere: $bwtoks,
                hasboundswhere: $hbwt,
                callback: $callbacktoks,
            }
        }
    };
    (
        @wherecallback {
            input: $inputtoks:tt,
            name: [$($name:ident)?],
            gparams: $gpt:tt,
            bounds: $boundstoks:tt,
            hasbounds: $hbt:tt,
            hasboundswhere: $hbwt:tt,
            callback: $callbacktoks:tt,
            where: $wheretoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_assoc_type! {
            @parse {
                input: $inputtoks,
                name: $($name)?,
                gparams: $gpt,
                bounds: $boundstoks,
                hasbounds: $hbt,
                boundswhere: $wheretoks,
                hasboundswhere: $hbwt,
                callback: $callbacktoks,
            }
        }
    };
}
