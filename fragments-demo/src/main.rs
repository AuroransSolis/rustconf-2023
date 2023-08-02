macro_rules! me_reaping {
    ($let:tt $lhs:tt $equal:tt $rhs:tt) => {
        // compose `:tt`s into a `:stmt`
        me_reaping!(@matchstmt $let $lhs $equal $rhs)
    };
    (@matchstmt $stmt:stmt) => {
        $stmt
    };
}

macro_rules! me_sowing {
    ($stmt:stmt) => {
        // attempt to break a `:stmt` back into component `:tt`s
        me_reaping!($stmt);
    }
}

fn main() {
    me_reaping!(let haha = "yes!!!");
    println!("{haha}");
    me_sowing!(let well_this = "sucks ):");
    println!("{well_this}");
}
