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
use async_shutdown::Shutdown;
use std::net::{IpAddr, SocketAddr};
use crate::Connection;


pub async fn run(shutdown: Shutdown, connection: Connection) -> std::io::Result<()> {
	
	let mut running = true;
	
	// Simply use `wrap_cancel` for everything, since we do not need clean-up for the listening socket.
	// See `handle_client` for a case where a future is given the time to perform logging after the shutdown was triggered.
	// while let Some(connection) = shutdown.wrap_cancel(server.accept()).await {
	// while(running) {
		// Handle a new client
		// tokio::spawn(handle_client(shutdown.clone(), stream, address, id));
	// }
	
	Ok(())
}