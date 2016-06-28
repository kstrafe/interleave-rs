#[allow(dead_code)]

struct MultiIter<T> {
	empty: bool,
	index: usize,
	items: Vec<Box<Iterator<Item = T>>>,
}

impl<T> Iterator for MultiIter<T> {
	type Item = T;
	fn next(&mut self) -> Option<Self::Item> {
		loop {
			if let Some(iterator) = self.items.get_mut(self.index) {
				if let Some(value) = iterator.next() {
					self.empty = false;
					self.index += 1;
					return Some(value);
				} else {
					self.index += 1;
				}
			} else {
				self.index = 0;
				if self.empty {
					return None;
				} else {
					self.empty = true;
				}
			}
		}
	}
}

macro_rules! interleave {
	($t:ty; $($e:expr),*,) => ( interleave!($t; $($e),*) );
	($t:ty; $($e:expr),*) => ({
		let mut temporary: Vec<Box<Iterator<Item = $t>>> = vec![];
		$(
			temporary.push(Box::new($e));
		)*
		MultiIter {
			empty: true,
			index: 0,
			items: temporary,
		}
	});
}

#[cfg(test)]
mod tests {
	use super::MultiIter;

	macro_rules! next {
		($e:expr; $($n:expr),*,) => ( next!($e; $($n),*) );
		($e:expr; $($n:expr),*) => ( $(assert_eq!($e.next(), Some($n)));* );
	}

	#[test]
	fn simple() {
		let mut iter = interleave!(i32; (0..10));
		assert_eq!(iter.next(), Some(0));
		assert_eq!(iter.next(), Some(1));
	}

	#[test]
	fn di_iter() {
		let mut iter = interleave!(i32; (0..10), (5..15));
		assert_eq!(iter.next(), Some(0));
		assert_eq!(iter.next(), Some(5));
		assert_eq!(iter.next(), Some(1));
		assert_eq!(iter.next(), Some(6));
	}

	#[test]
	fn tri_iter() {
		let mut iter = interleave!(i32; (0..), (0..), (0..));
		next!(iter; 0, 0, 0, 1, 1, 1, 2, 2, 2);
	}

	#[test]
	fn quad_iter() {
		let mut iter = interleave!{(i32, i32);
			(0..3).map(|x| (0, x)),
			(0..3).map(|x| (x, 0)),
		};
		next!(iter;
			(0, 0),
			(0, 0),
			(0, 1),
			(1, 0),
			(0, 2),
			(2, 0),
		);
	}

	#[test]
	fn diff_len() {
		fn check(mut iter: MultiIter<i32>) {
			next!(iter; 0,0,0,0, 1,1,1,1, 2,2,2, 3,3,3, 4,4,4, 5,5, 6,6, 7, 8, 9);
		}
		check(interleave!(i32; (0..10), (0..5), (0..2), (0..7)));
		check(interleave!(i32; (0..5), (0..2), (0..7), (0..10)));
		check(interleave!(i32; (0..5), (0..7), (0..2), (0..10)));
	}
}
