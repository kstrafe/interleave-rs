#[macro_use(interleave)]
extern crate interleave;

use interleave::{IterList, MultiIter};

fn main() {
	let iter = interleave!(i32; (0..2));
	for i in iter {
		println!("{:?}", i);
	}
}
