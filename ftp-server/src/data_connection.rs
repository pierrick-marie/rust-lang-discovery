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

use log::{error, info};
use portpicker::pick_unused_port;
use tokio::net::{TcpListener, TcpStream};
use crate::ADDR;
use crate::connection::Connection;
use crate::ftp_error::FtpResult;
use crate::protocol_codes::ServerResponse;

pub struct DataConnection {
	data_listener: Option<TcpListener>,
	connection: Option<Connection>,
}

impl DataConnection {
	pub fn new() -> Self {
		DataConnection {
			data_listener: None,
			connection: None,
		}
	}
	
	pub async fn connection_ready(&mut self) -> FtpResult<()> {
		let (stream, addr) = self.data_listener.as_ref().unwrap().accept().await?;
		info!("Data connection open with addr {:?}", addr);
		self.connection = Some(Connection::new(stream));
		Ok(())
	}
	
	pub async fn send(&mut self, data: Vec<String>) -> FtpResult<()> {
		for msg in data {
			self.connection.unwrap().write(msg).await?;
		}
		Ok(())
	}
	
	pub async fn open_passive_connection(&mut self) -> FtpResult<(String)> {
		let port: u16 = pick_unused_port().expect("No ports free");
		let server = TcpListener::bind(format!("{}:{}", ADDR, port)).await?;
		let addr = server.local_addr().unwrap();
		info!("Server listening data on {:?}", addr);
		self.data_listener = Some(server);
		
		let ip = ADDR.replace(".", ",");
		let port = addr.port();
		let port1 = port / 256;
		let port2 = port % 256;
		
		let msg = format!("({},{},{})", ip, port1, port2);
		
		Ok((msg))
	}
}