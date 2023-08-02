macro_rules! munch_and_crunch {
    () => {
        println!("empty!");
    };
    ($first:tt $($rest:tt)*) => {
        println!(concat!("munched: ", stringify!($first)));
        munch_and_crunch!($($rest)*);
    };
}

fn main() {
    munch_and_crunch!(foo bar baz baq);
    munch_and_crunch!(foo bar [baz baq]);
}
