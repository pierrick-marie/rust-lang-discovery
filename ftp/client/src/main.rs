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
use std::env;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;

mod protocol;
mod server_ftp;
mod utils;
mod client_ftp;

use async_shutdown::Shutdown;
use tokio::net::{TcpListener, TcpStream};
use crate::utils::connection::Connection;
use crate::utils::logger;


pub const LOCALHOST: &str = "localhost";
pub const DEFAULT_ADDR: &str = "::1";
pub const DEFAULT_PORT: u16 = 21;

pub const LEVEL: Level = Level::Info;

async fn wait_ctrl_c(shutdown: Shutdown) {
	
	// Spawn a task to wait for CTRL+C and trigger a shutdown.
	tokio::spawn({
		async move {
			if let Err(e) = tokio::signal::ctrl_c().await {
				error!("Failed to wait for CTRL+C: {}", e);
				std::process::exit(1);
			} else {
				info!("Received interrupt signal. Shutting down server_ftp...");
				shutdown.shutdown();
			}
		}
	});
}

async fn connect(addr: IpAddr, port: u16) {
	
	info!("Connecting with {} {}", addr.to_string(), port);
	
	let socket = TcpStream::connect(SocketAddr::new(addr, port.to_string().parse::<u16>().unwrap())).await.unwrap();
	let (rx, tx) = socket.into_split();
	
	// Create a new shutdown object.
	// We will clone it into all tasks that need it.
	let shutdown = Shutdown::new();
	
	wait_ctrl_c(shutdown.clone()).await;
	
	// Run the server_ftp and set a non-zero exit code if we had an error.
	let exit_code = match client_ftp::run(shutdown.clone(), Connection::new(rx, tx)).await {
		Ok(()) => 0,
		Err(e) => {
			error!("Server task finished with an error: {}", e);
			1
		}
	};
	
	// Wait for clients to run their cleanup code, then exit.
	// Without this, background tasks could be killed before they can run their cleanup code.
	shutdown.wait_shutdown_complete().await;
	
	std::process::exit(exit_code);
}

#[tokio::main]
async fn main() {
	if let Err(e) = logger::init() {
		println!("Failed to init logger: {:?}", e);
		return;
	}
	
	let args: Vec<String> = env::args().collect();
	match args.len() {
		1 => {
			connect(IpAddr::from_str(DEFAULT_ADDR).unwrap(), DEFAULT_PORT).await;
		}
		2 => {
			if let Ok(addr) = IpAddr::from_str(&*args.get(1).unwrap().to_string()) {
				connect(addr, DEFAULT_PORT).await;
			} else {
				error!("Address argument error: {:?}", args.get(1));
			}
		}
		3 => {
			if let Ok(port) = args.get(2).unwrap().to_string().parse::<u16>() {
				if let Some(arg_addr) = args.get(1) {
					arg_addr.to_lowercase();
					if arg_addr.eq(LOCALHOST) {
						let ip_addr = IpAddr::from_str(DEFAULT_ADDR).expect("Failed to create connection");
						connect(ip_addr, port).await;
					} else {
						let ip_addr = IpAddr::from_str(arg_addr).expect("Failed to create connection");
						connect(ip_addr, port).await;
					}
				} else {
					error!("Address argument error: {:?}", args.get(1));
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


