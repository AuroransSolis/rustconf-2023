Hey, thanks for checking out my RustConf 2023 presentation materials! I really appreciate it!

So, here's the what's what of this repo.

## `all-unique`

This crate contains a crate that demonstrates a macro which will take in a list of comma-separated
expressions and generate a new boolean one determining whether each of the input expressions were
uniquely valued.

The intent is to demonstrate the usefulness of internal rules, however, it does also make use of a
couple incremental TT munchers.

## `fragments-demo`

This crate contains two macros intended to demonstrate the irreversibility of token composition
(excepting TT bundling). The crate should fail to compile until the last two lines of `fn main()`
are commented out.

## `muncher-demo`

This crate demos a very simple TT muncher. Both `cargo expand` and `cargo run` will properly
demonstrate its behaviour - which is, given some list of input token trees, each token tree should
be printed out prefixed with `munched: ` until the list is empty, when `empty!` will be printed.

## `presentation`

This directory contains the LaTeX and SVG files necessary to build the presentation I used at
RustConf 2023 and the presentation I intend to be used as a standalone resource so that interested
parties don't have to go back and pick through the recording of my presentation. A makefile is
included to build these for you. Here's the requirements to build them:

- Software
    - Make
    - LuaLaTeX
    - latexmk
- LaTeX packages
    - beamer
    - emoji
    - fontawesome
    - fontspec
    - framed
    - hyperref
    - iftex
    - minted
    - pdfpc (only required for conference version)
    - svg
    - tabularray
    - tikz
    - ulem
    - xcolor

The PDFs are already included pre-rendered for your convenience.

## `reverse-tokens`

This crate contains a macro demonstrating both an incremental TT muncher and push-down accumulation.
The macro takes some input tokens and reverses their order, and this can be seen through use of
`cargo expand`. It doesn't work with any of `{}()[]`, since I didn't bother to special case them and
just putting them in reverse in the input will result in a compile error. Oops.

## `trait-xml`

The centrepiece of this repository. Give it a trait definition in the form of something that looks a
lot like XML and it'll parse and convert it to an actual trait definition. `cargo expand` will show
what the actual trait definition looks like.

## `tt-bundling`

This crate contains a macro intended to demonstrate the use of TT bundling. It shows how tokens can
be bundled into a single `:tt` with `[]` and then have the operation reversed, as the single
exception to the irreversibility of token composition. It also demonstrates one of the only
debugging tools for macros - `const _: &str = ...` to show that the input tokens were in fact
composed into a single `:tt`.
