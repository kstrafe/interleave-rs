#[macro_use(interleave)]
extern crate interleave;

use interleave::{IterList, MultiIter};

fn main() {
	let iter = interleave!(0..2);
	for i in iter {
		println!("{:?}", i);
	}
}
