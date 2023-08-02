// #![recursion_limit = "256"]

//! I've decided that it's time to advance the art of defining Rust traits. Thus, I introduce to
//! you: `trait_xml!`.
//!
//! This is a declarative macro that takes in an XML-like format and turns it into an actual trait
//! definition.
//!
//! # Usage
//!
//! Only one trait definition per macro invocation is allowed, which must start with `<trait>` and
//! end with `</trait>`, e.g.:
//! ```text
//! <trait>
//!     ...
//! </trait>
//! ```
//! Any leading or trailing tokens will produce a compile error.
//!
//! Only one tag in the middle is necessary: `<name>Name</name>`:
//! ```
//! trait_xml::trait_xml! {
//!     <trait>
//!         <name>Foo</name>
//!     </trait>
//! }
//! ```
//! is a valid trait definition, which will expand to
//! ```
//! trait Foo {}
//! ```
//! Traits can also have the following:
//!   - `<vis></vis>`
//!   - `<unsafe/>`
//!   - `<bounds></bounds>`
//!   - `<where></where>`
//!   - `<assoctype></assoctype>`
//!   - `<assocconst></assocconst>`
//!   - `<assocfn></assocfn>`
//!
//! Let's talk about what tags are valid within each of those contexts.
//!
//! ## Visibility
//!
//! This is done with the `<vis></vis>` tag set. Between the two tags there must be a valid
//! visibility specifier, e.g. `pub(crate)`. Usage looks something like
//! ```
//! trait_xml::trait_xml! {
//!     <trait>
//!         <name>Foo</name>
//!         <vis>pub</vis>
//!     </trait>
//! }
//! ```
//! This expands to
//! ```
//! pub trait Foo {}
//! ```
//!
//! ## Unsafe
//!
//! This is a single-item tag - just stick `<unsafe/>` somewhere in the trait definition, and the
//! trait will be marked as `unsafe`. For example:
//! ```
//! trait_xml::trait_xml! {
//!     <trait>
//!         <name>Foo</name>
//!         <vis>pub</vis>
//!         <unsafe/>
//!     </trait>
//! }
//! ```
//! This expands to
//! ```
//! pub unsafe trait Foo {}
//! ```
//!
//! ## Bounds
//!
//! For purposes of this macro, "bounds" refers to lifetimes, generic types, const generics, and
//! supertraits. Each of these has a specific tag set:
//!
//! - Lifetimes: `<lifetime></lifetime>`
//! - Generic types: `<type></type>`
//! - Const generics: `<const></const>`
//! - Supertraits: `<req></req>`
//!
//! Here's the requirements for each of them.
//!
//! #### Lifetimes
//!
//! Each lifetime requires:
//!
//! - Exactly one `<name></name>`, containing a valid lifetime name
//! - Zero or more `<lifetime-bound></lifetime-bound>`s, containing a valid lifetime name
//!
//! For example:
//! ```
//! trait_xml::trait_xml! {
//!     <trait>
//!         <name>Foo</name>
//!         <bounds>
//!             <lifetime>
//!                 <name>'bar</name>
//!                 <lifetime-bound>'baz</lifetime-bound>
//!             </lifetime>
//!             <lifetime>
//!                 <name>'baz</name>
//!             </lifetime>
//!         </bounds>
//!     </trait>
//! }
//! ```
//! This expands to
//! ```
//! trait Foo<'bar: 'baz, 'baz> {}
//! ```
//!
//! #### Generic types
//!
//! Each generic type requires:
//!
//! - Exactly one `<name></name>`, containing a valid generic type name (identifier)
//! - Zero or more of `<type-bound></type-bound>`, `<lifetime-bound></lifetime-bound>`, or
//! `<for-bound></for-bound>`.
//!
//! For example:
//! ```
//! trait_xml::trait_xml! {
//!     <trait>
//!         <name>Foo</name>
//!         <bounds>
//!             <lifetime>
//!                 <name>'bar</name>
//!             </lifetime>
//!             <type>
//!                 <name>Baz</name>
//!                 <type-bound>Clone</type-bound>
//!                 <lifetime-bound>'bar</lifetime-bound>
//!                 <for-bound>
//!                     <lifetime>'baq</lifetime>
//!                     <type-bound>std::ops::Fn(&'baq u8)</type-bound>
//!                 </for-bound>
//!             </type>
//!         </bounds>
//!     </trait>
//! }
//! ```
//! This expands to
//! ```
//! trait Foo<'bar, Baz: Clone + 'bar + for<'baq> std::ops::Fn(&'baq u8)> {}
//! ```
//!
//! #### Const generics
//!
//! Each const generic requires:
//!
//! - Exactly one `<name></name>`, containing a valid const generic name (identifier)
//! - Exactly one `<type></type>`, containing a valid type for a const generic
//!
//! For example:
//! ```
//! trait_xml::trait_xml! {
//!     <trait>
//!         <name>Foo</name>
//!         <bounds>
//!             <const>
//!                 <name>BAR</name>
//!                 <type>usize</type>
//!             </const>
//!         </bounds>
//!     </trait>
//! }
//! ```
//! This expands to
//! ```
//! trait Foo<const BAR: usize> {}
//! ```
//!
//! #### Supertraits
//!
//! Denoted by `<req></req>` and requires a valid supertrait path (`:path`).
//!
//! For example:
//! ```
//! trait_xml::trait_xml! {
//!     <trait>
//!         <name>Foo</name>
//!     </trait>
//! }
//!
//! trait_xml::trait_xml! {
//!     <trait>
//!         <name>Bar</name>
//!         <bounds>
//!             <req>Foo</req>
//!         </bounds>
//!     </trait>
//! }
//! ```
//! This expands to
//! ```
//! trait Foo {}
//!
//! trait Bar: Foo {}
//! ```
//!
//! ## \*-bound Tags
//!
//! #### Type Bounds
//!
//! Denoted by `<type-bound></type-bound>`, this represents a trait bound on a type. Between
//! `<type-bound></type-bound>` there must be a valid trait path (`:path`). For example:
//! ```
//! trait_xml::trait_xml! {
//!     <trait>
//!         <name>Foo</name>
//!         <bounds>
//!             <type>
//!                 <name>Bar</name>
//!                 <type-bound>Clone</type-bound>
//!                 <type-bound>Iterator<Item = usize></type-bound>
//!             </type>
//!         </bounds>
//!     </trait>
//! }
//! ```
//! This expands to
//! ```
//! trait Foo<Bar: Clone + Iterator<Item = usize>> {}
//! ```
//!
//! #### Lifetime Bounds
//!
//! Denoted by `<lifetime-bound></lifetime-bound>`, this represents a lifetime bound on a type or
//! lifetime. Between `<lifetime-bound></lifetime-bound>`, there must be a valid lifetime
//! (`:lifetime`). For example:
//! ```
//! trait_xml::trait_xml! {
//!     <trait>
//!         <name>Foo</name>
//!         <bounds>
//!             <lifetime>
//!                 <name>'bar</name>
//!             </lifetime>
//!             <type>
//!                 <name>Baz</name>
//!                 <lifetime-bound>'bar</lifetime-bound>
//!             </type>
//!         </bounds>
//!     </trait>
//! }
//! ```
//! This expands to
//! ```
//! trait Foo<'bar, Baz: 'bar> {}
//! ```
//!
//! #### `for` Bounds
//!
//! Denoted by `<for-bound></for-bound>`, this represents a `for` bound on a type. Each `for` bound
//! requires:
//!
//! - One or more `<lifetime></lifetime>`s containing valid lifetimes
//! - Exactly one `<type-bound></type-bound>`
//!
//! For example:
//! ```
//! trait_xml::trait_xml! {
//!     <trait>
//!         <name>Foo</name>
//!         <bounds>
//!             <type>
//!                 <name>Bar</name>
//!                 <for-bound>
//!                     <lifetime>'baz</lifetime>
//!                     <type-bound>std::ops::Fn(&'baz u8)</type-bound>
//!                 </for-bound>
//!             </type>
//!         </bounds>
//!     </trait>
//! }
//! ```
//! This expands to
//! ```
//! trait Foo<Bar: for<'baz> std::ops::Fn(&'baz u8)> {}
//! ```
//!
//! ## `where` clauses
//!
//! `where` clauses can contain any number of `<type-clause></type-clause>`,
//! `<lifetime-clause></lifetime-clause>`, or `<for-clause></for-clause>`. See the subsections for
//! each of these to learn what is required and/or allowed in each of them.
//!
//! #### Type clauses
//!
//! Type clauses are denoted by `<type-clause></type-clause>` and require:
//!
//! - Exactly one `<type></type>` containing a valid type that is bound by...
//! - Zero or more `<type-bound></type-bound>`, `<lifetime-bound></lifetime-bound>`, or
//! `<for-bound></for-bound>`
//!
//! For example:
//! ```
//! trait_xml::trait_xml! {
//!     <trait>
//!         <name>Foo</name>
//!         <bounds>
//!             <req>Iterator</req>
//!         </bounds>
//!         <where>
//!             <type-clause>
//!                 <type><Self as Iterator>::Item</type>
//!                 <type-bound>Iterator</type-bound>
//!             </type-clause>
//!         </where>
//!     </trait>
//! }
//! ```
//! This expands to
//! ```
//! trait Foo: Iterator
//! where
//!     <Self as Iterator>::Item: Iterator,
//! {}
//! ```
//!
//! #### Lifetime clauses
//!
//! Lifetime clauses are denoted by `<lifetime-clause></lifetime-clause>` and require:
//!
//! - Exactly one `<lifetime></lifetime>` containing a valid lifetime that is bound by...
//! - Zero or more `<lifetime-bound></lifetime-bound>`
//!
//! For example:
//! ```
//! trait_xml::trait_xml! {
//!     <trait>
//!         <name>Foo</name>
//!         <bounds>
//!             <lifetime>
//!                 <name>'bar</name>
//!             </lifetime>
//!             <lifetime>
//!                 <name>'baz</name>
//!             </lifetime>
//!         </bounds>
//!         <where>
//!             <lifetime-clause>
//!                 <lifetime>'bar</lifetime>
//!                 <lifetime-bound>'baz</lifetime-bound>
//!             </lifetime-clause>
//!         </where>
//!     </trait>
//! }
//! ```
//! This expands to
//! ```
//! trait Foo<'bar, 'baz>
//! where
//!     'bar: 'baz,
//! {}
//! ```
//!
//! #### `for` clauses
//!
//! `for` clauses are denoted by `<for-clause></for-clause>` and require:
//!
//! - One or more `<lifetime></lifetime>` containing valid lifetimes
//! - Exactly one `<type-clause></type-clause>`
//!
//! For example:
//! ```
//! trait_xml::trait_xml! {
//!     <trait>
//!         <name>Foo</name>
//!         <bounds>
//!             <type>
//!                 <name>Bar</name>
//!             </type>
//!         </bounds>
//!         <where>
//!             <for-clause>
//!                 <lifetime>'baz</lifetime>
//!                 <type-clause>
//!                     <type>Bar</type>
//!                     <type-bound>std::ops::Fn(&'baz u8)</type-bound>
//!                 </type-clause>
//!             </for-clause>
//!         </where>
//!     </trait>
//! }
//! ```
//! This expands to
//! ```
//! trait Foo<Bar>
//! where
//!     for<'baz> Bar: std::ops::Fn(&'baz u8),
//! {}
//! ```
//!
//! ## Associated types
//!
//! Associated types are denoted by `<assoctype></assoctype>` and require:
//!
//! - Exactly one `<name></name>` with a valid identifier
//! - Zero or one `<bounds></bounds>`
//! - Zero or one `<where></where>`
//!
//! For example:
//! ```
//! trait_xml::trait_xml! {
//!     <trait>
//!         <name>Foo</name>
//!     </trait>
//! }
//!
//! trait_xml::trait_xml! {
//!     <trait>
//!         <name>Bar</name>
//!         <assoctype>
//!             <name>Baz</name>
//!             <bounds>
//!                 <req>Foo</req>
//!                 <type>
//!                     <name>Baq</name>
//!                 </type>
//!             </bounds>
//!             <where>
//!                 <type-clause>
//!                     <type>Self::Baz<Baq></type>
//!                     <type-bound>Clone</type-bound>
//!                 </type-clause>
//!             </where>
//!         </assoctype>
//!     </trait>
//! }
//! ```
//! This expands to
//! ```
//! trait Foo {}
//!
//! trait Bar<Baz> {
//!     type Baz<Baq>: Foo
//!     where
//!         Self::Baz<Baq>: Clone;
//! }
//! ```
//!
//! ## Associated constants
//!
//! Associated types are denoted by `<assocconst></assocconst>` and require:
//!
//! - Exactly one `<name></name>` with a valid identifier
//! - Exactly one `<type></type>` with a valid type
//! - Zero or one `<default-value></default-value>` with a valid expression
//!
//! Yes, I know that it should technically be a block for default values, but I've spent way too
//! long on this project as-is and I don't feel like fixing it.
//!
//! For example:
//! ```
//! trait_xml::trait_xml! {
//!     <trait>
//!         <name>Foo</name>
//!         <assocconst>
//!             <name>BAR</name>
//!             <type>u8</type>
//!             <default-value>0</default-value>
//!         </assocconst>
//!     </trait>
//! }
//! ```
//! This expands to
//! ```
//! trait Foo {
//!     const BAR: u8 = 0;
//! }
//! ```
//!
//! ## Assocaited functions
//!
//! Associated functions are denoted by `<assocfn></assocfn>` and require:
//!
//! - Exactly one `<name></name>` with a valid identifier
//! - Zero or one `<unsafe/>`
//! - Zero or one `<extern></extern>` with a string literal argument denoting an ABI
//! - Zero or one `<gparams></gparams>`
//! - Zero or one `<params></params>`
//! - Zero or one `<ret></ret>`
//! - Zero or one `<where></where>`
//! - Zero or one `<rust></rust>`
//!
//! For example:
//! ```
//! trait_xml::trait_xml! {
//!     <trait>
//!         <name>Foo</name>
//!         <assocfn>
//!             <name>bar</name>
//!         </assocfn>
//!     </trait>
//! }
//! ```
//!
//! #### Specifying an ABI
//!
//! Associated function sections may contain zero or one `<extern>"ABI"</extern>`, where `ABI` is
//! the name of an ABI as discussed in [the language reference](https://doc.rust-lang.org/reference/items/functions.html#extern-function-qualifier).
//!
//! #### Associated function generic parameters
//!
//! Associated function generic parameters are denoted by `<gparams></gparams>`. These are nearly
//! identical to [`<bounds>`](#bounds) except `<req></req>` tags are not accepted.
//!
//! For example:
//! ```
//! trait_xml::trait_xml! {
//!     <trait>
//!         <name>Foo</name>
//!         <assocfn>
//!             <name>bar</name>
//!             <gparams>
//!                 <const>
//!                     <name>BAZ</name>
//!                     <type>u8</type>
//!                 </const>
//!             </gparams>
//!         </assocfn>
//!     </trait>
//! }
//! ```
//! This expands to
//! ```
//! trait Foo {
//!     fn bar<const BAZ: u8>();
//! }
//! ```
//!
//! #### Associated function parameters
//!
//! Associated function parameters are denoted by `<params></params>`. This is a section where the
//! only allowed tag is `<param></param>`. Within the one allowed `<params>` section there must be
//! zero or more `<param>` tags, which require:
//!
//! - Exactly one of `<pat></pat>` or `<name></name>`
//!   - `<pat></pat>` requires a valid pattern (`:pat`) and can only be used if a function body is
//!     provided
//!   - `<name></name>` requires a valid identifier
//! - Exactly one `<type></type>`
//!
//! The shorthands `self`, `&self`, `&mut self`, `&'lifetime self`, and `&'lifetime mut self` are
//! not allowed and must be expressed using their expanded forms with `Self`.
//!
//! For example:
//! ```
//! trait_xml::trait_xml! {
//!     <trait>
//!         <name>Foo</name>
//!         <assocfn>
//!             <name>bar</name>
//!             <params>
//!                 <param>
//!                     <name>self</name>
//!                     <type>Self</type>
//!                 </param>
//!             </params>
//!         </assocfn>
//!     </trait>
//! }
//! ```
//! This expands to
//! ```
//! trait Foo {
//!     fn bar(self: Self);
//! }
//! ```
//! 
//! #### Associated function return type
//! 
//! Associated function return types are denoted by `<ret></ret>`, which requires a valid type
//! between the tags.
//! 
//! For example:
//! ```
//! trait_xml::trait_xml! {
//!     <trait>
//!         <name>Foo</name>
//!         <assocfn>
//!             <name>bar</name>
//!             <ret>Self</ret>
//!         </assocfn>
//!     </trait>
//! }
//! ```
//! This expands to
//! ```
//! trait Foo {
//!     fn bar() -> Self;
//! }
//! ```
//! 
//! #### Associated function `where` clause
//! 
//! Works exactly the same as other `where` clauses.
//! 
//! #### Associated function default definition
//! 
//! Associated function default definitions are provided between `<rust></rust>` tags. Everything
//! between these two is directly put into the function definition because I couldn't be bothered to
//! reimplement the entirety of Rust in XML. I'm not that masochistic.
//! 
//! For example:
//! ```
//! trait_xml::trait_xml! {
//!     <trait>
//!         <name>NextOr0u8</name>
//!         <bounds>
//!             <req>Iterator<Item = u8></req>
//!         </bounds>
//!         <assocfn>
//!             <name>next_or_0u8</name>
//!             <params>
//!                 <param>
//!                     <name>self</name>
//!                     <type>&mut Self</type>
//!                 </param>
//!             </params>
//!             <ret>u8</ret>
//!             <rust>
//!                 self.next().unwrap_or(0)
//!             </rust>
//!         </assocfn>
//!     </trait>
//! }
//! ```
//!
//! ## Meta items??????
//!
//! LMFAO
//!
//! NO
//!
//! YOURE WRITING TRAITS WITH XML BOZO
//!
//! (i didnt feel like doing it)
//!
//! # Examples
//!
//! For some examples, let's rewrite some `std` traits in XML. [`Index`](std::ops::Index) could be
//! written as the following using the syntax described by the above:
//! ```
//! trait_xml::trait_xml! {
//!     <trait>
//!         <name>IndexButXml</name>
//!         <vis>pub</vis>
//!         <bounds>
//!             <type>
//!                 <name>Idx</name>
//!                 <type-bound>?Sized</type-bound>
//!             </type>
//!         </bounds>
//!         <assoctype>
//!             <name>Output</name>
//!             <bounds>
//!                 <req>?Sized</req>
//!             </bounds>
//!         </assoctype>
//!         <assocfn>
//!             <name>index</name>
//!             <params>
//!                 <param>
//!                     <name>self</name>
//!                     <type>&Self</type>
//!                 </param>
//!             </params>
//!         </assocfn>
//!     </trait>
//! }
//! ```
//! [`AsRef`](std::convert::AsRef) could be written as
//! ```
//! trait_xml::trait_xml! {
//!     <trait>
//!         <name>AsRefButXml</name>
//!         <bounds>
//!             <type>
//!                 <name>T</name>
//!                 <type-bound>?Sized</type-bound>
//!             </type>
//!         </bounds>
//!         <assocfn>
//!             <name>as_ref</name>
//!             <params>
//!                 <param>
//!                     <name>self</name>
//!                     <type>&Self</type>
//!                 </param>
//!             </params>
//!             <ret>&T</ret>
//!         </assocfn>
//!     </trait>
//! }
//! ```
//! And so on and so forth.

mod assoc_const;
mod assoc_fn;
mod assoc_type;
mod bounds;
mod const_generics;
mod for_bound;
mod for_clause;
mod gparams;
mod lifetime;
mod lifetime_bound;
mod lifetime_clause;
mod name_ident;
mod supertrait;
mod trait_xml_macro;
mod r#type;
mod type_bound;
mod type_clause;
mod type_ty;
mod vis;
mod r#where;

#[test]
fn test() {
    let t = trybuild::TestCases::new();
    t.pass("tests/pass/*.rs");
    t.compile_fail("tests/fail/*.rs");
}

trait_xml! {
    <trait>
        <name>Foo</name>
        <vis>pub</vis>
        <bounds>
            <const>
                <name>BAR</name>
                <type>usize</type>
            </const>
            <req>Baz</req>
        </bounds>
        <assoctype>
            <name>Baq</name>
            <bounds>
                <req>Qux</req>
            </bounds>
        </assoctype>
        <assocconst>
		<name>QUUX</name>
		<type>Self::Baq</type>
        </assocconst>
        <assocfn>
            <name>corge</name>
            <gparams>
                <type>
                    <name>Grault</name>
                </type>
                <type>
                    <name>Garply</name>
                </type>
            </gparams>
            <params>
                <param>
                    <name>waldo</name>
                    <type>Grault</type>
                </param>
            </params>
            <ret>Garply</ret>
        </assocfn>
    </trait>
}

pub trait Baz {}

pub trait Qux {}
