/// Parses a `<assocconst></assocconst>` section.
///
/// Expected initial call example:
/// ```
/// trait_xml::trait_xml_parse_assoc_const! {
///     @parse {
///         input: [
///                     <name>BAR</name>
///                     <type>&'static str</type>
///                     <default-value>"baz"</default-value>
///                 </assocconst>
///             </trait>
///         ],
///         name: ,
///         type: ,
///         default: ,
///         callback: [
///             name: trait_xml::trait_xml_inner,
///             rule: [@accallback],
///             args: [
///                 output: [[name Foo]],
///             ],
///         ],
///     }
/// }
/// ```
#[macro_export]
macro_rules! trait_xml_parse_assoc_const {
    // Empty input error
    (
        @parse {
            input: [],
            name: $($name:ident)?,
            type: $($type:ty)?,
            default: $($default:expr)?,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(concat!(
            "error parsing associated constant: ran out of tokens. caller: `",
            stringify!($callback),
            "`",
        ));
    };

    // Successfully parsed associated const
    (
        @parse {
            input: [</assocconst>$($rest:tt)*],
            name: $name:ident,
            type: $type:ty,
            default: $($default:expr)?,
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
                assoc const: [ac [$name] [$type] [$($default)?]],
            }
        }
    };

    // Missing field(s) error
    (
        @parse {
            input: [</assocconst>$($rest:tt)*],
            name: $($name:ident)?,
            type: $($type:ty)?,
            default: $($default:expr)?,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(concat!(
            "error parsing associated constant: missing one or more fields. name: `",
            $(stringify!($name),)?
            "`, type: `",
            $(stringify!($type),)?
            "`. caller: `",
            stringify!($callback),
            "`",
        ));
    };

    (
        @parse {
            input: [<name>$($rest:tt)*],
            name: $name:ident,
            type: $type:ty,
            default: $($default:expr)?,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(concat!(
            "error parsing associated constant: name already defined as `",
            stringify!($name),
            "`, but encountered a second `<name>` tag. caller: `",
            stringify!($callback),
        ));
    };
    (
        @parse {
            input: [<name>$($rest:tt)*],
            name: ,
            type: $($type:ty)?,
            default: $($default:expr)?,
            callback: $callbacktoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_name_ident! {
            @parse {
                input: [$($rest)*],
                name: ,
                callback: [
                    name: $crate::trait_xml_parse_assoc_const,
                    rule: [@namecallback],
                    args: [
                        type: [$($type)?],
                        default: [$($default)?],
                        callback: $callbacktoks,
                    ],
                ],
            }
        }
    };

    (
        @parse {
            input: [<type>$($rest:tt)*],
            name: $($name:ident)?,
            type: $type:ty,
            default: $($default:expr)?,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(concat!(
            "error parsing associated constant: type already defined as `",
            stringify!($type),
            "`, but encountered a second `<type>` tag. caller: `",
            stringify!($callback),
        ));
    };
    (
        @parse {
            input: [<type>$($rest:tt)*],
            name: $($name:ident)?,
            type: ,
            default: $($default:expr)?,
            callback: $callbacktoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_type_ty! {
            @parse {
                input: [$($rest)*],
                type: ,
                tokens: [],
                callback: [
                    name: $crate::trait_xml_parse_assoc_const,
                    rule: [@typecallback],
                    args: [
                        name: [$($name)?],
                        default: [$($default)?],
                        callback: $callbacktoks,
                    ],
                ],
            }
        }
    };

    (
        @parse {
            input: [<default-value>$($rest:tt)*],
            name: $($name:ident)?,
            type: $($type:ty)?,
            default: $default:expr,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(concat!(
            "error parsing associated constant: default value already defined as `",
            stringify!($default),
            "`, but encountered a second `<default-value>` tag. caller: `",
            stringify!($callback),
            "`",
        ));
    };
    (
        @parse {
            input: [<default-value>$($rest:tt)*],
            name: $($name:ident)?,
            type: $($type:ty)?,
            default: ,
            callback: $callbacktoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_assoc_const! {
            @parsedefault {
                input: [$($rest)*],
                name: $($name)?,
                type: $($type)?,
                default: ,
                tokens: [],
                callback: $callbacktoks,
            }
        }
    };
    (
        @parsedefault {
            input: [</default-value>$($rest:tt)*],
            name: $($name:ident)?,
            type: $($type:ty)?,
            default: ,
            tokens: [],
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(concat!(
            "error parsing associated constant default value: empty expression between tags. ",
            "caller: `",
            stringify!($callback),
            "`",
        ));
    };
    (
        @parsedefault {
            input: [</default-value>$($rest:tt)*],
            name: $($name:ident)?,
            type: $($type:ty)?,
            default: ,
            tokens: [$default:expr],
            callback: $callbacktoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_assoc_const! {
            @parse {
                input: [$($rest)*],
                name: $($name)?,
                type: $($type)?,
                default: $default,
                callback: $callbacktoks,
            }
        }
    };
    (
        @parsedefault {
            input: [</default-value>$($rest:tt)*],
            name: $($name:ident)?,
            type: $($type:ty)?,
            default: ,
            tokens: [$($inv:tt)+],
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(concat!(
            "error parsing associated constant default value: invalid expression between tags `",
            $(stringify!($inv)),+,
            "`. caller: `",
            stringify!($callback),
            "`",
        ));
    };
    (
        @parsedefault {
            input: [$first:tt$($rest:tt)*],
            name: $($name:ident)?,
            type: $($type:ty)?,
            default: ,
            tokens: [$($defaulttok:tt)*],
            callback: $callbacktoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_assoc_const! {
            @parsedefault {
                input: [$($rest)*],
                name: $($name)?,
                type: $($type)?,
                default: ,
                tokens: [$($defaulttok)* $first],
                callback: $callbacktoks,
            }
        }
    };

    // Catch callbacks
    (
        @namecallback {
            input: $inputtoks:tt,
            type: [$($type:ty)?],
            default: [$($default:expr)?],
            callback: $callbacktoks:tt,
            name: $name:ident,
        }
    ) => {
        $crate::trait_xml_parse_assoc_const! {
            @parse {
                input: $inputtoks,
                name: $name,
                type: $($type)?,
                default: $($default)?,
                callback: $callbacktoks,
            }
        }
    };
    (
        @typecallback {
            input: $inputtoks:tt,
            name: [$($name:ident)?],
            default: [$($default:expr)?],
            callback: $callbacktoks:tt,
            type: $type:ty,
        }
    ) => {
        $crate::trait_xml_parse_assoc_const! {
            @parse {
                input: $inputtoks,
                name: $($name)?,
                type: $type,
                default: $($default)?,
                callback: $callbacktoks,
            }
        }
    };
}
