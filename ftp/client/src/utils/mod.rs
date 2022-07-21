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
use std::fs::File;
use std::net::{IpAddr, SocketAddr};
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::{Path, PathBuf};
use async_std::{io, task};
use std::{thread::sleep, time::Duration};
use std::io::Read;
use async_std::io::WriteExt;
use chrono::{DateTime, Utc};
use log::{debug, error, info};
use regex::Regex;
use crate::{DEFAULT_ADDR};
use crate::utils::error::{FtpError, FtpResult};

pub mod connection;
pub mod error;
pub mod logger;

pub async fn get_two_args(arg: Option<String>, msg_1: &str, msg_2: &str) -> FtpResult<(String, String)> {
	let mut arg_1: String = "".to_string();
	let mut arg_2: String = "".to_string();

	if let Some(args) = arg {
		let mut split: Vec<&str> = args.split(" ").collect();
		match split.len() {
			1 => {
				arg_1 = split.get(0).unwrap().to_string();
				if let Ok(msg) = read_from_cmd_line(msg_1).await {
					arg_2 = msg.trim().to_string();
					return Ok((arg_1, arg_2));
				}
			}
			2 => {
				arg_1 = split.get(0).unwrap().to_string();
				arg_2 = split.get(1).unwrap().to_string();
				return Ok((arg_1, arg_2));
			}
			_ => {}
		}
	}

	if let Ok(msg) = read_from_cmd_line(msg_1).await {
		arg_1 = msg.trim().to_string();
		if let Ok(msg) = read_from_cmd_line(msg_2).await {
			arg_2 = msg.trim().to_string();
			return Ok((arg_1, arg_2));
		}
	}

	return Err(FtpError::InternalError("Impossible to get args".to_string()));
}

pub async fn get_one_arg(arg: Option<String>, msg: &str) -> FtpResult<String> {
	if let Some(args) = arg {
		let mut split: Vec<&str> = args.split(" ").collect();
		if split.len() >= 1 {
			return Ok(split.get(0).unwrap().to_string());
		}
	} else {
		if let Ok(msg) = read_from_cmd_line(msg).await {
			return Ok(msg.trim().to_string());
		}
	}

	return Err(FtpError::InternalError("Impossible to get arg".to_string()));
}

pub fn get_absolut_path(arg: &PathBuf, current_directory: &PathBuf) -> Option<PathBuf> {
	if let Some(p) = arg.to_str() { // Path exists
		let mut path: String = p.to_string();
		if !path.starts_with('/') { // This is a relative path
			if path.starts_with("./") || path.starts_with("~/") {
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
	let re = Regex::new(r"(([[:digit:]]{1,3}),([[:digit:]]{1,3}),([[:digit:]]{1,3}),([[:digit:]]{1,3}),([[:digit:]]{1,3}),([[:digit:]]{1,3}))").ok()?;

	let cap = re.captures(msg.as_str())?;
	if cap.len() < 6 {
		return None;
	}

	let mut addr: [u8; 4] = [0; 4];

	for i in 0..4 {
		addr[i] = cap.get(cap.len() - 6 + i).unwrap().as_str().to_string().parse::<u8>().ok()?;
	}

	let port1 = cap.get(cap.len() - 2).unwrap().as_str().to_string().parse::<u16>().ok()?;
	let port2 = cap.get(cap.len() - 1).unwrap().as_str().to_string().parse::<u16>().ok()?;
	let port = port1 * 256 + port2;

	Some((IpAddr::from(addr), port))
}

pub fn get_addr_msg(addr: SocketAddr) -> String {
	let ip = DEFAULT_ADDR.replace(".", ",");
	let port = addr.port();
	let port1 = port / 256;
	let port2 = port % 256;

	format!("{},{},{}", ip, port1, port2)
}

pub fn get_file(path: &Path) -> Option<Vec<u8>> {
	let mut result: Vec<u8> = vec![];

	match File::open(path) {
		Ok(mut file) => {
			match file.read_to_end(&mut result) {
				Ok(_) => {}
				Err(e) => {
					error!("Failed to read file: {}", e);
					return None;
				}
			};
		}
		Err(e) => {
			error!("Failed to open file {:?}: {}", path, e);
			return None;
		}
	}
	Some(result)
}

pub fn get_nls(working_path: &Path, prefix: &str) -> Vec<String> {
	let mut files_info = vec![];

	let mut filename; //  = path.as_ref().unwrap().file_name().to_str().unwrap().to_string();

	if working_path.is_dir() {
		if let Ok(paths) = fs::read_dir(working_path) {
			for path in paths {
				filename = path.as_ref().unwrap().file_name().to_str().unwrap().to_string();

				if filename.chars().next().unwrap() != '.' {
					let msg;
					if prefix.is_empty() {
						msg = filename;
					} else {
						if prefix.ends_with('/') {
							msg = format!("{}{}", prefix, filename)
						} else {
							msg = format!("{}/{}", prefix, filename)
						}
					}

					files_info.push(msg);
				}
			}
		}
	}

	files_info
}

fn get_file_info(path: &Path) -> FtpResult<String> {
	if let Ok(file) = File::open(path) {
		let metadata = file.metadata().unwrap();
		let mode = metadata.permissions().mode();
		let mut octal_right = format!("{:o}", mode);
		octal_right = octal_right[octal_right.len() - 3..octal_right.len()].to_string();
		let is_dir;

		let mut right = "".to_string();
		for c in octal_right.chars() {
			right += octal_to_string(c);
		}

		if metadata.is_dir() {
			is_dir = 'd';
		} else {
			is_dir = '-';
		}

		let modification: DateTime<Utc> = DateTime::from(metadata.modified().unwrap());

		return Ok(format!("{}{} {} {} {}      {}",
					is_dir,
					right,
					metadata.uid(),
					metadata.gid(),
					metadata.size(),
					modification.format("%Y %b %d %H:%M")));
	}
	return Err(FtpError::FileSystemError("Imposible to get file information".to_string()));
}

pub fn get_ls(path: &Path) -> Vec<String> {
	let mut files_info = vec![];

	let mut filename; //  = path.as_ref().unwrap().file_name().to_str().unwrap().to_string();


	if path.exists() {
		if path.is_dir() {
			if let Ok(paths) = fs::read_dir(path) {
				for path in paths {
					filename = path.as_ref().unwrap().file_name().to_str().unwrap().to_string();

					if filename.chars().next().unwrap() != '.' {
						if let Ok(msg) = get_file_info(path.as_ref().unwrap().path().as_path()) {
							files_info.push(format!("{} {}", msg, filename));
						}
					}
				}
			}
		} else {
			if path.is_file() {
				filename = path.file_name().unwrap().to_str().unwrap().to_string();

				if filename.chars().next().unwrap() != '.' {
					if let Ok(msg) = get_file_info(path) {
						files_info.push(format!("{} {}", msg, filename));
					}
				}
			}
		}
	}

	files_info
}

pub async fn read_from_cmd_line(msg: &str) -> FtpResult<String> {
	let stdin = io::stdin();
	let mut input_line = String::new();
	let reader = stdin.read_line(&mut input_line);

	print!("{} ", msg);
	io::stdout().flush().await;

	match reader.await {
		Ok(0) => {
			info!("Received EOF");
			return Err(FtpError::Abord("received EOF".to_string()));
		}
		Ok(n) => {
			return Ok(input_line);
		}
		_ => {
			return Err(FtpError::InternalError("Failed to read from async_std::io".to_string()));
		}
	}
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

pub fn check_word(word: &String) -> bool {
	let re = Regex::new(r"^([[:word:]]+)$").unwrap();
	re.captures(word.as_str()).is_some()
}