use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::fs::{File};
use std::io::{Write, Read};
use regex::Regex;
use chrono::{Utc};

#[derive(Debug, Clone)]
pub struct Score {
	date: String,
	pub nb_points: u32,
	pub nb_lines: u32,
}

const SIMPLE_MULTIPLICATOR: f32 = 1.5;
const BIG_MULTIPLICATOR: f32 = 2.5;

const DISPLAY_POINTS: &str = " : points = ";
const DISPLAY_LINES: &str = " lines = ";
const SCORE_FILE_PATH: &str = "./assets/score.txt";
const DATE_FORMAT: &str = "%Y-%m-%d %Hh %Mm";

impl Display for Score {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		writeln!(f, "{}{}{}{}{}", self.date, DISPLAY_POINTS, self.nb_points, DISPLAY_LINES, self.nb_lines)
	}
}

impl Score {
	pub fn new() -> Score {
		Score {
			date: "".to_string(),
			nb_points: 0,
			nb_lines: 0,
		}
	}
	
	pub fn add_line(&mut self, nb_lines: u32) {
		self.nb_lines += nb_lines;
		
		if 4 == nb_lines {
			self.nb_points += ((nb_lines as f32) * BIG_MULTIPLICATOR) as u32;
		} else {
			self.nb_points += ((nb_lines as f32) * SIMPLE_MULTIPLICATOR) as u32;
		}
	}
}

impl Eq for Score {}

impl PartialEq for Score {
	fn eq(&self, other: &Self) -> bool {
		self.nb_points == other.nb_points && self.nb_lines == other.nb_lines
	}
	
	fn ne(&self, other: &Self) -> bool {
		self.nb_points != other.nb_points || self.nb_lines != other.nb_lines
	}
}

impl Ord for Score {
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

pub fn save(score: &mut Score) {
	
	let now = Utc::now();
	score.date = now.format(DATE_FORMAT).to_string();
	
	let mut saved_result = read_score();
	saved_result.push(score.clone());
	saved_result.sort();
	saved_result.reverse();
	
	let mut f = File::create(SCORE_FILE_PATH).expect("Failed to open score file");
	
	for it_score in saved_result {
		write!(f, "{}", it_score).expect("Failed to save scores")
	}
}

pub fn read_score() -> Vec<Score> {
	let mut scores: Vec<Score> = vec![];
	let file_content = read_score_file();
	
	for line in file_content.split("\n").collect::<Vec<&str>>() {
		if !line.is_empty() {
			scores.push(extract_numbers(line).unwrap());
		}
	}
	
	return scores;
}

fn read_score_file() -> String {
	let mut content = String::new();
	let  result = File::open(SCORE_FILE_PATH);
	match result {
		Ok(mut f) => {
			match f.read_to_string(&mut content) {
				Ok(_) => return content,
				Err(_) => {
					println!("Failed to read score file");
					return content
				},
			};
		}
		Err(_) =>{
			println!("Failed to open score file");
			content
		}
	}
}

fn extract_numbers(text: &str) -> Option<Score> {
	let re = Regex::new(
		r"(.+) : points = (\d+) lines = (\d+)"
	).unwrap();
	let cap = re.captures(text).unwrap();
	
	// there are three patterns in the regex + the text itself
	if 4 == cap.len() {
		Some(Score {
			date: String::from(cap.get(1).unwrap().as_str()),
			nb_points: cap.get(2).unwrap().as_str().parse::<u32>().unwrap(),
			nb_lines: cap.get(3).unwrap().as_str().parse::<u32>().unwrap(),
		})
	} else {
		None
	}
}

#[cfg(test)]
mod tests {
	use std::cmp::Ordering;
	use crate::model::score;
	use crate::model::score::Score;
	use chrono::{Utc};
	use std::fs;
	
	#[test]
	fn run_tests() {
		test_eq();
		test_ord();
		test_display();
		test_save();
	}
	
	fn test_save() {
		fs::remove_file(score::SCORE_FILE_PATH).expect("Failed to remove score file");
		
		let mut result_to_save = vec![
			Score { date: "".to_string(), nb_points: 32, nb_lines: 12 },
			Score { date: "".to_string(), nb_points: 42, nb_lines: 6 },
			Score { date: "".to_string(), nb_points: 52, nb_lines: 3 }
		];
		
		for score in &mut result_to_save {
			score::save(score);
		}
		
		let saved_result = score::read_score();
		assert_eq!(result_to_save, saved_result);
		println!("Test save score OK");
	}
	
	fn test_display() {
		let now = Utc::now();
		
		let score1 = Score {
			date: now.format(score::DATE_FORMAT).to_string(),
			nb_points: 1,
			nb_lines: 1,
		};
		
		assert_eq!(format!("{} : points = 1 lines = 1\n", now.format(score::DATE_FORMAT).to_string()), score1.to_string());
		println!("Test display: OK");
	}
	
	fn test_eq() {
		let now = Utc::now();
		
		let score1 = Score {
			date: now.format(score::DATE_FORMAT).to_string(),
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
		let now = Utc::now();
		
		let ls_score = Score {
			date: now.format(score::DATE_FORMAT).to_string(),
			nb_points: 1,
			nb_lines: 2,
		};
		
		let gt_score = Score {
			date: now.format(score::DATE_FORMAT).to_string(),
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
			date: now.format(score::DATE_FORMAT).to_string(),
			nb_points: 5,
			nb_lines: 10,
		};
		let middle_score_result = middle_score_test.clone();
		assert_eq!(middle_score_test.clamp(ls_score, gt_score), middle_score_result);
		
		println!("Test ord: OK")
	}
}