#[macro_use(interleave)]
extern crate interleave;

use interleave::{IterList, MultiIter};

fn main() {
	let iter = interleave!(i32; (-3..3), (0..6));
	for i in iter {
		println!("{:?}", i);
	}
}
