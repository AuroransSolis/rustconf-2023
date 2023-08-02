/// Parses a `<const></const>` section.
///
/// Expected initial call example:
/// ```
/// trait_xml::trait_xml_parse_const_generic! {
///     @parse {
///         input: [
///                         <name>BAR</name>
///                         <type>u8</type>
///                     </const>
///                 </bounds>
///             </trait>
///         ],
///         name: ,
///         type: ,
///         callback: [
///             name: trait_xml::trait_xml_parse_bounds,
///             rule: [@constcallback],
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
#[macro_export]
macro_rules! trait_xml_parse_const_generic {
    // Empty input error
    (
        @parse {
            input: [],
            name: $($name:ident)?,
            type: $($type:path)?,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(
            concat!(
                "error parsing const generic parameter: ran out of tokens. caller: `",
                stringify!($callback),
                "`",
            )
        );
    };

    // End of input - no name or type
    (
        @parse {
            input: [</const>$($rest:tt)*],
            name: ,
            type: ,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(
            concat!(
                "error parsing const generic parameter: missing name and type. caller: `",
                stringify!($callback),
                "`",
            )
        );
    };

    // End of input - no name
    (
        @parse {
            input: [</const>$($rest:tt)*],
            name: ,
            type: $type:path,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(
            concat!(
                "error parsing const generic parameter: missing name. provided type: `",
                stringify!($type),
                "`, caller: `",
                stringify!($callback),
                "`",
            )
        );
    };

    // End of input - no type
    (
        @parse {
            input: [</const>$($rest:tt)*],
            name: $name:ident,
            type: ,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(
            concat!(
                "error parsing const generic parameter: missing type. provided name: `",
                stringify!($name),
                "`, caller: `",
                stringify!($callback),
                "`",
            )
        );
    };

    // End of input - success
    (
        @parse {
            input: [</const>$($rest:tt)*],
            name: $name:ident,
            type: $type:path,
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
                congen: [congen $name $type],
            }
        }
    };

    // found name tag, name already provided
    (
        @parse {
            input: [<name>$($rest:tt)*],
            name: $name:ident,
            type: $($type:path)?,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(
            concat!(
                "error parsing const generic parameter: multiple `<name>`s defined. ",
                "first defined as: `",
                stringify!($name),
                "`, caller: `",
                stringify!($callback),
                "`",
            )
        );
    };
    // found name tag, no name defined yet
    (
        @parse {
            input: [<name>$($rest:tt)*],
            name: ,
            type: $($type:ty)?,
            callback: $callbacktoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_name_ident! {
            @parse {
                input: [$($rest)*],
                name: ,
                callback: [
                    name: $crate::trait_xml_parse_const_generic,
                    rule: [@callback],
                    args: [
                        type: [$($type)?],
                        callback: $callbacktoks,
                    ],
                ],
            }
        }
    };

    // Parse CG type - broken up so that proper error handling can be provided.
    // 1: found type tag, type already provided
    (
        @parse {
            input: [<type>$($rest:tt)*],
            name: $($name:ident)?,
            type: $type:path,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(
            concat!(
                "error parsing const generic parameter: multiple `<type>`s defined. ",
                "first defined as: `",
                stringify!($type),
                "`, caller: `",
                stringify!($callback),
                "`",
            )
        );
    };
    // 2: found type tag, no type defined yet
    (
        @parse {
            input: [<type>$($rest:tt)*],
            name: $($name:ident)?,
            type: ,
            callback: $callbacktoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_const_generic! {
            @parsetype {
                input: [$($rest)*],
                name: $($name)?,
                type: ,
                tytoks: [],
                callback: $callbacktoks,
            }
        }
    };
    // 2.1: unexpected end of input
    (
        @parsetype {
            input: [],
            name: $($name:ident)?,
            type: ,
            tytoks: [],
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(
            concat!(
                "error parsing const generic parameter: expected type, found end of input. ",
                "caller: `",
                stringify!($callback),
                "`",
            )
        );
    };
    // 2.2: end of type - empty path
    (
        @parsetype {
            input: [</type> $($rest:tt)*],
            name: $($name:ident)?,
            type: ,
            tytoks: [],
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(
            concat!(
                "error parsing const generic parameter: empty path. caller: `",
                stringify!($callback),
                "`",
            )
        );
    };
    // 2.3: end of type - valid path
    (
        @parsetype {
            input: [</type> $($rest:tt)*],
            name: $($name:ident)?,
            type: ,
            tytoks: [$type:path],
            callback: $callbacktoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_const_generic! {
            @parse {
                input: [$($rest)*],
                name: $($name)?,
                type: $type,
                callback: $callbacktoks,
            }
        }
    };
    // 2.4: end of type - invalid path
    (
        @parsetype {
            input: [</type> $($rest:tt)*],
            name: $($name:ident)?,
            type: ,
            tytoks: [$($tytok:tt)+],
            callback: $callbacktoks:tt,
        }
    ) => {
        compile_error!(
            concat!(
                "error parsing const generic parameter: argument forms invalid type. tokens: `",
                $(stringify!($tytok)),+,
                "`, ",
                "caller: `",
                stringify!($callback),
                "`",
            )
        );
    };
    // 2.5: munch token
    (
        @parsetype {
            input: [$first:tt$($rest:tt)*],
            name: $($name:ident)?,
            type: ,
            tytoks: [$($tytok:tt)*],
            callback: $callbacktoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_const_generic! {
            @parsetype {
                input: [$($rest)*],
                name: $($name)?,
                type: ,
                tytoks: [$($tytok)* $first],
                callback: $callbacktoks,
            }
        }
    };

    (
        @callback {
            input: $inputtoks:tt,
            type: [$($type:path)?],
            callback: $callbacktoks:tt,
            name: $name:ident,
        }
    ) => {
        $crate::trait_xml_parse_const_generic! {
            @parse {
                input: $inputtoks,
                name: $name,
                type: $($type)?,
                callback: $callbacktoks,
            }
        }
    }
}
