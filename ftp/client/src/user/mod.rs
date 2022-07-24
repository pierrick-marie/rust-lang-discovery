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

use std::fs::{File, OpenOptions};
use log::{debug, error, info};
use tokio::net::{TcpListener, TcpStream};
use std::net::{IpAddr, SocketAddr};
use log::Level::Debug;
use crate::{Connection, DEFAULT_ADDR, protocol, utils};
use crate::utils::error::{FtpError, FtpResult};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{fs, thread, time};
use async_std::{io, task};
use std::{thread::sleep, time::Duration};
use tokio::sync::mpsc::Receiver;
use crate::protocol::*;
use std::io::prelude::*;
use std::path::PathBuf;
use futures::future::err;
use log::kv::ToValue;
use portpicker::pick_unused_port;
use scanpw::scanpw;
use users::get_user_by_name;
use users::os::unix::UserExt;
use crate::user::command::*;
use crate::utils::*;

mod command;

pub struct ClientFtp {
	ctrl_connection: Connection,
	data_connection: Option<Connection>,
	current_work_directory: Option<PathBuf>,
	mode: TransfertMode,
}

impl ClientFtp {
	pub async fn new(addr: IpAddr, port: u16) -> FtpResult<Self> {
		info!("New ClientFTP {} {}", addr.to_string(), port);

		return if let Ok(socket) = TcpStream::connect(SocketAddr::new(addr, port.to_string().parse::<u16>().unwrap())).await {
			let (rx, tx) = socket.into_split();
			let mut connection = Connection::new(rx, tx);

			Ok(ClientFtp {
				ctrl_connection: connection,
				data_connection: None,
				mode: TransfertMode::Active,
				current_work_directory: None,
			})
		} else {
			Err(FtpError::ConnectionError("Impossible to init control connection".to_string()))
		};
	}

	pub async fn start(&mut self) {
		info!("START !");
		tokio::select! {
			_ = tokio::spawn(wait_ctrlc()) => {
				println!("Wait CTRL-C completed first");
			}
			_ = self.run() => {
				println!("Client run completed first");
			}
		}
		self.close_connection().await;
	}

	async fn run(&mut self) {
		if self.connect().await.is_ok() {
			self.syst().await;
			self.handle_commands().await;
		}
	}

	async fn connect(&mut self) -> FtpResult<()> {
		self.user().await?;
		self.password().await?;

		if let Some(msg) = self.ctrl_connection.read().await {
			let response = protocol::parse_server_response(&msg);
			if response.0 == ServerResponse::UserLoggedIn {
				println!("{}", msg);
				return Ok(());
			}
		}
		Err(FtpError::UserConnectionError("Failed to login".to_string()))
	}

	async fn user(&mut self) -> FtpResult<()> {
		if let Some(msg) = self.ctrl_connection.read().await {
			let response = protocol::parse_server_response(&msg);
			if response.0 == ServerResponse::ServiceReadyForNewUser {
				println!("{}", msg);
				let user_name = utils::read_from_cmd_line("Name: ").await?;
				self.ctrl_connection.send(ClientCommand::User(user_name.to_string()), None).await?;
				let user = get_user_by_name(user_name.trim());
				if user.is_some() {
					self.current_work_directory = Some(user.unwrap().home_dir().to_path_buf());
				}
				return Ok(());
			}
		}
		return Err(FtpError::UserConnectionError("Failed to send USER command to the server".to_string()));
	}

	async fn password(&mut self) -> FtpResult<()> {
		if let Some(msg) = self.ctrl_connection.read().await {
			let response = protocol::parse_server_response(&msg);
			if response.0 == ServerResponse::UserNameOkayNeedPassword {
				let handle = tokio::spawn(async move {
					let password = scanpw!("Password: ");
					return password;
				});
				let password = handle.await.unwrap();
				self.ctrl_connection.send(ClientCommand::Pass(password), None).await;
				return Ok(());
			}
		}
		return Err(FtpError::UserConnectionError("Failed to send PASS command to the server".to_string()));
	}

	async fn syst(&mut self) -> FtpResult<()> {
		self.ctrl_connection.send(ClientCommand::Syst, None).await?;
		if let Some(msg) = self.ctrl_connection.read().await {
			println!("{}", msg);
		}
		Ok(())
	}

	async fn handle_commands(&mut self) -> FtpResult<()> {
		let mut command: String;
		loop {
			command = utils::read_from_cmd_line("ftp>  ").await?;
			let command = parse_user_command(&command);
			match command {
				UserCommand::Help => { self.help(); }
				UserCommand::Unknown(arg) => { println!("Unknown command {}", arg); }
				UserCommand::Ls(arg) => {
					self.ls(arg).await;
				}
				UserCommand::Pass => { self.pass(); }
				UserCommand::Append(arg) => {
					let (local, remote) = get_two_args(arg, "(local file)", "(remote file)").await?;
					self.append(PathBuf::from(local), PathBuf::from(remote)).await?;
				}
				UserCommand::Bye => {
					self.bye().await;
					return Ok(());
				}
				UserCommand::Cd(arg) => {
					let path = get_one_arg(arg, "(directory)").await?;
					self.cd(PathBuf::from(path)).await?;
				}
				UserCommand::CdUp => {
					self.cdup().await?;
				}
				UserCommand::Delete(arg) => {
					let path = get_one_arg(arg, "(remote file)").await?;
					self.delete(PathBuf::from(path)).await?;
				}
				UserCommand::Dir => {
					self.dir().await?;
				}
				UserCommand::Exit => {
					self.bye().await;
					return Ok(());
				}
				UserCommand::Get(arg) => {
					let (remote, local) = get_two_args(arg, "(remote file)", "(local file)").await?;
					self.get(PathBuf::from(remote), PathBuf::from(local)).await?;
				}
			}
		}
	}

	fn help(&mut self) {
		println!(" Help message");
		println!(" Available commands: help ls pass append bye cd cdup delete dir exit");
	}

	fn pass(&mut self) {
		if self.mode == TransfertMode::Passive {
			self.mode = TransfertMode::Active;
			println!("Set up active mode");
		} else {
			self.mode = TransfertMode::Passive;
			println!("Set up passive mode");
		}
	}

	async fn ls(&mut self, path: Option<String>) -> FtpResult<()> {
		if let Some(file) = path {
			self.setup_data_connection(ClientCommand::List(PathBuf::from(file)), None).await?;
		} else {
			self.setup_data_connection(ClientCommand::List(self.current_work_directory.as_ref().unwrap().clone()), None).await?;
		}

		return if self.data_connection.is_some() {
			self.read_data().await
		} else {
			Err(FtpError::ConnectionError("Failed to initiate data connection".to_string()))
		};
	}

	/**
	 * Same to STOR, but if the file exists, the data are not removed.
	 */
	async fn append(&mut self, local: PathBuf, remote: PathBuf) -> FtpResult<()> {
		if let Some(path) = get_absolut_path(&local, self.current_work_directory.as_ref().unwrap()) {
			if path.exists() {
				println!("local: {} remote: {}", path.to_str().unwrap(), remote.to_str().unwrap());

				self.setup_data_connection(ClientCommand::Appe(remote), Some(ServerResponse::FileStatusOk)).await?;

				if self.data_connection.is_some() {
					if let Some(data) = utils::get_file(path.as_path()) {
						return self.send_data(vec![String::from_utf8(data).unwrap()]).await;
					}
				}
			}
		}
		error!("Data connection not initialized");
		Err(FtpError::DataConnectionError)
	}

	async fn bye(&mut self) {
		if let Some(mut connection) = self.data_connection.take() {
			connection.close().await;
		}
		if self.ctrl_connection.send(ClientCommand::Quit, None).await.is_ok() {
			if let Some(msg) = self.ctrl_connection.read().await {
				println!("{}", msg);
			}
		}

		self.ctrl_connection.close().await;
	}

	async fn cd(&mut self, path: PathBuf) -> FtpResult<()> {
		self.ctrl_connection.send(ClientCommand::Cwd(path), Some(ServerResponse::RequestedFileActionOkay)).await
	}

	async fn cdup(&mut self) -> FtpResult<()> {
		self.ctrl_connection.send(ClientCommand::CdUp, Some(ServerResponse::RequestedFileActionOkay)).await
	}

	async fn delete(&mut self, path: PathBuf) -> FtpResult<()> {
		self.ctrl_connection.send(ClientCommand::Dele(path), Some(ServerResponse::RequestedFileActionOkay)).await
	}

	async fn dir(&mut self) -> FtpResult<()> {
		self.setup_data_connection(ClientCommand::List(self.current_work_directory.as_ref().unwrap().clone()), None).await?;

		return if self.data_connection.is_some() {
			self.read_data().await
		} else {
			Err(FtpError::ConnectionError("Failed to initiate data connection".to_string()))
		};
	}

	async fn get(&mut self, remote_file: PathBuf, local_file: PathBuf) -> FtpResult<()> {

		self.setupTransferType(TransferType::Binary).await?;

		self.setup_data_connection(ClientCommand::Retr(remote_file), Some(ServerResponse::FileStatusOk)).await?;

		let file = OpenOptions::new().write(true).append(true).open(local_file)?;
		self.save_data(file).await;

		self.ctrl_connection.receive(ServerResponse::ClosingDataConnection).await
	}

	async fn setup_data_connection(&mut self, command: ClientCommand, expectedResponse: Option<ServerResponse>) -> FtpResult<()> {
		dbg!(&command);
		if self.mode == TransfertMode::Active {
			self.setup_active_transfert_mode(command, expectedResponse).await
		} else {
			self.setup_passive_transfert_mode(command, expectedResponse).await
		}
	}

	async fn setupTransferType(&mut self, transferType: TransferType) -> FtpResult<()> {
		self.ctrl_connection.write(transferType.to_string()).await?;
		self.ctrl_connection.receive(ServerResponse::OK).await
	}

	async fn setup_active_transfert_mode(&mut self, command: ClientCommand, expectedResponse: Option<ServerResponse>) -> FtpResult<()> {
		let port: u16 = pick_unused_port().expect("No ports free");
		let listener = TcpListener::bind(format!("{}:{}", DEFAULT_ADDR, port)).await?;
		let socket_addr = listener.local_addr()?;
		info!("Server listening data on {:?}", socket_addr);
		self.ctrl_connection.send(ClientCommand::Port(utils::get_addr_msg(socket_addr)), Some(ServerResponse::OK)).await?;

		self.ctrl_connection.send(command, expectedResponse).await?;

		debug!("Wait new connection");
		let (stream, addr) = listener.accept().await?;
		info!("Data connection open with addr {:?}", addr);
		let (rx, tx) = stream.into_split();
		self.data_connection = Some(Connection::new(rx, tx));

		Ok(())
	}

	async fn setup_passive_transfert_mode(&mut self, command: ClientCommand, expectedResponse: Option<ServerResponse>) -> FtpResult<()> {
		self.ctrl_connection.write(ClientCommand::Pasv.to_string()).await?;

		if let Some(msg) = self.ctrl_connection.read().await {
			let response = parse_server_response(&msg);
			if response.0 == ServerResponse::EnteringPassiveMode {
				println!("{}", msg);

				if let Some(addr) = parse_port(response.1) {
					debug!("Connect to {} {}", &addr.0, &addr.1);
					let socket = TcpStream::connect(SocketAddr::new(addr.0, addr.1)).await?;
					let (rx, tx) = socket.into_split();
					self.data_connection = Some(Connection::new(rx, tx));

					self.ctrl_connection.send(command, expectedResponse).await?;
				} else {
					return Err(FtpError::ConnectionError("Failed to parse port information".to_string()));
				}
			} else {
				return Err(FtpError::ConnectionError("Failed to get port information".to_string()));
			}
		} else {
			return Err(FtpError::ConnectionError("Failed to get pasv information".to_string()));
		}

		Ok(())
	}

	async fn read_data(&mut self) -> FtpResult<()> {
		let mut data_connection = self.data_connection.take().unwrap();

		if let Some(msg) = self.ctrl_connection.read().await {
			if parse_server_response(&msg).0 == ServerResponse::FileStatusOk {
				println!("{}", msg);
			} else {
				return Err(FtpError::ConnectionError("Failed to send LIST command".to_string()));
			}
		}

		let mut msg: Option<String> = data_connection.read().await;
		while msg.is_some() {
			println!("{}", msg.unwrap());
			msg = data_connection.read().await;
		}

		if let Some(msg) = self.ctrl_connection.read().await {
			if parse_server_response(&msg).0 == ServerResponse::ClosingDataConnection {
				println!("{}", msg);
			} else {
				return Err(FtpError::ConnectionError("Failed to transfer LIST data".to_string()));
			}
		} else {
			return Err(FtpError::ConnectionError("Failed to finish LIST command".to_string()));
		}

		data_connection.close().await;
		self.data_connection = None;

		Ok(())
	}

	async fn close_connection(&mut self) -> FtpResult<()> {
		self.ctrl_connection.close().await;
		info!("Connection closed");
		Ok(())
	}

	async fn save_data(&mut self, mut file: File) -> FtpResult<()> {
		debug!("Client::save_data");

		let mut data_connection = self.data_connection.take().unwrap();

		if let Some(data) = data_connection.read().await {
			writeln!(file, "{}", data)?;
			data_connection.close().await;
			self.data_connection = None;
			return self.ctrl_connection.write(format!("{} Transfer complete", ServerResponse::ClosingDataConnection.to_string())).await;
		}
		error!("Cannot read data connection");
		Err(FtpError::DataConnectionError)
	}

	async fn send_data(&mut self, data: Vec<String>) -> FtpResult<()> {
		let mut data_connection = self.data_connection.take().unwrap();

		tokio::select! {
			_ = async {
				for msg in data {
					data_connection.write(msg).await?;
				}
				Ok::<_, FtpError>(())
			} => {
				if let Some(msg) = self.ctrl_connection.read().await {
					let response = protocol::parse_server_response(&msg);
					if response.0 == ServerResponse::ClosingDataConnection {
						data_connection.close().await;
						self.data_connection = None;
						return Ok(());
					}
				}
				return Err(FtpError::DataConnectionError);
			}
			cmd = self.ctrl_connection.read() => {
				if cmd.is_some() {
					match parse_client_command(&cmd.as_ref().unwrap()) {
						ClientCommand::Abor => {
							data_connection.close().await;
							self.data_connection = None;
							let msg = format!("{} transfer interrupted by ABORD", ServerResponse::ConnectionClosed.to_string());
							self.ctrl_connection.write(msg).await?;
							let msg = format!("{} ABORD: ok", ServerResponse::ClosingDataConnection.to_string());
							self.ctrl_connection.write(msg).await?;
							return Err(FtpError::Abord("End of transfer file".to_string()));
						}
						_ => { }
					}
				}
			}
		}
		Ok(())
	}
}

async fn wait_ctrlc() {
	let keep_running = Arc::new(AtomicBool::new(true));
	let running = keep_running.clone();

	ctrlc::set_handler(move || {
		info!("Received CTRL-C");
		running.store(false, Ordering::SeqCst);
	}).expect("Error setting Ctrl-C handler");

	while keep_running.load(Ordering::SeqCst) {
		thread::sleep(time::Duration::from_millis(500));
	}
	debug!("End of wait CTRL-C");
}
