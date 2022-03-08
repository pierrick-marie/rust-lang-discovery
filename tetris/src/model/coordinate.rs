use std::fmt::{Display, Formatter, Result};
use std::ops::{Range, RangeInclusive};

#[derive(Debug, Copy, Clone)]
pub struct Coordinate {
	pub x: usize,
	pub y: usize,
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

