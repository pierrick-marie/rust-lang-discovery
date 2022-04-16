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

use std::io::Error;
use std::net::SocketAddr;
use futures::future::BoxFuture;
use futures::FutureExt;
use log::{error, info};
use portpicker::pick_unused_port;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::oneshot;
use tokio::sync::oneshot::{Receiver, Sender};
use crate::ADDR;
use crate::connection::Connection;
use crate::ftp_error::{FtpError, FtpResult};
use crate::protocol_codes::ServerResponse;

pub struct DataConnection {
	connection: Option<Connection>,
}

impl DataConnection {
	
	pub fn new() -> Self {
		
		DataConnection {
			connection: None,
		}
	}
	
	// pub async fn new_server(&mut self, sender: Sender<String>) -> FtpResult<()> {
	//
	// 	tokio::spawn(async move {
	//
	//
	// 		// if let Err(e) = sender.send(self.get_addr_msg(socket_addr)) {
	// 		// 	error!("Failed to send message through channel {:?}", e);
	// 		// 	return Err(FtpError::InternalChannelError);
	// 		// }
	//
	//
	// 		Ok(())
	// 	});
	//
	// 	Ok(())
	// }
	
	pub async fn send(&mut self, data: Vec<String>) -> FtpResult<()> {
	
		
	
		Ok(())
	}
	
	
}