macro_rules! bundle_and_unbundle {
	($name:ident, $type:ty, $value:expr) => {
		bundle_and_unbundle! {
			@bundled [$name, $type, $value]
		}
	};
	(@bundled $bundle:tt) => {
		const _: &str = stringify!($bundle);
		bundle_and_unbundle! {
			@unbundle $bundle
		}
	};
	(@unbundle [$name:ident, $type:ty, $value:expr]) => {
		let $name: $type = $value;
	};
}

fn main() {
	bundle_and_unbundle! {
		foo, u8, 0
	}
}
