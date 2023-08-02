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

fn main() {
    let [a, b, c, d] = [0, 1, 2, 3];
    if all_unique!(a, b, c, d) {
        println!("a, b, c, d all unique!");
    } else {
        println!("a, b, c, d not all unique!");
    }
    let [a, b, c, d, e, f, g, h, i, j] = [0, 0, 1, 2, 3, 4, 5, 6, 7, 8];
    if all_unique!(a, b, c, d, e, f, g, h, i, j) {
        println!("a..=j all unique!");
    } else {
        println!("a..=j not all unique!");
    }
}
