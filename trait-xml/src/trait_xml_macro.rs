/// # Wow this is dumb!!!
///
/// This macro allows you to define traits with XML. It's just a wrapper around
/// [`trait_xml_inner!`](crate::trait_xml_inner) that accepts everything as valid input. Let's talk
/// about what trait definitions should look like in XML.
///
/// See the top level documentation for information on how to use this.
#[macro_export]
macro_rules! trait_xml {
    ($($input:tt)*) => {
        $crate::trait_xml_inner! {
            @parse {
                input: [$($input)+],
            }
        }
    };
}

/// Start of internal parsing.
#[macro_export]
macro_rules! trait_xml_inner {
    /*
        #####################################################
        #####################################################
        ###                                               ###
        ###   ####    #   ####   #### ##### #   #  ####   ###
        ###   #   #  # #  #   # #       #   ##  # #       ###
        ###   ####   ###  ####   ###    #   # # # #  ##   ###
        ###   #     #   # #  #      #   #   #  ## #   #   ###
        ###   #     #   # #   # ####  ##### #   #  ###    ###
        ###                                               ###
        #####################################################
        #####################################################
    */

    // Entry point
    (
        @parse {
            input: [<trait> $($rest:tt)*],
        }
    ) => {
        $crate::trait_xml_inner! {
            @parsetrait {
                input: [$($rest)+],
                output: [],
            }
        }
    };
    // Entry point failure
    (
        @parse {
            input: [$unx:tt $($rest:tt)*],
        }
    ) => {
        compile_error!(concat!(
            "expected `<trait>`, found unexpected token `",
            stringify!($unx),
            "`",
        ));
    };

    (
        @parsetrait {
            input: [],
            output: [$($out:tt)*],
        }
    ) => {
        compile_error!("error parsing trait: unexpected end of input");
    };

    // Unsafe marker
    (
        @parsetrait {
            input: [<unsafe/> $($rest:tt)*],
            output: [$($out:tt)*],
        }
    ) => {
        $crate::trait_xml_inner! {
            @parsetrait {
                input: [$($rest)*],
                output: [$($out)* [unsafe]],
            }
        }
    };

    // Name
    (
        @parsetrait {
            input: [<name>$($rest:tt)*],
            output: $outtoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_name_ident! {
            @parse {
                input: [$($rest)*],
                name: ,
                callback: [
                    name: $crate::trait_xml_inner,
                    rule: [@namecallback],
                    args: [
                        output: $outtoks,
                    ],
                ],
            }
        }
    };

    // Visibility
    (
        @parsetrait {
            input: [<vis>$($rest:tt)*],
            output: $outtoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_vis! {
            @parse {
                input: [$($rest)*],
                callback: [
                    name: $crate::trait_xml_inner,
                    rule: [@viscallback],
                    args: [
                        output: $outtoks,
                    ],
                ],
            }
        }
    };

    // Generic bounds (lifetimes, types, const generics) and supertraits
    (
        @parsetrait {
            input: [<bounds>$($rest:tt)*],
            output: $outtoks:tt,
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
                    name: $crate::trait_xml_inner,
                    rule: [@boundscallback],
                    args: [
                        output: $outtoks,
                    ],
                ],
            }
        }
    };

    // Where clause
    (
        @parsetrait {
            input: [<where>$($rest:tt)*],
            output: $outtoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_where! {
            @parse {
                input: [$($rest)*],
                clauses: [],
                callback: [
                    name: $crate::trait_xml_inner,
                    rule: [@wherecallback],
                    args: [
                        output: $outtoks,
                    ],
                ],
            }
        }
    };

    // Associated constant
    (
        @parsetrait {
            input: [<assocconst>$($rest:tt)*],
            output: $outtoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_assoc_const! {
            @parse {
                input: [$($rest)*],
                name: ,
                type: ,
                default: ,
                callback: [
                    name: $crate::trait_xml_inner,
                    rule: [@accallback],
                    args: [
                        output: $outtoks,
                    ],
                ],
            }
        }
    };

    // Associated type
    (
        @parsetrait {
            input: [<assoctype>$($rest:tt)*],
            output: $outtoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_assoc_type! {
            @parse {
                input: [$($rest)*],
                name: ,
                gparams: [],
                bounds: [],
                hasbounds: [],
                boundswhere: [],
                hasboundswhere: [],
                callback: [
                    name: $crate::trait_xml_inner,
                    rule: [@atcallback],
                    args: [
                        output: $outtoks,
                    ],
                ],
            }
        }
    };

    // Associated function
    (
        @parsetrait {
            input: [<assocfn>$($rest:tt)*],
            output: $outtoks:tt,
        }
    ) => {
        $crate::trait_xml_parse_assoc_fn! {
            @parse {
                input: [$($rest)*],
                name: ,
                qualifiers: [
                    unsafe: [],
                    extern: [],
                ],
                gparams: [],
                params: [],
                hasparams: [],
                ret: ,
                where: [],
                def: [],
                hasdef: [],
                callback: [
                    name: $crate::trait_xml_inner,
                    rule: [@afcallback],
                    args: [
                        output: $outtoks,
                    ],
                ],
            }
        }
    };

    // Catch callbacks
    (
        @namecallback {
            input: $inputtoks:tt,
            output: [$($output:tt)*],
            name: $name:ident,
        }
    ) => {
        $crate::trait_xml_inner! {
            @parsetrait {
                input: $inputtoks,
                output: [$($output)* [name $name]],
            }
        }
    };
    (
        @viscallback {
            input: $inputtoks:tt,
            output: [$($output:tt)*],
            vis: [$vis:vis],
        }
    ) => {
        $crate::trait_xml_inner! {
            @parsetrait {
                input: $inputtoks,
                output: [$($output)* [vis $vis]],
            }
        }
    };
    (
        @boundscallback {
            input: $inputtoks:tt,
            output: [$($out:tt)*],
            gparams: $gpt:tt,
            reqs: $rt:tt,
        }
    ) => {
        $crate::trait_xml_inner! {
            @parsetrait {
                input: $inputtoks,
                output: [$($out)* [gparams $gpt reqs $rt]],
            }
        }
    };
    (
        @wherecallback {
            input: $inputtoks:tt,
            output: [$($out:tt)*],
            where: [$($clause:tt)*],
        }
    ) => {
        $crate::trait_xml_inner! {
            @parsetrait {
                input: $inputtoks,
                output: [$($out)* [wc $($clause)*]],
            }
        }
    };
    (
        @accallback {
            input: $inputtoks:tt,
            output: [$($out:tt)*],
            assoc const: [ac [$name:ident] [$type:ty] [$($default:expr)?]],
        }
    ) => {
        $crate::trait_xml_inner! {
            @parsetrait {
                input: $inputtoks,
                output: [$($out)* [ac [$name] [$type] [$($default)?]]],
            }
        }
    };
    (
        @atcallback {
            input: $inputtoks:tt,
            output: [$($out:tt)*],
            assoc type: $att:tt,
        }
    ) => {
        $crate::trait_xml_inner! {
            @parsetrait {
                input: $inputtoks,
                output: [$($out)* $att],
            }
        }
    };
    (
        @afcallback {
            input: $inputtoks:tt,
            output: [$($out:tt)*],
            assoc fn: $attoks:tt,
        }
    ) => {
        $crate::trait_xml_inner! {
            @parsetrait {
                input: $inputtoks,
                output: [$($out)* $attoks],
            }
        }
    };

    // End of trait definition
    (
        @parsetrait {
            input: [</trait>],
            output: $outtoks:tt,
        }
    ) => {
        // const _: &str = stringify!($outtoks);
        $crate::trait_xml_inner! {
            @expand {
                output: $outtoks,
                vis: [],
                unsafe: ,
                name: ,
                gparams: [],
                tpbs: [],
                wc: [],
                assoc types: [],
                assoc consts: [],
                fns: [],
            }
        }
    };
    (
        @parsetrait {
            input: [</trait> $($rest:tt)*]
            output: [$($out:tt)*],
        }
    ) => {
        compile_error!(concat!(
            "extraneous tokens after end of trait def: `",
            $(stringify!($rest)),+
            "`",
        ));
    };

    // Unknown tags
    (
        @parsetrait {
            input: [<$unk:tt> $($rest:tt)*],
            output: [$($out:tt)*],
        }
    ) => {
        compile_error!(concat!("unknown start tag: `", stringify!($unk), "`"));
    };
    (
        @parsetrait {
            input: [</$unk:tt> $($rest:tt)*],
            output: [$($out:tt)*],
        }
    ) => {
        compile_error!(concat!("unknown end tag: `", stringify!($unk), "`"));
    };

    /*
        #################################################################
        #################################################################
        ###                                                           ###
        ###   ##### #   # ####    #   #   #  #### #####  ###  #   #   ###
        ###   #      # #  #   #  # #  ##  # #       #   #   # ##  #   ###
        ###   #####   #   ####   ###  # # #  ###    #   #   # # # #   ###
        ###   #      # #  #     #   # #  ##     #   #   #   # #  ##   ###
        ###   ##### #   # #     #   # #   # ####  #####  ###  #   #   ###
        ###                                                           ###
        #################################################################
        #################################################################
    */

    // Expand vis
    (
        @expand {
            output: [[vis $vis:vis] $($out:tt)*],
            vis: [],
            unsafe: $($unsafe:ident)?,
            name: $($name:ident)?,
            gparams: $gpt:tt,
            tpbs: $tpbt:tt,
            wc: $wct:tt,
            assoc types: $att:tt,
            assoc consts: $act:tt,
            fns: $fnt:tt,
        }
    ) => {
        $crate::trait_xml_inner! {
            @expand {
                output: [$($out)*],
                vis: [$vis],
                unsafe: $($unsafe)?,
                name: $($name)?,
                gparams: $gpt,
                tpbs: $tpbt,
                wc: $wct,
                assoc types: $att,
                assoc consts: $act,
                fns: $fnt,
            }
        }
    };
    // Expand vis - already present
    (
        @expand {
            output: [[vis $newvis:vis] $($out:tt)*],
            vis: [$($oldvis:tt)+],
            unsafe: $($unsafe:ident)?,
            name: $($name:ident)?,
            gparams: $gpt:tt,
            tpbs: $tpbt:tt,
            wc: $wct:tt,
            assoc types: $att:tt,
            assoc consts: $act:tt,
            fns: $fnt:tt,
        }
    ) => {
        compile_error!(
            concat!(
                "trait visibility defined multiple times - first as `",
                $(stringify!($oldvis)),+,
                "`, then as `",
                stringify!($newvis),
                "`",
            )
        );
    };

    // Expand unsafe
    (
        @expand {
            output: [[unsafe] $($out:tt)*],
            vis: $vistoks:tt,
            unsafe: ,
            name: $($name:ident)?,
            gparams: $gpt:tt,
            tpbs: $tpbt:tt,
            wc: $wct:tt,
            assoc types: $att:tt,
            assoc consts: $act:tt,
            fns: $fnt:tt,
        }
    ) => {
        $crate::trait_xml_inner! {
            @expand {
                output: [$($out)*],
                vis: $vistoks,
                unsafe: unsafe,
                name: $($name)?,
                gparams: $gpt,
                tpbs: $tpbt,
                wc: $wct,
                assoc types: $att,
                assoc consts: $act,
                fns: $fnt,
            }
        }
    };
    // Expand unsafe - already present
    (
        @expand {
            output: [[unsafe] $($out:tt)*],
            vis: $vistoks:tt,
            unsafe: unsafe,
            name: $($name:ident)?,
            gparams: $gpt:tt,
            tpbs: $tpbt:tt,
            wc: $wct:tt,
            assoc types: $att:tt,
            assoc consts: $act:tt,
            fns: $fnt:tt,
        }
    ) => {
        compile_error!("trait marked as unsafe multiple times");
    };

    // Expand name - no name present
    (
        @expand {
            output: [[name $name:ident] $($out:tt)*],
            vis: $vistoks:tt,
            unsafe: $($unsafe:ident)?,
            name: ,
            gparams: $gpt:tt,
            tpbs: $tpbt:tt,
            wc: $wct:tt,
            assoc types: $att:tt,
            assoc consts: $act:tt,
            fns: $fnt:tt,
        }
    ) => {
        $crate::trait_xml_inner! {
            @expand {
                output: [$($out)*],
                vis: $vistoks,
                unsafe: $($unsafe)?,
                name: $name,
                gparams: $gpt,
                tpbs: $tpbt,
                wc: $wct,
                assoc types: $att,
                assoc consts: $act,
                fns: $fnt,
            }
        }
    };
    // Expand name - name already present
    (
        @expand {
            output: [[name $name:ident] $($out:tt)*],
            vis: $vistoks:tt,
            unsafe: $($unsafe:ident)?,
            name: $presname:ident,
            gparams: $gpt:tt,
            tpbs: $tpbt:tt,
            wc: $wct:tt,
            assoc types: $att:tt,
            assoc consts: $act:tt,
            fns: $fnt:tt,
        }
    ) => {
        compile_error!(concat!(
            "name already present - have `",
            stringify!($presname),
            "`, found `",
            stringify!($name), "`"
        ));
    };

    // Expand gparams - none present
    (
        @expand {
            output: [[gparams $gpt:tt reqs $rt:tt] $($out:tt)*],
            vis: $vistoks:tt,
            unsafe: $($unsafe:ident)?,
            name: $($name:ident)?,
            gparams: [],
            tpbs: [],
            wc: $wct:tt,
            assoc types: $att:tt,
            assoc consts: $act:tt,
            fns: $fnt:tt,
        }
    ) => {
        $crate::trait_xml_inner! {
            @expand {
                output: [$($out)*],
                vis: $vistoks,
                unsafe: $($unsafe)?,
                name: $($name)?,
                gparams: $gpt,
                tpbs: $rt,
                wc: $wct,
                assoc types: $att,
                assoc consts: $act,
                fns: $fnt,
            }
        }
    };
    // Expand gparams - already present
    (
        @expand {
            output: [[gparams $gpt:tt reqs $rt:tt]$($out:tt)*],
            vis: $vistoks:tt,
            unsafe: $($unsafe:ident)?,
            name: $($name:ident)?,
            gparams: [$($present:tt)+],
            tpbs: $tpbt:tt,
            wc: $wct:tt,
            assoc types: $att:tt,
            assoc consts: $act:tt,
            fns: $fnt:tt,
        }
    ) => {
        compile_error!(concat!("multiple `<bounds>` sections present!"));
    };

    // Expand where clause
    (
        @expand {
            output: [[wc $([$($clausetok:tt)+])*]$($out:tt)*],
            vis: $vistoks:tt,
            unsafe: $($unsafe:ident)?,
            name: $($name:ident)?,
            gparams: $gpt:tt,
            tpbs: $tpbt:tt,
            wc: [],
            assoc types: $att:tt,
            assoc consts: $act:tt,
            fns: $fnt:tt,
        }
    ) => {
        $crate::trait_xml_inner! {
            @expand {
                output: [$($out)*],
                vis: $vistoks,
                unsafe: $($unsafe)?,
                name: $($name)?,
                gparams: $gpt,
                tpbs: $tpbt,
                wc: [$($($clausetok)+,)*],
                assoc types: $att,
                assoc consts: $act,
                fns: $fnt,
            }
        }
    };

    // Move associated constant expansion
    (
        @expand {
            output: [[ac $nametok:tt $typetok:tt $defaulttok:tt]$($out:tt)*],
            vis: $vistoks:tt,
            unsafe: $($unsafe:ident)?,
            name: $($name:ident)?,
            gparams: $gpt:tt,
            tpbs: $tpbt:tt,
            wc: $wct:tt,
            assoc types: $att:tt,
            assoc consts: [$($act:tt)*],
            fns: $fnt:tt,
        }
    ) => {
        $crate::trait_xml_inner! {
            @expand {
                output: [$($out)*],
                vis: $vistoks,
                unsafe: $($unsafe)?,
                name: $($name)?,
                gparams: $gpt,
                tpbs: $tpbt,
                wc: $wct,
                assoc types: $att,
                assoc consts: [$($act)* [$nametok $typetok $defaulttok]],
                fns: $fnt,
            }
        }
    };

    // Move associated type expansion
    (
        @expand {
            output: [[at $($new:tt)+]$($out:tt)*],
            vis: $vistoks:tt,
            unsafe: $($unsafe:ident)?,
            name: $($name:ident)?,
            gparams: $gpt:tt,
            tpbs: $tpbt:tt,
            wc: $wct:tt,
            assoc types: [$($att:tt)*],
            assoc consts: $act:tt,
            fns: $fnt:tt,
        }
    ) => {
        $crate::trait_xml_inner! {
            @expand {
                output: [$($out)*],
                vis: $vistoks,
                unsafe: $($unsafe)?,
                name: $($name)?,
                gparams: $gpt,
                tpbs: $tpbt,
                wc: $wct,
                assoc types: [$($att)* [$($new)+]],
                assoc consts: $act,
                fns: $fnt,
            }
        }
    };

    // Move associated function expansion
    (
        @expand {
            output: [[af $($args:tt)*]$($out:tt)*],
            vis: $vistoks:tt,
            unsafe: $($unsafe:ident)?,
            name: $($name:ident)?,
            gparams: $gpt:tt,
            tpbs: $tpbt:tt,
            wc: $wct:tt,
            assoc types: $att:tt,
            assoc consts: $act:tt,
            fns: [$($fnt:tt)*],
        }
    ) => {
        $crate::trait_xml_inner! {
            @expand {
                output: [$($out)*],
                vis: $vistoks,
                unsafe: $($unsafe)?,
                name: $($name)?,
                gparams: $gpt,
                tpbs: $tpbt,
                wc: $wct,
                assoc types: $att,
                assoc consts: $act,
                fns: [$($fnt)* [af $($args)*]],
            }
        }
    };

    (
        @expand {
            output: [],
            vis: [$($vistok:tt)*],
            unsafe: $($unsafe:ident)?,
            name: ,
            gparams: [$($gpt:tt)*],
            tpbs: [$($tpbt:tt)*],
            wc: [$($wct:tt)*],
            assoc types: $att:tt,
            assoc consts: $act:tt,
            fns: $fnt:tt,
        }
    ) => {
        compile_error!("error expanding trait: no name provided.");
    };

    // Finish trait expansion
    (
        @expand {
            output: [],
            vis: [$($vistok:tt)*],
            unsafe: $($unsafe:ident)?,
            name: $name:ident,
            gparams: [$($gpt:tt)*],
            tpbs: [$($tpbt:tt)*],
            wc: [$($wct:tt)*],
            assoc types: [$($att:tt)*],
            assoc consts: [$($act:tt)*],
            fns: [$($fnt:tt)*],
        }
    ) => {
        $($vistok)* $($unsafe)? trait $name<
            $($gpt)*
        >: $($tpbt)*
        where
            $($wct)*
        {
            $(
                $crate::trait_xml_inner! {
                    @expandat $att
                }
            )*

            $(
                $crate::trait_xml_inner! {
                    @expandac $act
                }
            )*

            $(
                $crate::trait_xml_inner! {
                    @expandfn $fnt
                }
            )*
        }
    };

    (
        @expandac [[$name:ident] [$type:ty] [$($default:expr)?]]
    ) => {
        const $name: $type$( = $default)?;
    };

    (
        @expandat [
            [$name:ident]
            [$($($gpt:tt)+)?]
            [$($($bt:tt)+)?]
            [$($([$($bw:tt)+])+)?]
        ]
    ) => {
        type $name$(<
            $($gpt)+
        >)?$(: $($bt)+)?
        $(where $($($bw)+,)+)?;
    };

    (
        @expandfn [
            af
            [$name:ident]
            [
                unsafe: [$($unsafe:tt)?],
                extern: [$($extern:literal)?],
            ]
            [$($($gpt:tt)+)?]
            [$([param [$arg:tt] [$type:ty]])*]
            [$($rt:ty)?]
            [$($($wt:tt)+)?]
            []
        ]
    ) => {
        $($unsafe)? $(extern $extern)? fn $name$(<$($gpt)+>)?(
            $($arg: $type)*
        )$( -> $rt)?
        $(where $($wt)+)?;
    };
    (
        @expandfn [
            af
            [$name:ident]
            [
                unsafe: [$($unsafe:tt)?],
                extern: [$($extern:literal)?],
            ]
            [$($($gpt:tt)+)?]
            [$([param [$arg:tt] [$type:ty]])*]
            [$($rt:ty)?]
            [$($($wt:tt)+)?]
            [$($dt:tt)*]
        ]
    ) => {
        $($unsafe)? $(extern $extern)? fn $name$(<$($gpt)+>)?(
            $($arg: $type)*
        )$( -> $rt)?
        $(where $($wt)+)?
        {
            $($dt)*
        }
    };
}
