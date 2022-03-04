pub mod score {
	use std::cmp::Ordering;
	use std::fmt::{Display, Formatter};
	use std::fs::{File};
	use std::io::{Write, Read};
	use regex::Regex;
	
	#[derive(Debug, Clone)]
	pub struct Score {
		pub name: String,
		pub nb_points: u32,
		pub nb_lines: u32,
	}
	
	impl Display for Score {
		fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
			writeln!(f, "{}: points = {} lines = {}", self.name, self.nb_points, self.nb_lines)
		}
	}
	
	impl Eq for Score {}
	
	impl Ord for Score{
		fn cmp(&self, other: &Self) -> Ordering {
			self.partial_cmp(other).unwrap()
		}
		
		fn max(self, other: Self) -> Self where Self: Sized {
			if self > other {
				self
			} else {
				other
			}
		}
		
		fn min(self, other: Self) -> Self where Self: Sized {
			if self < other {
				self
			} else {
				other
			}
		}
		
		fn clamp(self, min: Self, max: Self) -> Self where Self: Sized {
			if self < min {
				return min;
			}
			
			if self > max {
				return max;
			}
			
			return self;
		}
	}
	
	impl PartialOrd for Score {
		fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
			if self.gt(other) {
				return Some(Ordering::Greater);
			}
		
			if self.lt(other) {
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
	
	impl PartialEq for Score {
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
	
	pub fn save_score(scores: &mut Vec<Score>) {
		let mut f = File::create(SCORE_FILE_PATH).expect("Failed to open score file");
		
		scores.sort();
		
		for score in scores {
			write!(f, "{}", score).expect("Failed to save scores")
		}
	}
	
	pub fn read_score() -> Vec<Score> {
		
		let mut scores: Vec<Score> = vec![];
		let file_content = read_score_file();
		
		for line in file_content.split("\n").collect::<Vec<&str>>() {
			if !line.is_empty() {
				scores.push(get_score_line(String::from(line)));
			}
		}
		
		return scores;
	}
	
	fn read_score_file() -> String {
		
		let mut f = File::open(SCORE_FILE_PATH).expect("Failed to open score file");
		let mut content = String::new();
		
		match f.read_to_string(&mut content) {
			Ok(_) => return content,
			Err(error) => panic!("Failed to read score file file: {:?}", error),
		};
	}
	
	fn get_score_line(line: String) -> Score {
		
		let numbers = extract_numbers(&*line);
		let name = line.split(':').collect::<Vec<&str>>()[0];
		
		match numbers {
			Some((points, lines)) => Score {
				name: String::from(name),
				nb_points: points,
				nb_lines: lines,
			},
			None => Score {
				name: String::new(),
				nb_points: 0,
				nb_lines: 0,
			},
		}
	}
}

#[cfg(test)]
mod tests {
	use std::cmp::Ordering;
	use crate::Score;
	
	#[test]
	fn run_tests() {
		test_eq();
		test_ord();
		test_display();
	}
	
	fn test_display() {
		let score1 = Score {
			name: String::new(),
			nb_points: 1,
			nb_lines: 1,
		};
		
		assert_eq!(": points = 1 lines = 1\n", score1.to_string());
		println!("Test display: OK");
	}
	
	fn test_eq() {
		let score1 = Score {
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
		println!("Test eq: OK");
	}
	
	fn test_ord() {
		let ls_score = Score {
			name: String::new(),
			nb_points: 1,
			nb_lines: 2,
		};
		
		let gt_score = Score {
			name: String::new(),
			nb_points: 10,
			nb_lines: 20,
		};
		
		assert_eq!(ls_score < gt_score, true);
		assert_eq!(gt_score > ls_score, true);
		assert_eq!(ls_score <= ls_score, true);
		assert_eq!(ls_score >= ls_score, true);
		
		assert_eq!(ls_score.cmp(&gt_score), Ordering::Less);
		assert_eq!(gt_score.cmp(&ls_score), Ordering::Greater);
		assert_eq!(ls_score.cmp(&ls_score), Ordering::Equal);
		
		let middle_score_test = Score {
			name: String::new(),
			nb_points: 5,
			nb_lines: 10,
		};
		let middle_score_result = middle_score_test.clone();
		assert_eq!(middle_score_test.clamp(ls_score, gt_score), middle_score_result);
		
		println!("Test ord: OK")
	}
}