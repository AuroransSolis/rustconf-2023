/// Parses a `<name></name>` section where the tokens between the tags should form an identifier.
///
/// Expected initial call example:
/// ```
/// trait_xml::trait_xml_parse_name_ident! {
///     @parse {
///         input: [
///                 Foo</name>
///             </trait>
///         ],
///         name: ,
///         callback: [
///             name: trait_xml::trait_xml_inner,
///             rule: [@namecallback],
///             args: [
///                 output: [],
///             ],
///         ],
///     }
/// }
/// ```
#[macro_export]
macro_rules! trait_xml_parse_name_ident {
    (
        @parse {
            input: [],
            name: $($name:ident)?,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(
            concat!(
                "error parsing name: expected identifier, found end of input. caller: `",
                stringify!($callback),
                "`",
            )
        );
    };

    (
        @parse {
            input: [$name:ident$($rest:tt)*],
            name: ,
            callback: $callbacktoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_name_ident! {
            @parseend {
                input: [$($rest)*],
                name: $name,
                callback: $callbacktoks,
            }
        }
    };
    (
        @parse {
            input: [$unk:tt$($rest:tt)*],
            name: ,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(
            concat!(
                "error parsing name: expected identifier, found `",
                stringify!($unk),
                "`. caller: `",
                stringify!($callback),
                "`",
            )
        );
    };

    (
        @parseend {
            input: [</name>$($rest:tt)*],
            name: $name:ident,
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
                name: $name,
            }
        }
    };
    (
        @parseend {
            input: [</$unk:tt$($rest:tt)*],
            name: $name:ident,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(
            concat!(
                "error parsing name: expected `</name>`, found start of unkown end tag `",
                stringify!($unk),
                "`. caller: `",
                stringify!($callback),
                "`",
            )
        );
    };
    (
        @parseend {
            input: [$unk:tt$($rest:tt)*],
            name: $name:ident,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(
            concat!(
                "error parsing name: expected `</name>`, found `",
                stringify!($unk),
                "`. caller: `",
                stringify!($callback),
                "`",
            )
        );
    };
}
