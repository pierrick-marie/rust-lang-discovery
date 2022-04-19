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

use std::fs;
use std::fs::{File, Metadata};
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::Path;

use chrono::{DateTime, Utc};

pub fn get_ls(path: &Path) -> Vec<String> {
	let mut files_info = vec![];
	
	let mut filename; //  = path.as_ref().unwrap().file_name().to_str().unwrap().to_string();
	let mut is_dir;
	let mut right: String;
	let mut modification: DateTime<Utc>; //: DateTime<Utc> = DateTime::from(metadata.modified().unwrap());
	
	let mut file: File;
	let mut metadata: Metadata;
	let mut mode;
	let mut octal_right;
	
	if path.is_dir() {
		let paths = fs::read_dir(path).unwrap();
		
		for path in paths {
			filename = path.as_ref().unwrap().file_name().to_str().unwrap().to_string();
			
			if filename.chars().next().unwrap() != '.' {
				file = File::open(path.as_ref().unwrap().path()).unwrap();
				metadata = file.metadata().unwrap();
				mode = metadata.permissions().mode();
				octal_right = format!("{:o}", mode);
				octal_right = octal_right[octal_right.len() - 3..octal_right.len()].to_string();
				
				right = "".to_string();
				for c in octal_right.chars() {
					right += octal_to_string(c);
				}
				
				if metadata.is_dir() {
					is_dir = 'd';
				} else {
					is_dir = '-';
				}
				
				modification = DateTime::from(metadata.modified().unwrap());
				
				files_info.push(format!("{}{} {} {} {}      {} {}",
				                        is_dir,
				                        right,
				                        metadata.uid(),
				                        metadata.gid(),
				                        metadata.size(),
				                        modification.format("%Y %b %d %H:%M"),
				                        filename));
			}
		}
	}
	
	files_info
}

fn octal_to_string(octal_right: char) -> &'static str {
	match octal_right {
		'0' => { "---" }
		'1' => { "--x" }
		'2' => { "-w-" }
		'3' => { "-wx" }
		'4' => { "r--" }
		'5' => { "r-x" }
		'6' => { "rw-" }
		'7' => { "rwx" }
		_ => { "" }
	}
}