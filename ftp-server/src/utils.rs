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
use std::net::{IpAddr, SocketAddr};
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::{Path, PathBuf};

use chrono::{DateTime, Utc};
use log::debug;
use regex::Regex;
use crate::ADDR;

pub fn get_absolut_path(arg: &PathBuf, current_directory: &PathBuf) -> Option<PathBuf> {
	if let Some(p) = arg.to_str() { // Path exists
		let mut path: String = p.to_string();
		if !path.starts_with('/') { // This is a relative path
			if path.starts_with("./") {
				path.remove(0); // removing the first char (.)
				path.remove(0); // removing the new first char (/)
			}
			path = format!("{}/{}", current_directory.to_str().unwrap(), path);
		}
		if path.ends_with('/') {
			path.pop();
		}
		return Some(PathBuf::from(path));
	}
	None
}

pub fn parse_port(msg: String) -> Option<(IpAddr, u16)> {
	debug!("client::parse_port {}", msg);
	let re = Regex::new(r"^([[:digit:]]{1,3}),([[:digit:]]{1,3}),([[:digit:]]{1,3}),([[:digit:]]{1,3}),([[:digit:]]{1,3}),([[:digit:]]{1,3})$").ok()?;
	let cap = re.captures(msg.as_str())?;
	
	let mut addr: [u8; 4] = [0; 4];
	for i in 1..5 {
		addr[i - 1] = cap.get(i).unwrap().as_str().to_string().parse::<u8>().ok()?;
	}
	
	let port1 = cap.get(5).unwrap().as_str().to_string().parse::<u16>().ok()?;
	let port2 = cap.get(6).unwrap().as_str().to_string().parse::<u16>().ok()?;
	let port = port1 * 256 + port2;
	
	Some((IpAddr::from(addr), port))
}

pub fn get_addr_msg(addr: SocketAddr) -> String {
	let ip = ADDR.replace(".", ",");
	let port = addr.port();
	let port1 = port / 256;
	let port2 = port % 256;
	
	format!("({},{},{})", ip, port1, port2)
}

pub fn get_ls(path: &Path) -> Vec<String> {
	let mut files_info = vec![];
	
	let mut filename; //  = path.as_ref().unwrap().file_name().to_str().unwrap().to_string();
	let mut is_dir;
	let mut right: String;
	let mut modification: DateTime<Utc>; //: DateTime<Utc> = DateTime::from(metadata.modified().unwrap());
	
	let mut metadata: Metadata;
	let mut mode;
	let mut octal_right;
	
	if path.is_dir() {
		if let Ok(paths) = fs::read_dir(path) {
			for path in paths {
				filename = path.as_ref().unwrap().file_name().to_str().unwrap().to_string();
				
				if filename.chars().next().unwrap() != '.' {
					if let Ok(file) = File::open(path.as_ref().unwrap().path()) {
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