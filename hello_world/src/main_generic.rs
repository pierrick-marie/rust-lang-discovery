use num::cast::ToPrimitive;

struct Point<T: ToPrimitive> {
	x: T,
	y: T,
}

impl<T: ToPrimitive> Point<T> {
	fn x(&self) -> &T {
		println!("Call function x()");
		&self.x
	}
}

fn main() {
	fn max<T: PartialOrd>(a: T, b: T) -> T {
		if a > b {
			a
		} else {
			b
		}
	}
	
	fn first<T>(slice: &[T]) -> &T {
		&slice[0]
	}
	
	println!("Max 2 vs 5 : {}", max('2', '5'));
	println!("Max a vs z : {}", max('a', 'z'));
	
	println!("\n ====================== \n");
	
	let p = Point { x: 3, y: 2 };
	println!("Point x : {}", p.x());
}
