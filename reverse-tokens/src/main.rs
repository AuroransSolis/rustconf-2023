macro_rules! reverse_tokens {
	(@rev [$first:tt$(, $rest:tt)*] [$($rev:tt),*]) => {
		reverse_tokens! {
			@rev [$($rest),*][$first $(, $rev)*]
		}
	};
	(@rev [] [$($rev:tt),*]) => {
		$($rev)*
	};
	($($tt:tt)+) => {
		reverse_tokens! {
			@rev [$($tt),+] []
		}
	};
}

fn main() {
    reverse_tokens! {
        ;0 = foo let
    }
    println!("{foo}");
}
