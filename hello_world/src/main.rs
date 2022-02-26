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

	fn max<T: PartialOrd>(a: T, b: T) -> T  {
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

	let p = Point{ x: 3, y: 2 };
	println!("Point x : {}", p.x());

	let array1 = [1u16, 2, 3, 4];
	println!("array1[3] : {}", array1[3]);
	let array2 = [1u8; 100];
	println!("array2[3] : {}", array2[3]);

	println!("First array1 : {}", first(&array1[1..]));
}
