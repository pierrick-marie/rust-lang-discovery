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

use log::{debug, error, info};
use tokio::net::{TcpListener, TcpStream};
use std::net::{IpAddr, SocketAddr};
use log::Level::Debug;
use crate::Connection;
use crate::utils::error::{FtpError, FtpResult};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub async fn connect(addr: IpAddr, port: u16) {
	info!("Connecting with {} {}", addr.to_string(), port);
	
	if let Ok(socket) = TcpStream::connect(SocketAddr::new(addr, port.to_string().parse::<u16>().unwrap())).await {
		let (rx, tx) = socket.into_split();
		
		// Init CTRL-C
		let keep_running = Arc::new(AtomicBool::new(true));
		let r = keep_running.clone();
		ctrlc::set_handler(move || {
			r.clone().store(false, Ordering::SeqCst);
		}).expect("Error setting Ctrl-C handler");
		
		
		tokio::spawn(async move {
			run(keep_running.clone(), Connection::new(rx, tx), "Plop 0".to_string()).await;
		});
	} else {
		error!("Failed to connect to the server");
	}
}

pub async fn run(keep_running: Arc<AtomicBool>, connection: Connection, msg: String) -> FtpResult<()> {
	while keep_running.load(Ordering::SeqCst) {
		println!("{}", msg);
	}
	
	close_connection().await
}

async fn close_connection() -> FtpResult<()> {
	debug!("Finished");
	Ok(())
}