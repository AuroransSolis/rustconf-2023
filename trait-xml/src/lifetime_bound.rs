/// Parses a `<lifetime-bound></lifetime-bound>` section.
///
/// Expected initial call example:
/// ```
/// trait_xml::trait_xml_parse_lifetime_bound! {
///     @parse {
///         input: [
///                         'bar</lifetime-bound>
///                     </type>
///                 </bounds>
///             </trait>
///         ],
///         lifetime: ,
///         callback: [
///             name: trait_xml::trait_xml_parse_type,
///             rule: [@lbcallback],
///             args: [
///                 name: [Baz],
///                 bounds: [],
///                 callback: [
///                     name: trait_xml::trait_xml_parse_bounds,
///                     rule: [@typecallback],
///                     args: [
///                         consts: [],
///                         lifetimes: [
///                             [lt 'bar []]
///                         ],
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
macro_rules! trait_xml_parse_lifetime_bound {
    // Parse LT bound - broken up so that proper error handling can be provided.
    // 2.1: unexpected end of input
    (
        @parse {
            input: [],
            lifetime: ,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argtoks:tt,
            ],
        }
    ) => {
        compile_error!(
            concat!(
                "error parsing lifetime bound: expected lifetime, found end of input. ",
                "caller: `",
                stringify!($callback),
                "`",
            )
        );
    };
    // 2.2: valid lifetime
    (
        @parse {
            input: [$lt:lifetime $($rest:tt)*],
            lifetime: ,
            callback: $callbacktoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_lifetime_bound! {
            @parseltbend {
                input: [$($rest)*],
                lifetime: $lt,
                callback: $callbacktoks,
            }
        }
    };
    // 2.3: invalid lifetime
    (
        @parse {
            input: [$unx:tt$($rest:tt)*],
            lifetime: ,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argtoks:tt,
            ],
        }
    ) => {
        compile_error!(
            concat!(
                "error parsing lifetime parameter: expected lifetime, found `",
                stringify!($unx),
                "`. caller: `",
                stringify!($callback),
                "`",
            )
        );
    };
    // 3.1: found end name tag
    (
        @parseltbend {
            input: [</lifetime-bound>$($rest:tt)*],
            lifetime: $lt:lifetime,
            callback: $callbacktoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_lifetime_bound! {
            @makecallback {
                input: [$($rest)*],
                lifetime: $lt,
                callback: $callbacktoks,
            }
        }
    };
    // 3.2: unexpected end of input
    (
        @parseltbend {
            input: [],
            lifetime: $lt:lifetime,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(
            concat!(
                "error parsing lifetime parameter: expected `</lifetime>`, found end of input. ",
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
            lifetime: $lt:lifetime,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(
            concat!(
                "error parsing lifetime parameter: expected `</name>`, found `",
                stringify!($unx),
                "`. caller: `",
                stringify!($callback),
                "`",
            )
        );
    };

    (
        @makecallback {
            input: $inputtoks:tt,
            lifetime: $lt:lifetime,
            callback: [
                name: $callback:path,
                rule: [$($rule:tt)+],
                args: [$($field:tt: $fieldtokens:tt,)+],
            ],
        }
    ) => {
        $callback! {
            $($rule)+ {
                input: $inputtoks,
                $($field: $fieldtokens,)+
                ltbound: $lt,
            }
        }
    };
}
