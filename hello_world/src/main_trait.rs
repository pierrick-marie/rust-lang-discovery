use std::fmt::{Debug, Formatter};

struct Point {
	x: i32,
	y: i32,
}

/*
    Test implement Debug
    Formatter ?
    Result ?
    Too early for my skills about rust
*/
impl Debug for Point {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		println!("Debug ?!?");
		todo!()
	}
}

trait BitSet {
	fn clear(&mut self, index: usize);
	fn is_set(&self, index: usize) -> bool;
	fn set(&mut self, index: usize);
}

impl BitSet for u64 {
	fn clear(&mut self, index: usize) {

		*self &= !(1 << index);
	}

	fn is_set(&self, index: usize) -> bool {

		( (*self >> index) & 1 ) == 1
	}

	fn set(&mut self, index: usize) {

		*self |= 1 << index;
	}
}

fn main() {

	let mut num = 0;
	num.set(3);
	println!("num = {}", num);
	println!("num is set ? {}", num.is_set(3));
	println!("num is set ? {}", num.is_set(5));

	num.set(5);
	println!("num = {}", num);
	println!("num is set ? {}", num.is_set(3));
	println!("num is set ? {}", num.is_set(5));

	num.clear(5);
	println!("num = {}", num);
	println!("num is set ? {}", num.is_set(3));
	println!("num is set ? {}", num.is_set(5));

	let p = Point {
		x: 1,
		y: 2,
	};

	println!("p = {:#?}", p);
}
