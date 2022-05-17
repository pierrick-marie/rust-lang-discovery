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

extern crate core;
use log::{debug, error, info, Level};
use std::{env};
use std::net::{IpAddr};
use std::str::FromStr;

mod protocol;
mod server_ftp;
mod utils;
mod client_ftp;

use crate::utils::connection::Connection;
use crate::utils::logger;

pub const LOCALHOST: &str = "localhost";
pub const DEFAULT_ADDR: &str = "::1";
pub const DEFAULT_PORT: u16 = 21;

pub const LEVEL: Level = Level::Debug;

#[tokio::main]
async fn main() {
	
	// Init logger
	if let Err(e) = logger::init() {
		println!("Failed to init logger: {:?}", e);
		return;
	}
	
	// Check args
	let args: Vec<String> = env::args().collect();
	match args.len() {
		1 => {
			client_ftp::connect(IpAddr::from_str(DEFAULT_ADDR).unwrap(), DEFAULT_PORT).await;
		}
		2 => {
			if let Some(str_arg) = args.get(1) {
				if let Ok(addr) = IpAddr::from_str(str_arg) {
					client_ftp::connect(addr, DEFAULT_PORT).await;
				} else {
					error!("Address argument error: {:?}", args.get(1));
				}
			} else {
				error!("Address argument error: {:?}", args.get(1));
			}
		}
		3 => {
			if let Some(str_port) = args.get(2) {
				if let Ok(port) = str_port.parse::<u16>() {
					if let Some(arg_addr) = args.get(1) {
						arg_addr.to_lowercase();
						if arg_addr.eq(LOCALHOST) {
							let ip_addr = IpAddr::from_str(DEFAULT_ADDR).expect("Failed to create connection");
							client_ftp::connect(ip_addr, port).await;
						} else {
							let ip_addr = IpAddr::from_str(arg_addr).expect("Failed to create connection");
							client_ftp::connect(ip_addr, port).await;
						}
					} else {
						error!("Address argument error: {:?}", args.get(1));
					}
				} else {
					error!("Port argument error: {:?}", args.get(2));
				}
			} else {
				error!("Port argument error: {:?}", args.get(2));
			}
		}
		_ => {
			error!("Too many arguments {:?}", args);
			return;
		}
	};
}
