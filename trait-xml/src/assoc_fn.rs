/// Parses a `<assocfn></assocfn>` section.
///
/// Expected initial call example:
/// ```
/// trait_xml::trait_xml_parse_assoc_fn! {
///     @parse {
///         input: [
///                     <name>bar</name>
///                     <extern>"C"</extern>
///                     <unsafe/>
///                     <params>
///                         <param>
///                             <name>self</name>
///                             <type>Self</type>
///                         </param>
///                     </params>
///                 </assocfn>
///             </trait>
///         ],
///         name: ,
///         qualifiers: [
///             unsafe: [],
///             extern: [],
///         ],
///         gparams: [],
///         params: [],
///         hasparams: [],
///         ret: ,
///         where: [],
///         def: [],
///         hasdef: [],
///         callback: [
///             name: trait_xml::trait_xml_inner,
///             rule: [@afcallback],
///             args: [
///                 output: [[name Foo]],
///             ],
///         ],
///     }
/// }
/// ```
#[macro_export]
macro_rules! trait_xml_parse_assoc_fn {
    // Empty input error
    (
        @parse {
            input: [],
            name: $($name:ident)?,
            qualifiers: $qt:tt,
            gparams: $gpt:tt,
            params: $pt:tt,
            hasparams: $hpt:tt,
            ret: $($rt:ty)?,
            where: $wt:tt,
            def: $dt:tt,
            hasdef: $hdt:tt,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(concat!(
            "error parsing associated function: ran out of tokens. caller: `",
            stringify!($callback),
            "`",
        ));
    };

    // Successfully parsed associated function
    (
        @parse {
            input: [</assocfn>$($rest:tt)*],
            name: $name:ident,
            qualifiers: $qt:tt,
            gparams: $gpt:tt,
            params: $pt:tt,
            hasparams: $hpt:tt,
            ret: $($rt:ty)?,
            where: $wt:tt,
            def: $dt:tt,
            hasdef: $hdt:tt,
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
                assoc fn: [
                    af
                    [$name]
                    $qt
                    $gpt
                    $pt
                    [$($rt)?]
                    $wt
                    $dt
                ],
            }
        }
    };

    // Other parsing failures
    (
        @parse {
            input: [</assocfn>$($rest:tt)*],
            name: ,
            qualifiers: $qt:tt,
            gparams: $gpt:tt,
            params: $pt:tt,
            hasparams: $hpt:tt,
            ret: $($rt:ty)?,
            where: $wt:tt,
            def: $dt:tt,
            hasdef: $hdt:tt,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(concat!(
            "error parsing associated function: no name provided. caller: `",
            stringify!($callback),
            "`",
        ));
    };

    // Parse name
    (
        @parse {
            input: [<name>$($rest:tt)*],
            name: $name:ident,
            qualifiers: $qt:tt,
            gparams: $gpt:tt,
            params: $pt:tt,
            hasparams: $hpt:tt,
            ret: $($rt:ty)?,
            where: $wt:tt,
            def: $dt:tt,
            hasdef: $hdt:tt,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(concat!(
            "error parsing associated function: name already defined as `",
            stringify!($name),
            "`. caller: `",
            stringify!($callback),
            "`",
        ));
    };
    (
        @parse {
            input: [<name>$($rest:tt)*],
            name: ,
            qualifiers: $qt:tt,
            gparams: $gpt:tt,
            params: $pt:tt,
            hasparams: $hpt:tt,
            ret: $($rt:ty)?,
            where: $wt:tt,
            def: $dt:tt,
            hasdef: $hdt:tt,
            callback: $callbacktoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_name_ident! {
            @parse {
                input: [$($rest)*],
                name: ,
                callback: [
                    name: $crate::trait_xml_parse_assoc_fn,
                    rule: [@namecallback],
                    args: [
                        qualifiers: $qt,
                        gparams: $gpt,
                        params: $pt,
                        hasparams: $hpt,
                        ret: [$($rt)?],
                        where: $wt,
                        def: $dt,
                        hasdef: $hdt,
                        callback: $callbacktoks,
                    ],
                ],
            }
        }
    };

    // Parse qualifiers
    // `unsafe` qualifier
    (
        @parse {
            input: [<unsafe/>$($rest:tt)*],
            name: $($name:ident)?,
            qualifiers: [
                unsafe: [unsafe],
                extern: [$($extern:tt)?],
            ],
            gparams: $gpt:tt,
            params: $pt:tt,
            hasparams: $hpt:tt,
            ret: $($rt:ty)?,
            where: $wt:tt,
            def: $dt:tt,
            hasdef: $hdt:tt,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(concat!(
            "error parsing associated function: already qualified as `unsafe`, but encountered ",
            "another `<unsafe/>` tag. caller: `",
            stringify!($callback),
            "`",
        ));
    };
    (
        @parse {
            input: [<unsafe/>$($rest:tt)*],
            name: $($name:ident)?,
            qualifiers: [
                unsafe: [],
                extern: [$($extern:tt)?],
            ],
            gparams: $gpt:tt,
            params: $pt:tt,
            hasparams: $hpt:tt,
            ret: $($rt:ty)?,
            where: $wt:tt,
            def: $dt:tt,
            hasdef: $hdt:tt,
            callback: $callbacktoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_assoc_fn! {
            @parse {
                input: [$($rest)*],
                name: $($name)?,
                qualifiers: [
                    unsafe: [unsafe],
                    extern: [$($extern)?],
                ],
                gparams: $gpt,
                params: $pt,
                hasparams: $hpt,
                ret: $($rt)?,
                where: $wt,
                def: $dt,
                hasdef: $hdt,
                callback: $callbacktoks,
            }
        }
    };
    // `extern` qualifier
    (
        @parse {
            input: [<extern>$($rest:tt)*],
            name: $($name:ident)?,
            qualifiers: [
                unsafe: [$($unsafe:tt)?],
                extern: [$extern:literal],
            ],
            gparams: $gpt:tt,
            params: $pt:tt,
            hasparams: $hpt:tt,
            ret: $($rt:ty)?,
            where: $wt:tt,
            def: $dt:tt,
            hasdef: $hdt:tt,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(concat!(
            "error parsing associated function: already qualified as `extern ",
            stringify!($extern),
            "`, but encountered ",
            "another `<extern>` tag. caller: `",
            stringify!($callback),
            "`",
        ));
    };
    (
        @parse {
            input: [<extern>$($rest:tt)*],
            name: $($name:ident)?,
            qualifiers: [
                unsafe: [$($unsafe:tt)?],
                extern: [],
            ],
            gparams: $gpt:tt,
            params: $pt:tt,
            hasparams: $hpt:tt,
            ret: $($rt:ty)?,
            where: $wt:tt,
            def: $dt:tt,
            hasdef: $hdt:tt,
            callback: $callbacktoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_assoc_fn! {
            @parseextern {
                input: [$($rest)*],
                name: $($name)?,
                qualifiers: [
                    unsafe: [$($unsafe)?],
                    extern: [],
                ],
                gparams: $gpt,
                params: $pt,
                hasparams: $hpt,
                ret: $($rt)?,
                where: $wt,
                def: $dt,
                hasdef: $hdt,
                callback: $callbacktoks,
            }
        }
    };
    (
        @parseextern {
            input: [$extern:literal$($rest:tt)*],
            name: $($name:ident)?,
            qualifiers: [
                unsafe: [$($unsafe:tt)?],
                extern: [],
            ],
            gparams: $gpt:tt,
            params: $pt:tt,
            hasparams: $hpt:tt,
            ret: $($rt:ty)?,
            where: $wt:tt,
            def: $dt:tt,
            hasdef: $hdt:tt,
            callback: $callbacktoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_assoc_fn! {
            @parseexternend {
                input: [$($rest)*],
                name: $($name)?,
                qualifiers: [
                    unsafe: [$($unsafe)?],
                    extern: [$extern],
                ],
                gparams: $gpt,
                params: $pt,
                hasparams: $hpt,
                ret: $($rt)?,
                where: $wt,
                def: $dt,
                hasdef: $hdt,
                callback: $callbacktoks,
            }
        }
    };
    (
        @parseextern {
            input: [$unx:tt$($rest:tt)*],
            name: $($name:ident)?,
            qualifiers: [
                unsafe: [$($unsafe:tt)?],
                extern: [],
            ],
            gparams: $gpt:tt,
            params: $pt:tt,
            hasparams: $hpt:tt,
            ret: $($rt:ty)?,
            where: $wt:tt,
            def: $dt:tt,
            hasdef: $hdt:tt,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(concat!(
            "error parsing associated function `extern` qualifier: expected literal but found `",
            stringify!($unx),
            "`. caller: `",
            stringify!($callback),
            "`",
        ));
    };
    (
        @parseexternend {
            input: [</extern>$($rest:tt)*],
            name: $($name:ident)?,
            qualifiers: $qt:tt,
            gparams: $gpt:tt,
            params: $pt:tt,
            hasparams: $hpt:tt,
            ret: $($rt:ty)?,
            where: $wt:tt,
            def: $dt:tt,
            hasdef: $hdt:tt,
            callback: $callbacktoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_assoc_fn! {
            @parse {
                input: [$($rest)*],
                name: $($name)?,
                qualifiers: $qt,
                gparams: $gpt,
                params: $pt,
                hasparams: $hpt,
                ret: $($rt)?,
                where: $wt,
                def: $dt,
                hasdef: $hdt,
                callback: $callbacktoks,
            }
        }
    };
    (
        @parseexternend {
            input: [</$unx:tt$($rest:tt)*],
            name: $($name:ident)?,
            qualifiers: $qt:tt,
            gparams: $gpt:tt,
            params: $pt:tt,
            hasparams: $hpt:tt,
            ret: $($rt:ty)?,
            where: $wt:tt,
            def: $dt:tt,
            hasdef: $hdt:tt,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(concat!(
            "error parsing associated function: expected `</extern>`, instead found unexpected ",
            "end tag `",
            stringify!($unx),
            "`. caller: `",
            stringify!($callback),
            "`",
        ));
    };
    (
        @parseexternend {
            input: [<$unx:tt$($rest:tt)*],
            name: $($name:ident)?,
            qualifiers: $qt:tt,
            gparams: $gpt:tt,
            params: $pt:tt,
            hasparams: $hpt:tt,
            ret: $($rt:ty)?,
            where: $wt:tt,
            def: $dt:tt,
            hasdef: $hdt:tt,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(concat!(
            "error parsing associated function: expected `</extern>`, instead found unexpected ",
            "start tag `",
            stringify!($unx),
            "`. caller: `",
            stringify!($callback),
            "`",
        ));
    };
    (
        @parseexternend {
            input: [$unx:tt$($rest:tt)*],
            name: $($name:ident)?,
            qualifiers: $qt:tt,
            gparams: $gpt:tt,
            params: $pt:tt,
            hasparams: $hpt:tt,
            ret: $($rt:ty)?,
            where: $wt:tt,
            def: $dt:tt,
            hasdef: $hdt:tt,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(concat!(
            "error parsing associated function: expected `</extern>`, instead found unexpected ",
            "token `",
            stringify!($unx),
            "`. caller: `",
            stringify!($callback),
            "`",
        ));
    };

    // Parse generic parameters
    (
        @parse {
            input: [<gparams>$($rest:tt)*],
            name: $($name:ident)?,
            qualifiers: $qt:tt,
            gparams: [$($gpt:tt)+],
            params: $pt:tt,
            hasparams: $hpt:tt,
            ret: $($rt:ty)?,
            where: $wt:tt,
            def: $dt:tt,
            hasdef: $hdt:tt,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(concat!(
            "error parsing associated function: multiple `<gparams>` sections found. caller: `",
            stringify!($callback),
            "`",
        ));
    };
    (
        @parse {
            input: [<gparams>$($rest:tt)*],
            name: $($name:ident)?,
            qualifiers: $qt:tt,
            gparams: [],
            params: $pt:tt,
            hasparams: $hpt:tt,
            ret: $($rt:ty)?,
            where: $wt:tt,
            def: $dt:tt,
            hasdef: $hdt:tt,
            callback: $callbacktoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_gparams! {
            @parse {
                input: [$($rest)*],
                consts: [],
                lifetimes: [],
                types: [],
                callback: [
                    name: $crate::trait_xml_parse_assoc_fn,
                    rule: [@gpcallback],
                    args: [
                        name: [$($name)?],
                        qualifiers: $qt,
                        params: $pt,
                        hasparams: $hpt,
                        ret: [$($rt)?],
                        where: $wt,
                        def: $dt,
                        hasdef: $hdt,
                        callback: $callbacktoks,
                    ],
                ],
            }
        }
    };

    // Parse parameters
    (
        @parse {
            input: [<params>$($rest:tt)*],
            name: $($name:ident)?,
            qualifiers: $qt:tt,
            gparams: $gpt:tt,
            params: $pt:tt,
            hasparams: [[]],
            ret: $($rt:ty)?,
            where: $wt:tt,
            def: $dt:tt,
            hasdef: $hdt:tt,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(concat!(
            "error parsing associated function: encountered multiple `<params>` sections. ",
            "caller: `",
            stringify!($callback),
            "`",
        ));
    };
    (
        @parse {
            input: [<params>$($rest:tt)*],
            name: $($name:ident)?,
            qualifiers: $qt:tt,
            gparams: $gpt:tt,
            params: [],
            hasparams: [],
            ret: $($rt:ty)?,
            where: $wt:tt,
            def: $dt:tt,
            hasdef: $hdt:tt,
            callback: $callbacktoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_assoc_fn! {
            @parseparams {
                input: [$($rest)*],
                name: $($name)?,
                qualifiers: $qt,
                gparams: $gpt,
                params: [],
                hasparams: [[]],
                ret: $($rt)?,
                where: $wt,
                def: $dt,
                hasdef: $hdt,
                callback: $callbacktoks,
            }
        }
    };
    (
        @parseparams {
            input: [],
            name: $($name:ident)?,
            qualifiers: $qt:tt,
            gparams: $gpt:tt,
            params: $pt:tt,
            hasparams: $hpt:tt,
            ret: $($rt:ty)?,
            where: $wt:tt,
            def: $dt:tt,
            hasdef: $hdt:tt,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(concat!(
            "error parsing associated function parameters: ran out of tokens. caller: `",
            stringify!($callback),
            "`",
        ));
    };
    (
        @parseparams {
            input: [</params>$($rest:tt)*],
            name: $($name:ident)?,
            qualifiers: $qt:tt,
            gparams: $gpt:tt,
            params: $pt:tt,
            hasparams: $hpt:tt,
            ret: $($rt:ty)?,
            where: $wt:tt,
            def: $dt:tt,
            hasdef: $hdt:tt,
            callback: $callbacktoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_assoc_fn! {
            @parse {
                input: [$($rest)*],
                name: $($name)?,
                qualifiers: $qt,
                gparams: $gpt,
                params: $pt,
                hasparams: $hpt,
                ret: $($rt)?,
                where: $wt,
                def: $dt,
                hasdef: $hdt,
                callback: $callbacktoks,
            }
        }
    };
    (
        @parseparams {
            input: [<param>$($rest:tt)*],
            name: $($name:ident)?,
            qualifiers: $qt:tt,
            gparams: $gpt:tt,
            params: $pt:tt,
            hasparams: $hpt:tt,
            ret: $($rt:ty)?,
            where: $wt:tt,
            def: $dt:tt,
            hasdef: $hdt:tt,
            callback: $callbacktoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_assoc_fn! {
            @parseparam {
                input: [$($rest)*],
                name: $($name)?,
                qualifiers: $qt,
                gparams: $gpt,
                params: $pt,
                hasparams: $hpt,
                ret: $($rt)?,
                where: $wt,
                def: $dt,
                hasdef: $hdt,
                callback: $callbacktoks,
                arg: [],
                type: ,
            }
        }
    };
    (
        @parseparams {
            input: [</$unk:tt$($rest:tt)*],
            name: $($name:ident)?,
            qualifiers: $qt:tt,
            gparams: $gpt:tt,
            params: $pt:tt,
            hasparams: $hpt:tt,
            ret: $($rt:ty)?,
            where: $wt:tt,
            def: $dt:tt,
            hasdef: $hdt:tt,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(concat!(
            "error parsing associated function parameters: encountered unknown end tag `",
            stringify!($unk),
            "`. caller: `",
            stringify!($callback),
            "`",
        ));
    };
    (
        @parseparams {
            input: [<$unk:tt$($rest:tt)*],
            name: $($name:ident)?,
            qualifiers: $qt:tt,
            gparams: $gpt:tt,
            params: $pt:tt,
            hasparams: $hpt:tt,
            ret: $($rt:ty)?,
            where: $wt:tt,
            def: $dt:tt,
            hasdef: $hdt:tt,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(concat!(
            "error parsing associated function parameters: encountered unknown start tag `",
            stringify!($unk),
            "`. caller: `",
            stringify!($callback),
            "`",
        ));
    };
    (
        @parseparams {
            input: [$unx:tt$($rest:tt)*],
            name: $($name:ident)?,
            qualifiers: $qt:tt,
            gparams: $gpt:tt,
            params: $pt:tt,
            hasparams: $hpt:tt,
            ret: $($rt:ty)?,
            where: $wt:tt,
            def: $dt:tt,
            hasdef: $hdt:tt,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(concat!(
            "error parsing associated function parameters: encountered unexpected token `",
            stringify!($unx),
            "`. caller: `",
            stringify!($callback),
            "`",
        ));
    };

    // Parse parameter
    (
        @parseparam {
            input: [],
            name: $($name:ident)?,
            qualifiers: $qt:tt,
            gparams: $gpt:tt,
            params: $pt:tt,
            hasparams: $hpt:tt,
            ret: $($rt:ty)?,
            where: $wt:tt,
            def: $dt:tt,
            hasdef: $hdt:tt,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
            arg: $argtoks:tt,
            type: $($type:ty)?,
        }
    ) => {
        compile_error!(concat!(
            "error parsing associated function parameter: ran out of tokens. caller: `",
            stringify!($callback),
            "`",
        ));
    };
    (
        @parseparam {
            input: [</param>$($rest:tt)*],
            name: $($name:ident)?,
            qualifiers: $qt:tt,
            gparams: $gpt:tt,
            params: $pt:tt,
            hasparams: $hpt:tt,
            ret: $($rt:ty)?,
            where: $wt:tt,
            def: $dt:tt,
            hasdef: $hdt:tt,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
            arg: [],
            type: ,
        }
    ) => {
        compile_error!(concat!(
            "error parsing associated function parameter: no parameter argument or type given. ",
            "caller: `",
            stringify!($callback),
            "`",
        ));
    };
    (
        @parseparam {
            input: [</param>$($rest:tt)*],
            name: $($name:ident)?,
            qualifiers: $qt:tt,
            gparams: $gpt:tt,
            params: $pt:tt,
            hasparams: $hpt:tt,
            ret: $($rt:ty)?,
            where: $wt:tt,
            def: $dt:tt,
            hasdef: $hdt:tt,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
            arg: [$($argtok:tt)+],
            type: ,
        }
    ) => {
        compile_error!(concat!(
            "error parsing associated function parameter: no type given. caller: `",
            stringify!($callback),
            "`",
        ));
    };
    (
        @parseparam {
            input: [</param>$($rest:tt)*],
            name: $($name:ident)?,
            qualifiers: $qt:tt,
            gparams: $gpt:tt,
            params: $pt:tt,
            hasparams: $hpt:tt,
            ret: $($rt:ty)?,
            where: $wt:tt,
            def: $dt:tt,
            hasdef: $hdt:tt,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
            arg: [],
            type: $type:ty,
        }
    ) => {
        compile_error!(concat!(
            "error parsing associated function parameter: no argument given. caller: `",
            stringify!($callback),
            "`",
        ));
    };
    (
        @parseparam {
            input: [</param>$($rest:tt)*],
            name: $($name:ident)?,
            qualifiers: $qt:tt,
            gparams: $gpt:tt,
            params: [$($pt:tt)*],
            hasparams: $hpt:tt,
            ret: $($rt:ty)?,
            where: $wt:tt,
            def: $dt:tt,
            hasdef: $hdt:tt,
            callback: $callbacktoks:tt,
            arg: [$arg:tt],
            type: $type:ty,
        }
    ) => {
        $crate::trait_xml_parse_assoc_fn! {
            @parseparams {
                input: [$($rest)*],
                name: $($name)?,
                qualifiers: $qt,
                gparams: $gpt,
                params: [$($pt)* [param [$arg] [$type]]],
                hasparams: $hpt,
                ret: $($rt)?,
                where: $wt,
                def: $dt,
                hasdef: $hdt,
                callback: $callbacktoks,
            }
        }
    };
    (
        @parseparam {
            input: [</param>$($rest:tt)*],
            name: $($name:ident)?,
            qualifiers: $qt:tt,
            gparams: $gpt:tt,
            params: $pt:tt,
            hasparams: $hpt:tt,
            ret: $($rt:ty)?,
            where: $wt:tt,
            def: $dt:tt,
            hasdef: $hdt:tt,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
            arg: [$($ainv:tt)+],
            type: $type:ty,
        }
    ) => {
        compile_error!(concat!(
            "error parsing associated function parameter: invalid argument `",
            $(stringify!($ainv),)+
            "`. caller: `",
            stringify!($callback),
            "`",
        ));
    };
    (
        @parseparam {
            input: [<pat>$($rest:tt)*],
            name: $($name:ident)?,
            qualifiers: $qt:tt,
            gparams: $gpt:tt,
            params: $pt:tt,
            hasparams: $hpt:tt,
            ret: $($rt:ty)?,
            where: $wt:tt,
            def: $dt:tt,
            hasdef: $hdt:tt,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
            arg: [$($arg:tt)+],
            type: $($type:ty)?,
        }
    ) => {
        compile_error!(concat!(
            "error parsing associated function parameter: argument already defined as `",
            $(stringify!($arg),)+
            "`, but encountered a `<pat>` tag. caller: `",
            stringify!($callback),
            "`",
        ));
    };
    (
        @parseparam {
            input: [<pat>$($rest:tt)*],
            name: $($name:ident)?,
            qualifiers: $qt:tt,
            gparams: $gpt:tt,
            params: $pt:tt,
            hasparams: $hpt:tt,
            ret: $($rt:ty)?,
            where: $wt:tt,
            def: $dt:tt,
            hasdef: $hdt:tt,
            callback: $callbacktoks:tt,
            arg: [],
            type: $($type:ty)?,
        }
    ) => {
        $crate::trait_xml_parse_assoc_fn! {
            @parseparamarg {
                input: [$($rest)*],
                name: $($name)?,
                qualifiers: $qt,
                gparams: $gpt,
                params: $pt,
                hasparams: $hpt,
                ret: $($rt)?,
                where: $wt,
                def: $dt,
                hasdef: $hdt,
                callback: $callbacktoks,
                arg: [],
                type: $($type)?,
            }
        }
    };
    (
        @parseparam {
            input: [<name>$($rest:tt)*],
            name: $($name:ident)?,
            qualifiers: $qt:tt,
            gparams: $gpt:tt,
            params: $pt:tt,
            hasparams: $hpt:tt,
            ret: $($rt:ty)?,
            where: $wt:tt,
            def: $dt:tt,
            hasdef: $hdt:tt,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
            arg: [$($arg:tt)+],
            type: $($type:ty)?,
        }
    ) => {
        compile_error!(concat!(
            "error parsing associated function parameter: argument already defined as `",
            $(stringify!($arg),)+
            "`, but encountered a `<name>` tag. caller: `",
            stringify!($callback),
            "`",
        ));
    };
    (
        @parseparam {
            input: [<name>$($rest:tt)*],
            name: $($name:ident)?,
            qualifiers: $qt:tt,
            gparams: $gpt:tt,
            params: $pt:tt,
            hasparams: $hpt:tt,
            ret: $($rt:ty)?,
            where: $wt:tt,
            def: $dt:tt,
            hasdef: $hdt:tt,
            callback: $callbacktoks:tt,
            arg: [],
            type: $($type:ty)?,
        }
    ) => {
        $crate::trait_xml_parse_name_ident! {
            @parse {
                input: [$($rest)*],
                name: ,
                callback: [
                    name: $crate::trait_xml_parse_assoc_fn,
                    rule: [@paramnamecallback],
                    args: [
                        name: [$($name)?],
                        qualifiers: $qt,
                        gparams: $gpt,
                        params: $pt,
                        hasparams: $hpt,
                        ret: [$($rt)?],
                        where: $wt,
                        def: $dt,
                        hasdef: $hdt,
                        callback: $callbacktoks,
                        type: [$($type)?],
                    ],
                ],
            }
        }
    };
    (
        @parseparam {
            input: [<type>$($rest:tt)*],
            name: $($name:ident)?,
            qualifiers: $qt:tt,
            gparams: $gpt:tt,
            params: $pt:tt,
            hasparams: $hpt:tt,
            ret: $($rt:ty)?,
            where: $wt:tt,
            def: $dt:tt,
            hasdef: $hdt:tt,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
            arg: $argtoks:tt,
            type: $type:ty,
        }
    ) => {
        compile_error!(concat!(
            "error parsing associated function parameter: type already defined as `",
            stringify!($type),
            "`, but encountered another `<type>` tag. caller: `",
            stringify!($callback),
            "`",
        ));
    };
    (
        @parseparam {
            input: [<type>$($rest:tt)*],
            name: $($name:ident)?,
            qualifiers: $qt:tt,
            gparams: $gpt:tt,
            params: $pt:tt,
            hasparams: $hpt:tt,
            ret: $($rt:ty)?,
            where: $wt:tt,
            def: $dt:tt,
            hasdef: $hdt:tt,
            callback: $callbacktoks:tt,
            arg: $argtoks:tt,
            type: ,
        }
    ) => {
        $crate::trait_xml_parse_type_ty! {
            @parse {
                input: [$($rest)*],
                type: ,
                tokens: [],
                callback: [
                    name: $crate::trait_xml_parse_assoc_fn,
                    rule: [@paramtypecallback],
                    args: [
                        name: [$($name)?],
                        qualifiers: $qt,
                        gparams: $gpt,
                        params: $pt,
                        hasparams: $hpt,
                        ret: [$($rt)?],
                        where: $wt,
                        def: $dt,
                        hasdef: $hdt,
                        callback: $callbacktoks,
                        arg: $argtoks,
                    ],
                ],
            }
        }
    };
    (
        @parseparam {
            input: [</$unk:tt$($rest:tt)*],
            name: $($name:ident)?,
            qualifiers: $qt:tt,
            gparams: $gpt:tt,
            params: $pt:tt,
            hasparams: $hpt:tt,
            ret: $($rt:ty)?,
            where: $wt:tt,
            def: $dt:tt,
            hasdef: $hdt:tt,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
            arg: $argtoks:tt,
            type: $($type:ty)?,
        }
    ) => {
        compile_error!(concat!(
            "error parsing associated function parameter: encountered unknown end tag `",
            stringify!($unk),
            "`. caller: `",
            stringify!($callback),
            "`",
        ));
    };
    (
        @parseparam {
            input: [<$unk:tt$($rest:tt)*],
            name: $($name:ident)?,
            qualifiers: $qt:tt,
            gparams: $gpt:tt,
            params: $pt:tt,
            hasparams: $hpt:tt,
            ret: $($rt:ty)?,
            where: $wt:tt,
            def: $dt:tt,
            hasdef: $hdt:tt,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
            arg: $argtoks:tt,
            type: $($type:ty)?,
        }
    ) => {
        compile_error!(concat!(
            "error parsing associated function parameter: encountered unknown start tag `",
            stringify!($unk),
            "`. caller: `",
            stringify!($callback),
            "`",
        ));
    };
    (
        @parseparam {
            input: [$unx:tt$($rest:tt)*],
            name: $($name:ident)?,
            qualifiers: $qt:tt,
            gparams: $gpt:tt,
            params: $pt:tt,
            hasparams: $hpt:tt,
            ret: $($rt:ty)?,
            where: $wt:tt,
            def: $dt:tt,
            hasdef: $hdt:tt,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
            arg: $argtoks:tt,
            type: $($type:ty)?,
        }
    ) => {
        compile_error!(concat!(
            "error parsing associated function parameter: encountered unexpected token `",
            stringify!($unx),
            "`. caller: `",
            stringify!($callback),
            "`",
        ));
    };

    // Parse parameter arg
    (
        @parseparamarg {
            input: [</arg>$($rest:tt)*],
            name: $($name:ident)?,
            qualifiers: $qt:tt,
            gparams: $gpt:tt,
            params: $pt:tt,
            hasparams: $hpt:tt,
            ret: $($rt:ty)?,
            where: $wt:tt,
            def: $dt:tt,
            hasdef: $hdt:tt,
            callback: $callbacktoks:tt,
            arg: [$($argtok:tt)+],
            type: $($type:ty)?,
        }
    ) => {
        $crate::trait_xml_parse_assoc_fn! {
            @parseparam {
                input: [$($rest)*],
                name: $($name)?,
                qualifiers: $qt,
                gparams: $gpt,
                params: $pt,
                hasparams: $hpt,
                ret: $($rt)?,
                where: $wt,
                def: $dt,
                hasdef: $hdt,
                callback: $callbacktoks,
                arg: [$($argtok)+],
                type: $($type)?,
            }
        }
    };
    (
        @parseparamarg {
            input: [$first:tt$($rest:tt)*],
            name: $($name:ident)?,
            qualifiers: $qt:tt,
            gparams: $gpt:tt,
            params: $pt:tt,
            hasparams: $hpt:tt,
            ret: $($rt:ty)?,
            where: $wt:tt,
            def: $dt:tt,
            hasdef: $hdt:tt,
            callback: $callbacktoks:tt,
            arg: [$($argtok:tt)*],
            type: $($type:ty)?,
        }
    ) => {
        $crate::trait_xml_parse_assoc_fn! {
            @parseparamarg {
                input: [$($rest)*],
                name: $($name)?,
                qualifiers: $qt,
                gparams: $gpt,
                params: $pt,
                hasparams: $hpt,
                ret: $($rt)?,
                where: $wt,
                def: $dt,
                hasdef: $hdt,
                callback: $callbacktoks,
                arg: [$($argtok)* $first],
                type: $($type)?,
            }
        }
    };

    // Parse return type
    (
        @parse {
            input: [<ret>$($rest:tt)*],
            name: $($name:ident)?,
            qualifiers: $qt:tt,
            gparams: $gpt:tt,
            params: $pt:tt,
            hasparams: $hpt:tt,
            ret: $rt:ty,
            where: $wt:tt,
            def: $dt:tt,
            hasdef: $hdt:tt,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(concat!(
            "error parsing associated function: return type already defined as `",
            stringify!($rt),
            "`, but encountered another `<ret>` tag. caller: `",
            stringify!($callback),
            "`",
        ));
    };
    (
        @parse {
            input: [<ret>$($rest:tt)*],
            name: $($name:ident)?,
            qualifiers: $qt:tt,
            gparams: $gpt:tt,
            params: $pt:tt,
            hasparams: $hpt:tt,
            ret: ,
            where: $wt:tt,
            def: $dt:tt,
            hasdef: $hdt:tt,
            callback: $callbacktoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_assoc_fn! {
            @parseret {
                input: [$($rest)*],
                name: $($name)?,
                qualifiers: $qt,
                gparams: $gpt,
                params: $pt,
                hasparams: $hpt,
                ret: [],
                where: $wt,
                def: $dt,
                hasdef: $hdt,
                callback: $callbacktoks,
            }
        }
    };
    (
        @parseret {
            input: [</ret>$($rest:tt)*],
            name: $($name:ident)?,
            qualifiers: $qt:tt,
            gparams: $gpt:tt,
            params: $pt:tt,
            hasparams: $hpt:tt,
            ret: [],
            where: $wt:tt,
            def: $dt:tt,
            hasdef: $hdt:tt,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(concat!(
            "error parsing associated function return type: empty return type. caller: `",
            stringify!($callback),
            "`",
        ));
    };
    (
        @parseret {
            input: [</ret>$($rest:tt)*],
            name: $($name:ident)?,
            qualifiers: $qt:tt,
            gparams: $gpt:tt,
            params: $pt:tt,
            hasparams: $hpt:tt,
            ret: [$rt:ty],
            where: $wt:tt,
            def: $dt:tt,
            hasdef: $hdt:tt,
            callback: $callbacktoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_assoc_fn! {
            @parse {
                input: [$($rest)*],
                name: $($name)?,
                qualifiers: $qt,
                gparams: $gpt,
                params: $pt,
                hasparams: $hpt,
                ret: $rt,
                where: $wt,
                def: $dt,
                hasdef: $hdt,
                callback: $callbacktoks,
            }
        }
    };
    (
        @parseret {
            input: [</ret>$($rest:tt)*],
            name: $($name:ident)?,
            qualifiers: $qt:tt,
            gparams: $gpt:tt,
            params: $pt:tt,
            hasparams: $hpt:tt,
            ret: [$($rtinv:tt)+],
            where: $wt:tt,
            def: $dt:tt,
            hasdef: $hdt:tt,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(concat!(
            "error parsing associated function return type: input forms invalid type `",
            $(stringify!($rtinv),)+
            "`. caller: `",
            stringify!($callback),
            "`",
        ));
    };
    (
        @parseret {
            input: [$first:tt$($rest:tt)*],
            name: $($name:ident)?,
            qualifiers: $qt:tt,
            gparams: $gpt:tt,
            params: $pt:tt,
            hasparams: $hpt:tt,
            ret: [$($rt:tt)*],
            where: $wt:tt,
            def: $dt:tt,
            hasdef: $hdt:tt,
            callback: $callbacktoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_assoc_fn! {
            @parseret {
                input: [$($rest)*],
                name: $($name)?,
                qualifiers: $qt,
                gparams: $gpt,
                params: $pt,
                hasparams: $hpt,
                ret: [$($rt)* $first],
                where: $wt,
                def: $dt,
                hasdef: $hdt,
                callback: $callbacktoks,
            }
        }
    };

    // Parse `where` clause
    (
        @parse {
            input: [<where>$($rest:tt)*],
            name: $($name:ident)?,
            qualifiers: $qt:tt,
            gparams: $gpt:tt,
            params: $pt:tt,
            hasparams: $hpt:tt,
            ret: $($rt:ty)?,
            where: [$($wt:tt)+],
            def: $dt:tt,
            hasdef: $hdt:tt,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(concat!(
            "error parsing associated function: encountered multiple `where` clauses. caller: `",
            stringify!($callback),
            "`",
        ));
    };
    (
        @parse {
            input: [<where>$($rest:tt)*],
            name: $($name:ident)?,
            qualifiers: $qt:tt,
            gparams: $gpt:tt,
            params: $pt:tt,
            hasparams: $hpt:tt,
            ret: $($rt:ty)?,
            where: [],
            def: $dt:tt,
            hasdef: $hdt:tt,
            callback: $callbacktoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_where! {
            @parse {
                input: [$($rest)*],
                clauses: [],
                callback: [
                    name: $crate::trait_xml_parse_assoc_fn,
                    rule: [@wherecallback],
                    args: [
                        name: [$($name)?],
                        qualifiers: $qt,
                        gparams: $gpt,
                        params: $pt,
                        hasparams: $hpt,
                        ret: [$($rt)?],
                        def: $dt,
                        hasdef: $hdt,
                        callback: $callbacktoks,
                    ],
                ],
            }
        }
    };

    // Parse default function defs
    (
        @parse {
            input: [<rust>$($rest:tt)*],
            name: $($name:ident)?,
            qualifiers: $qt:tt,
            gparams: $gpt:tt,
            params: $pt:tt,
            hasparams: $hpt:tt,
            ret: $($rt:ty)?,
            where: $wt:tt,
            def: $dt:tt,
            hasdef: [[]],
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(concat!(
            "error parsing associated function: multiple `<rust>` tags encountered. caller: `",
            stringify!($callback),
            "`",
        ));
    };
    (
        @parse {
            input: [<rust>$($rest:tt)*],
            name: $($name:ident)?,
            qualifiers: $qt:tt,
            gparams: $gpt:tt,
            params: $pt:tt,
            hasparams: $hpt:tt,
            ret: $($rt:ty)?,
            where: $wt:tt,
            def: [],
            hasdef: [],
            callback: $callbacktoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_assoc_fn! {
            @parserust {
                input: [$($rest)*],
                name: $($name)?,
                qualifiers: $qt,
                gparams: $gpt,
                params: $pt,
                hasparams: $hpt,
                ret: $($rt)?,
                where: $wt,
                def: [],
                hasdef: [[]],
                callback: $callbacktoks,
            }
        }
    };
    (
        @parserust {
            input: [</rust>$($rest:tt)*],
            name: $($name:ident)?,
            qualifiers: $qt:tt,
            gparams: $gpt:tt,
            params: $pt:tt,
            hasparams: $hpt:tt,
            ret: $($rt:ty)?,
            where: $wt:tt,
            def: $dt:tt,
            hasdef: $hdt:tt,
            callback: $callbacktoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_assoc_fn! {
            @parse {
                input: [$($rest)*],
                name: $($name)?,
                qualifiers: $qt,
                gparams: $gpt,
                params: $pt,
                hasparams: $hpt,
                ret: $($rt)?,
                where: $wt,
                def: $dt,
                hasdef: $hdt,
                callback: $callbacktoks,
            }
        }
    };
    (
        @parserust {
            input: [$first:tt$($rest:tt)*],
            name: $($name:ident)?,
            qualifiers: $qt:tt,
            gparams: $gpt:tt,
            params: $pt:tt,
            hasparams: $hpt:tt,
            ret: $($rt:ty)?,
            where: $wt:tt,
            def: [$($dt:tt)*],
            hasdef: $hdt:tt,
            callback: $callbacktoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_assoc_fn! {
            @parserust {
                input: [$($rest)*],
                name: $($name)?,
                qualifiers: $qt,
                gparams: $gpt,
                params: $pt,
                hasparams: $hpt,
                ret: $($rt)?,
                where: $wt,
                def: [$($dt)* $first],
                hasdef: $hdt,
                callback: $callbacktoks,
            }
        }
    };
    (
        @parserust {
            input: [],
            name: $($name:ident)?,
            qualifiers: $qt:tt,
            gparams: $gpt:tt,
            params: $pt:tt,
            hasparams: $hpt:tt,
            ret: $($rt:ty)?,
            where: $wt:tt,
            def: $dt:tt,
            hasdef: $hdt:tt,
            callback: [
                name: $callback:path,
                rule: $ruletoks:tt,
                args: $argstoks:tt,
            ],
        }
    ) => {
        compile_error!(concat!(
            "error parsing associated function default definition: ran out of tokens. caller: `",
            stringify!($callback),
            "`",
        ));
    };

    // Catch callbacks
    (
        @namecallback {
            input: $inputtoks:tt,
            qualifiers: $qt:tt,
            gparams: $gpt:tt,
            params: $pt:tt,
            hasparams: $hpt:tt,
            ret: [$($rt:ty)?],
            where: $wt:tt,
            def: $dt:tt,
            hasdef: $hdt:tt,
            callback: $callbacktoks:tt,
            name: $name:ident,
        }
    ) => {
        $crate::trait_xml_parse_assoc_fn! {
            @parse {
                input: $inputtoks,
                name: $name,
                qualifiers: $qt,
                gparams: $gpt,
                params: $pt,
                hasparams: $hpt,
                ret: $($rt)?,
                where: $wt,
                def: $dt,
                hasdef: $hdt,
                callback: $callbacktoks,
            }
        }
    };
    (
        @gpcallback {
            input: $inputtoks:tt,
            name: [$($name:ident)?],
            qualifiers: $qt:tt,
            params: $pt:tt,
            hasparams: $hpt:tt,
            ret: [$($rt:ty)?],
            where: $wt:tt,
            def: $dt:tt,
            hasdef: $hdt:tt,
            callback: $callbacktoks:tt,
            gparams: $gpt:tt,
        }
    ) => {
        $crate::trait_xml_parse_assoc_fn! {
            @parse {
                input: $inputtoks,
                name: $($name)?,
                qualifiers: $qt,
                gparams: $gpt,
                params: $pt,
                hasparams: $hpt,
                ret: $($rt)?,
                where: $wt,
                def: $dt,
                hasdef: $hdt,
                callback: $callbacktoks,
            }
        }
    };
    (
        @paramnamecallback {
            input: $inputtoks:tt,
            name: [$($name:ident)?],
            qualifiers: $qt:tt,
            gparams: $gpt:tt,
            params: $pt:tt,
            hasparams: $hpt:tt,
            ret: [$($rt:ty)?],
            where: $wt:tt,
            def: $dt:tt,
            hasdef: $hdt:tt,
            callback: $callbacktoks:tt,
            type: [$($type:ty)?],
            name: $paramname:ident,
        }
    ) => {
        $crate::trait_xml_parse_assoc_fn! {
            @parseparam {
                input: $inputtoks,
                name: $($name)?,
                qualifiers: $qt,
                gparams: $gpt,
                params: $pt,
                hasparams: $hpt,
                ret: $($rt)?,
                where: $wt,
                def: $dt,
                hasdef: $hdt,
                callback: $callbacktoks,
                arg: [$paramname],
                type: $($type)?,
            }
        }
    };
    (
        @paramtypecallback {
            input: $inputtoks:tt,
            name: [$($name:ident)?],
            qualifiers: $qt:tt,
            gparams: $gpt:tt,
            params: $pt:tt,
            hasparams: $hpt:tt,
            ret: [$($rt:ty)?],
            where: $wt:tt,
            def: $dt:tt,
            hasdef: $hdt:tt,
            callback: $callbacktoks:tt,
            arg: $argtoks:tt,
            type: $type:ty,
        }
    ) => {
        $crate::trait_xml_parse_assoc_fn! {
            @parseparam {
                input: $inputtoks,
                name: $($name)?,
                qualifiers: $qt,
                gparams: $gpt,
                params: $pt,
                hasparams: $hpt,
                ret: $($rt)?,
                where: $wt,
                def: $dt,
                hasdef: $hdt,
                callback: $callbacktoks,
                arg: $argtoks,
                type: $type,
            }
        }
    };
    (
        @wherecallback {
            input: $inputtoks:tt,
            name: [$($name:ident)?],
            qualifiers: $qt:tt,
            gparams: $gpt:tt,
            params: $pt:tt,
            hasparams: $hpt:tt,
            ret: [$($rt:ty)?],
            def: $dt:tt,
            hasdef: $hdt:tt,
            callback: $callbacktoks:tt,
            where: $wheretoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_assoc_fn! {
            @parse {
                input: $inputtoks,
                name: $($name)?,
                qualifiers: $qt,
                gparams: $gpt,
                params: $pt,
                hasparams: $hpt,
                ret: $($rt)?,
                where: $wheretoks,
                def: $dt,
                hasdef: $hdt,
                callback: $callbacktoks,
            }
        }
    };
}
