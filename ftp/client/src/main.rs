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
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::mpsc;
use crate::user::ClientFtp;

use console::Term;
use dialoguer::Password;
use scanpw::scanpw;

mod protocol;
mod utils;
mod user;

use crate::utils::connection::Connection;
use crate::utils::logger;

pub const LOCALHOST: &str = "localhost";
const IPV4_ADDR: &str = "127.0.0.1";
const IPV6_ADDR: &str = "127.0.0.1";
pub const DEFAULT_ADDR: &str = IPV4_ADDR;
pub const IPV4: bool = true;
pub const IPV6: bool = !IPV4;
pub const DEFAULT_PORT: u16 = 21;

pub const LEVEL: Level = Level::Info;

use rustyline::error::ReadlineError;
use rustyline::{Editor, Result};

#[tokio::main]
async fn main() {
	run().await;
}

async fn run() {
	
	// Init logger
	if let Err(e) = logger::init() {
		println!("Failed to init logger: {:?}", e);
		return;
	}

	// Check args
	let args: Vec<String> = env::args().collect();
	let r_client = match args.len() {
		1 => {
			ClientFtp::new(IpAddr::from_str(DEFAULT_ADDR).unwrap(), DEFAULT_PORT).await
		}
		2 => {
			if let Some(str_arg) = args.get(1) {
				if let Ok(addr) = IpAddr::from_str(str_arg) {
					ClientFtp::new(addr, DEFAULT_PORT).await
				} else {
					error!("Address argument error: {:?}", args.get(1));
					return;
				}
			} else {
				error!("Address argument error: {:?}", args.get(1));
				return;
			}
		}
		3 => {
			if let Some(str_port) = args.get(2) {
				if let Ok(port) = str_port.parse::<u16>() {
					if let Some(arg_addr) = args.get(1) {
						arg_addr.to_lowercase();
						if arg_addr.eq(LOCALHOST) {
							let ip_addr = IpAddr::from_str(DEFAULT_ADDR).expect("Failed to create connection");
							ClientFtp::new(ip_addr, port).await
						} else {
							let ip_addr = IpAddr::from_str(arg_addr).expect("Failed to create connection");
							ClientFtp::new(ip_addr, port).await
						}
					} else {
						error!("Address argument error: {:?}", args.get(1));
						return;
					}
				} else {
					error!("Port argument error: {:?}", args.get(2));
					return;
				}
			} else {
				error!("Port argument error: {:?}", args.get(2));
				return;
			}
		}
		_ => {
			error!("Too many arguments {:?}", args);
			return;
		}
	};
	
	if let Ok(mut client) = r_client {
		client.start().await;
	} else {
		error!("Client is not initialised");
	}
	
	info!("Finished");
	std::process::exit(0);
}

