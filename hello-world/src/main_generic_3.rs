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

use std::fmt::*;

struct MinMax<T> {
	min: T,
	max: T,
}

impl<T> Display for MinMax<T>
	where T: Display {
	
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		write!(f, "Min {}, Max {}", self.min, self.max)
	}
}

struct DisplayOption<T>(Option<T>);

impl<T> Display for DisplayOption<T>
	where T: Display {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		match self.0 {
			Some(ref t) => write!(f, "Some : {}", t),
			None => write!(f, "None"),
		}
	}
}

fn min_max<T>(slice: &[T]) -> DisplayOption<MinMax<T>>
	where T: PartialOrd + Clone {
	if slice.is_empty() {
		return DisplayOption(None);
	}
	
	let mut min = &slice[0];
	let mut max = &slice[0];
	
	for i in slice.iter() {
		if min > i {
			min = i;
		} else {
			if max < i {
				max = i;
			}
		}
	}
	
	DisplayOption(Some(MinMax{min: min.clone(), max: max.clone()}))
}

fn main() {
	
	let slice = ['1', 'z', '3', '1', '8', '4', 'a', '6'];
	
	println!("{}", min_max(&slice));
}
