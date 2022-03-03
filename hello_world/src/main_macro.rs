use std::fmt::*;

trait BitSet {
	fn clear(&mut self, index: usize);
	fn is_set(&self, index: usize) -> bool;
	fn set(&mut self, index: usize);
}

macro_rules! bitset {
    ($ty:ty) => {
        impl BitSet for $ty {
            fn clear(&mut self, index: usize) {
                *self &= !(1 << index);
            }

            fn is_set(&self, index: usize) -> bool {
                (*self >> index) & 1 == 1
            }

            fn set(&mut self, index: usize) {
                *self |= 1 << index;
            }
        }
    };
}

#[derive(Clone, Copy)]
struct Point {
	x: usize,
	y: usize,
}

macro_rules! op {
	(+ $self:ident : $self_type:ty, $other:ident $expr:expr) => {
		impl ::std::ops::Add for $self_type {
			type Output = $self_type;

			fn add($self, $other: $self_type) -> $self_type {
				$expr
			}
		}
	};

	(- $self:ident : $self_type:ty, $other:ident $expr:expr) => {
		impl ::std::ops::Sub for $self_type {
			type Output = $self_type;

			fn sub($self, $other: $self_type) -> $self_type {
				$expr
			}
		}
	};

	(* $self:ident : $self_type:ty, $other:ident $expr:expr) => {
		impl ::std::ops::Mul for $self_type {
			type Output = $self_type;

			fn mul($self, $other: $self_type) -> $self_type {
				$expr
			}
		}
	};

	( (($self:ident : $self_type:ty) / $other:ident : $other_type:ty) -> $return_type:ty {$expr:expr}) => {
		impl ::std::ops::Div for $self_type {
			type Output = $return_type;

			fn div($self, $other: $other_type) -> $return_type {
				$expr
			}
		}
	};
}

macro_rules! hash {
	( $( $key:expr => $value:expr ,)* ) => { {
		let mut hashmap = ::std::collections::HashMap::new();
		$(hashmap.insert($key, $value);)*
		hashmap
	} };
}

fn main() {
	bitset!(i16);
	
	let mut i: i16 = 0;
	i.set(3);
	println!("is set is {}", i.is_set(3));
	
	op!(+ self:Point, other {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    });
	op!(- self:Point, other {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    });
	op!(* self:Point, other {
        Point {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    });
	op!( ( (self:Point) / other:Point) -> Point {
        Point {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    });
	
	let p1 = Point { x: 5, y: 7 };
	let p2 = Point { x: 2, y: 5 };
	let mut p3 = p1 + p2;
	
	println!("p3.x : {} ; p3.y : {}", p3.x, p3.y);
	
	p3 = p3 - p1;
	println!("p3.x : {} ; p3.y : {}", p3.x, p3.y);
	
	p3 = p1 * p2;
	println!("p3.x : {} ; p3.y : {}", p3.x, p3.y);
	
	p3 = p3 / p2;
	println!("p3.x : {} ; p3.y : {}", p3.x, p3.y);
	
	let my_map = hash!(
		"e" => 3,
		"h" => 4,
	);
	println!("my map {:#?}", my_map);
}
