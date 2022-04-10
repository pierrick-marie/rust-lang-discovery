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

mod server;
use std::alloc::System;
use std::future::Future;
use std::time::SystemTime;
use tokio::net::TcpListener;
use tokio_io::io::read_until;
use crate::server::Server;

mod protocol_codes;
mod client;
mod ftp_error;
mod message;
mod connection;
mod ftp_logger;

use tokio::sync::oneshot;
use crate::client::Client;
use crate::connection::Connection;

use log::{debug, error, info};


async fn wait_ctrl_c() {
	
	match tokio::signal::ctrl_c().await {
		Ok(()) => {
			println!("\nCtrl-C received -> end of server");
		}
		Err(err) => {
			eprintln!("Unable to listen for shutdown signal: {}", err);
			// we also shut down in case of error
		}
	}
	println!("Exit");
	std::process::exit(0);
}



#[tokio::main]
async fn main() {
	
	ftp_logger::init();
	
	debug!("Coucou");
	
	let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
	
	tokio::spawn(async move {
		Connection::new(listener).run().await;
	});
	
	wait_ctrl_c().await;
}
