pub mod score {
	use std::cmp::Ordering;
	use std::fmt::{Display, Formatter};
	use std::fs::{File};
	use std::io::{self, Write, Read};
	use regex::Regex;
	
	#[derive(Debug, Clone)]
	pub struct DataScore {
		pub name: String,
		pub nb_points: u32,
		pub nb_lines: u32,
	}
	
	impl Display for DataScore {
		fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
			writeln!(f, "{}: points = {} lines = {}", self.name, self.nb_points, self.nb_lines)
		}
	}
	
	impl PartialOrd for DataScore {
		fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
			if self.ge(other) {
				return Some(Ordering::Greater);
			}
		
			if self.le(other) {
				return Some(Ordering::Less);
			}
			
			return Some(Ordering::Equal);
		}
		
		fn lt(&self, other: &Self) -> bool {
			self.nb_points < other.nb_points
		}
		
		fn le(&self, other: &Self) -> bool {
			self.nb_points <= other.nb_points
		}
		
		fn gt(&self, other: &Self) -> bool {
			self.nb_points > other.nb_points
		}
		
		fn ge(&self, other: &Self) -> bool {
			self.nb_points >= other.nb_points
		}
	}
	
	impl PartialEq for DataScore {
		fn eq(&self, other: &Self) -> bool {
			self.nb_points == other.nb_points && self.nb_lines == other.nb_lines
		}
		
		fn ne(&self, other: &Self) -> bool {
			self.nb_points != other.nb_points || self.nb_lines != other.nb_lines
		}
	}
	
	const SCORE_FILE_PATH: &str = "./assets/score.txt";
	
	fn extract_numbers(text: &str) -> Option<(u32, u32)> {
		let re = Regex::new(
			r"\d+"
		).unwrap();
		let result: Vec<u32> = re.find_iter(text)
			.filter_map(|digits| digits.as_str().parse().ok())
			.collect();
		if 2 == result.len() {
			Some((result[0], result[1]))
		} else {
			None
		}
	}
	
	pub fn save_score(score: &DataScore) -> io::Result<()> {
		let mut f = File::create(SCORE_FILE_PATH)?;
		writeln!(f, "{}", score)
	}
	
	pub fn read_score() -> DataScore {
		let mut f = File::open(SCORE_FILE_PATH).expect("Failed to open score file");
		let mut content = String::new();
		
		f.read_to_string(&mut content).expect("Failed to read score file");
		let numbers = extract_numbers(content.as_str());
		
		match numbers {
			Some((points, lines)) => DataScore {
				name: String::new(),
				nb_points: points,
				nb_lines: lines,
			},
			None => DataScore {
				name: String::new(),
				nb_points: 0,
				nb_lines: 0,
			},
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::DataScore;
	
	#[test]
	fn run_tests() {
		test_partial_eq();
		test_partial_cmp();
		test_display();
	}
	
	fn test_display() {
		let score1 = DataScore {
			name: String::new(),
			nb_points: 1,
			nb_lines: 1,
		};
		
		assert_eq!(": points = 1 lines = 1\n", score1.to_string());
		println!("Test display: OK");
	}
	
	fn test_partial_eq() {
		let score1 = DataScore {
			name: String::new(),
			nb_points: 1,
			nb_lines: 2,
		};
		let mut score2 = score1.clone();
		assert_eq!(score1, score2);
		
		score2.nb_points = 3;
		assert_ne!(score1, score2);
		
		score2 = score1.clone();
		score2.nb_lines = 3;
		assert_ne!(score1, score2);
		println!("Test partial EQ: OK");
	}
	
	fn test_partial_cmp() {
		let ls_score = DataScore {
			name: String::new(),
			nb_points: 1,
			nb_lines: 2,
		};
		
		let gt_score = DataScore {
			name: String::new(),
			nb_points: 10,
			nb_lines: 20,
		};
		
		assert_eq!(ls_score < gt_score, true);
		assert_eq!(ls_score <= gt_score, true);
		assert_eq!(ls_score > gt_score, false);
		assert_eq!(ls_score >= gt_score, false);
		println!("Test partial CMP: OK")
	}
}