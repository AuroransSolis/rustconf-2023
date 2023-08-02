/// Parses a `<vis></vis>` section.
///
/// Expected initial call example:
/// ```
/// trait_xml::trait_xml_parse_vis! {
///     @parse {
///         input: [
///                 pub</vis>
///             </trait>
///         ],
///         callback: [
///             name: trait_xml::trait_xml_inner,
///             rule: [@viscallback],
///             args: [output: [[name Foo]],],
///         ],
///     }
/// }
#[macro_export]
macro_rules! trait_xml_parse_vis {
    (
        @parse {
            input: [],
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(
            concat!(
                "error parsing visibility: ran out of tokens. caller: `",
                stringify!($callback),
                "`",
            )
        );
    };

    (
        @parse {
            input: [</vis>$($rest:tt)*],
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(
            concat!(
                "error parsing visibility: empty input. caller: `",
                stringify!($callback),
                "`",
            )
        );
    };

    (
        @parse {
            input: [$vis:vis</vis>$($rest:tt)*],
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
                vis: [$vis],
            }
        }
    };

    (
        @parse {
            input: [$inv:tt$($rest:tt)*],
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(
            concat!(
                "error parsing visibility: start of invalid visibility specifier `",
                stringify!($inv),
                "`. caller: `",
                stringify!($callback),
                "`",
            )
        );
    }
}
