So let's take a look at the macro shown in `src/main.rs` a bit more closely and work out how it
expands a bit. For reference, said macro is:
```rs
macro_rules! all_unique {
    ($($vars:expr),+$(,)?) => {
        all_unique!(@genexpr $($vars),+)
    };
    (@genexpr $first:expr, $second:expr$(, $rest:expr)+) => {
        all_unique!(@andconcat $first != $second$(, $first != $rest)+)
            && all_unique!(@genexpr $second$(, $rest)+)
    };
    (@genexpr $first:expr, $second:expr) => {
        $first != $second
    };
    (@andconcat $first:expr$(, $rest:expr)+) => {
        $first && all_unique!(@andconcat $($rest),+)
    };
    (@andconcat $last:expr) => {
        $last
    };
}
```
With a call like `all_unique!(a, b, c, d)`. Let's walk through the expansion steps.

First, we hit the top branch, `$($vars:expr),+$(,)?`. This then expands to
```rs
all_unique!(@genexpr a, b, c, d)
```
That then matches on the second branch, `@genexpr $first:expr, $second:expr$(, $rest:expr)+`,
which expands to
```rs
all_unique!(@andconcat a != b, a != c, a != d)
    && all_unique!(@genexpr b, c, d)
```
Let's focus on the expansion of the `@andconcat` branches first. In order, separated by double
newlines:
```rs
a != b
    && all_unique!(@andconcat a != c, a != d)
    && all_unique!(@genexpr b, c, d)

a != b
    && (a != c && all_unique!(@andconcat a != d))
    && all_unique!(@genexpr b, c, d)

a != b
    && (a != c && a != d)
    && all_unique!(@genexpr b, c, d)
```
And then we can again expand the `@genexpr` branch. Since we know how both branches look on
expansion now, I'm going to do one step of expansion at a time as the compiler would do it.
```rs
a != b
    && (a != c && a != d)
    && all_unique!(@genexpr b, c, d)

a != b
    && (a != c && a != d)
    && (
        all_unique!(@andconcat b != c, b != d)
            && all_unique!(@genexpr c, d)
    )

a != b
    && (a != c && a != d)
    && (
        b != c && b != d
            && c != d
    )

// Just for the sake of nicer formatting
a != b && (a != c && a != d) && (b != c && b != d && c != d)
```
This exactly matches the output from `cargo expand`.