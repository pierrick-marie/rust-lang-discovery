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
along with rust-discovery.  If not, see <http://www.gnu.org/licenses/>.
*/

/*
This file comes from the project tokio-rs/mini-redis (Licence MIT) : https://raw.githubusercontent.com/tokio-rs/mini-redis/master/src/connection.rs
I modified some parts of it.
*/

use log::{debug, error, info};
use tokio::net::{TcpListener, TcpStream};
use crate::{ADDR, Client, PORT};
use crate::connection::Connection;
use async_shutdown::Shutdown;
use std::net::SocketAddr;


pub async fn run(shutdown: Shutdown) -> std::io::Result<()> {
	
	let server = TcpListener::bind(format!("{}:{}", ADDR, PORT)).await?;
	info!("Server listening on {:?}", server.local_addr().unwrap());
	
	// Simply use `wrap_cancel` for everything, since we do not need clean-up for the listening socket.
	// See `handle_client` for a case where a future is given the time to perform logging after the shutdown was triggered.
	while let Some(connection) = shutdown.wrap_cancel(server.accept()).await {
		let (stream, address) = connection?;
		// Handle a new client
		tokio::spawn(handle_client(shutdown.clone(), stream, address));
	}
	
	Ok(())
}

async fn handle_client(shutdown: Shutdown, mut stream: TcpStream, address: SocketAddr) {
	info!("Accepted new connection from {}", address);
	
	// Make sure the shutdown doesn't complete until the delay token is dropped.
	//
	// Getting the token will fail if the shutdown has already started,
	// in which case we just log a message and return.
	//
	// If you already have a future that should be allowed to complete,
	// you can also use `shutdown.wrap_wait(...)`.
	// Here it is easier to use a token though.
	let _delay_token = match shutdown.delay_shutdown_token() {
		Ok(token) => {
			debug!("Token delay shutdown");
			token
		}
		Err(_) => {
			error!("Shutdown already started, closing connection with {}", address);
			return;
		}
	};
	
	let (rx, tx) = stream.into_split();
	let connection = Connection::new(rx, tx);
	let mut client = Client::new(connection);
	
	// Now run the echo loop, but cancel it when the shutdown is triggered.
	match shutdown.wrap_cancel(client.run()).await {
		Some(Err(e)) => error!("Error in connection {}: {:?}", address, e),
		Some(Ok(())) => info!("Connection closed by {}", address),
		None => {
			info!("Shutdown triggered, closing connection with {}", address);
			client.close_connection().await;
		}
	}
	
	// The delay token will be dropped here, allowing the shutdown to complete.
}
