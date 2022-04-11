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

use tokio::net::TcpListener;
use tokio::sync::oneshot;
use log::{debug, error, info};
use regex::{Regex};

mod protocol_codes;
mod client;
mod ftp_error;
mod server;
mod ftp_logger;
mod connection;
use crate::client::Client;
use crate::server::Server;

async fn wait_ctrl_c() {
	info!("Wainting for Ctrl-C");
	match tokio::signal::ctrl_c().await {
		Ok(()) => {
			info!("Ctrl-C received -> end of server");
		}
		Err(err) => {
			error!("Unable to listen for shutdown signal: {}", err);
			// we also shut down in case of error
		}
	}
}


#[tokio::main]
async fn main() {
	ftp_logger::init();
	
	let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
	let mut server = Server::new(listener);
	
	tokio::select! {
            _ = server.run() => { },
		_ = wait_ctrl_c() => {
                  server.close_connections().await;
		}
	}
	
	info!("Exit");
}
