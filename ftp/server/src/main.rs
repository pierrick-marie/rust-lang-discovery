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

use async_shutdown::Shutdown;

mod protocol;
mod client;
mod ftp_error;
mod server;
mod ftp_logger;
mod connection;
mod utils;
use crate::client::Client;

pub const ADDR: &str = "127.0.0.1";
pub const PORT: &str = "8080";

pub const LEVEL: Level = Level::Info;

async fn wait_ctrl_c(shutdown: Shutdown) {
	
	// Spawn a task to wait for CTRL+C and trigger a shutdown.
	tokio::spawn({
		async move {
			if let Err(e) = tokio::signal::ctrl_c().await {
				error!("Failed to wait for CTRL+C: {}", e);
				std::process::exit(1);
			} else {
				info!("Received interrupt signal. Shutting down server...");
				shutdown.shutdown();
			}
		}
	});
}


async fn server() {
	// Create a new shutdown object.
	// We will clone it into all tasks that need it.
	let shutdown = Shutdown::new();
	
	wait_ctrl_c(shutdown.clone()).await;
	
	// Run the server and set a non-zero exit code if we had an error.
	let exit_code = match server::run(shutdown.clone()).await {
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
	
	if let Err(e) = ftp_logger::init() {
		error!("Failed to init logger: {:?}", e);
	}
	
	server().await;
}


