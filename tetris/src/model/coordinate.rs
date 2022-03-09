use std::fmt::{Display, Formatter, Result};
use std::ops::{Range, RangeInclusive};

#[derive(Debug, Copy, Clone, std::hash::Hash)]
pub struct Coordinate {
	pub x: u32,
	pub y: u32,
}

impl std::ops::Add for Coordinate {
	type Output = Coordinate;
	
	fn add(self, coordinate: Self) -> Self::Output {
		Coordinate {
			x: self.x + coordinate.x,
			y: self.y + coordinate.y,
		}
	}
}

impl Display for Coordinate {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		
		write!(f, "({}, {})", self.x, self.y)
	}
}

impl Eq for Coordinate {}

impl PartialEq for Coordinate {
	fn eq(&self, other: &Self) -> bool {
		self.x == other.x && self.y == other.y
	}
	
	fn ne(&self, other: &Self) -> bool {
		self.x != other.x || self.y != other.y
	}
}