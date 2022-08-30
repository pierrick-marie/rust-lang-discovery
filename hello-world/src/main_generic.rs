/* Copyright 2022 Pierrick MARIE

This file is part of rust-discovery

LCS is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

Rust-discovery is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with rust-discovery.  If not, see <http://www.gnu.org/licenses/>. */

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
