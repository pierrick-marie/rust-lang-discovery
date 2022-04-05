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

use std::fmt;

fn index_of<T>(array: &[T], target: &T) -> Option<usize>
	where T: PartialEq {
	for (index, element) in array.iter().enumerate() {
		if element == target {
			return Some(index);
		}
	}
	return None;
}

fn print_option<T>(option: Option<T>)
	where T: fmt::Display {
	match option {
		Some(T) => println!("It's T : {}", T),
		None => println!("None")
	}
}

fn main() {
	
	fn max<T>(a: T, b: T) -> T
		where T: PartialOrd {
		if a > b {
			a
		} else {
			b
		}
	}
	
	fn first<T>(slice: &[T]) -> &T {
		
		&slice[0]
	}
	
	fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
		if x.len() > y.len() {
			x
		} else {
			y
		}
	}
	
	println!("Max 2 vs 5 : {}", max('2', '5'));
	println!("Max a vs z : {}", max('a', 'z'));
	
	println!("\n ====================== \n");
	
	let array1 = [1u16, 2, 3, 4];
	println!("array1[3] : {}", array1[3]);
	let array2 = [1u8; 100];
	println!("array2[3] : {}", array2[3]);
	
	println!("First array1 : {}", first(&array1[1..]));
	
	let mut sum = 0;
	for it in &array1 {
		sum += *it;
	}
	println!("sum = {}", sum);
	
	let result = longest("string1.as_str()", "string2");
	println!("The longest string is {}", result);
	
	print_option(index_of(&array1, &3u16));
	print_option(index_of(&array1, &9u16));
}
