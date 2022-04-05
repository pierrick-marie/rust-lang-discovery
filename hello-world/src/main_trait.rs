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

use std::fmt::{Debug, Formatter, Result};
use std::ops::{Add, Mul, Sub};

struct Point {
	x: i32,
	y: i32,
}

struct Vector {
	p1: Point,
	p2: Point,
}

impl Add<Point> for Point {
	type Output = Point;

	fn add(self, point: Point) -> Self::Output {
		Point {
			x: self.x + point.y,
			y: self.y + point.y,
		}
	}
}

impl Sub<Point> for Point {
	type Output = Point;

	fn sub(self, point: Point) -> Self::Output {
		Point {
			x: self.x - point.x,
			y: self.y - point.y,
		}
	}
}

impl Mul<Vector> for Vector {
	type Output = i32;

	fn mul(self, vector: Vector) -> Self::Output {
		let v1 = self.p1 - self.p2;
		let v2 = vector.p1 - vector.p2;
		v1.x * v2.x + v1.y * v2.y
	}
}

impl Debug for Point {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		write!(f, "Debug: ({}, {})", self.x, self.y)
	}
}

trait BitSet {
	fn clear(&mut self, index: usize);
	fn is_set(&self, index: usize) -> bool;
	fn set(&mut self, index: usize);

	fn toggle(&mut self, index: usize) {
		if self.is_set(index) {
			self.clear(index);
		} else {
			self.set(index);
		}
	}
}

impl BitSet for u64 {
	fn clear(&mut self, index: usize) {
		*self &= !(1 << index);
	}

	fn is_set(&self, index: usize) -> bool {
		((*self >> index) & 1) == 1
	}

	fn set(&mut self, index: usize) {
		*self |= 1 << index;
	}

	fn toggle(&mut self, index: usize) {
		*self ^= 1 << index;
	}
}

fn main() {

	let mut num = 0;
	println!("Set 3");
	num.set(3);
	println!("num = {}", num);
	println!("num is set ? {}", num.is_set(3));
	println!("num is set ? {}", num.is_set(5));

	println!("Set 5");
	num.set(5);
	println!("num = {}", num);
	println!("num is set ? {}", num.is_set(3));
	println!("num is set ? {}", num.is_set(5));

	println!("Clear 5");
	num.clear(5);
	println!("num = {}", num);
	println!("num is set ? {}", num.is_set(3));
	println!("num is set ? {}", num.is_set(5));

	println!("Toggle 3");
	num.toggle(3);
	println!("num = {}", num);
	println!("num is set ? {}", num.is_set(3));

	println!("Toggle 3");
	num.toggle(3);
	println!("num = {}", num);
	println!("num is set ? {}", num.is_set(3));

	println!("\n ====================== \n");

	let p1 = Point { x: 2, y: 0, };
	let p2 = Point { x: 4, y: 0, };
	let v1 = Vector{ p1, p2 };

	let p3 = Point { x: 2, y: 0, };
	let p4 = Point { x: 0, y: 4, };
	let v2 = Vector{ p1: p3, p2: p4 };

	let scalar = v1 * v2;
	println!("Res scalar = {}", scalar);
}
