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
use std::{env, fs, thread, time};
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
use crate::utils::cmd_line_reader::CmdLineReader;

mod command;

pub struct ClientFtp {
	ctrl_connection: Connection,
	data_connection: Option<Connection>,
	current_work_directory: Option<PathBuf>,
	mode: TransfertMode,
	cmd_reader: CmdLineReader,
	transfert_type: TransferType,
}

impl ClientFtp {
	pub async fn new(addr: IpAddr, port: u16) -> FtpResult<Self> {
		info!("New ClientFTP {} {}", addr.to_string(), port);
		
		return if let Ok(socket) = TcpStream::connect(SocketAddr::new(addr, port.to_string().parse::<u16>().unwrap())).await {
			let (rx, tx) = socket.into_split();
			let mut connection = Connection::new(rx, tx);
			let rd = CmdLineReader::new()?;
			
			Ok(ClientFtp {
				ctrl_connection: connection,
				data_connection: None,
				mode: TransfertMode::Active,
				current_work_directory: None,
				cmd_reader: rd,
				transfert_type: TransferType::Ascii,
			})
		} else {
			Err(FtpError::ConnectionError("Impossible to init control connection".to_string()))
		};
	}
	
	pub async fn start(&mut self) {
		info!("START !");
		if self.connect().await.is_ok() {
			self.syst().await;
			self.handle_commands().await;
		}
		self.close_connection().await;
	}
	
	async fn connect(&mut self) -> FtpResult<()> {
		self.user().await?;
		self.password().await?;
		
		if self.ctrl_connection.receive(ServerResponse::UserLoggedIn).await.is_err() {
			return Err(FtpError::UserConnectionError("Failed to login".to_string()));
		}
		Ok(())
	}
	
	async fn user(&mut self) -> FtpResult<()> {
		self.ctrl_connection.receive(ServerResponse::ServiceReadyForNewUser).await?;
		let user_name = self.cmd_reader.get_user_name()?;
		self.ctrl_connection.sendCommand(ClientCommand::User(user_name.to_string()), None).await?;
		let user = get_user_by_name(user_name.trim());
		if user.is_some() {
			self.current_work_directory = Some(user.unwrap().home_dir().to_path_buf());
			return Ok(());
		}
		return Err(FtpError::UserConnectionError("Failed to send USER command to the server".to_string()));
	}
	
	async fn password(&mut self) -> FtpResult<()> {
		self.ctrl_connection.receive(ServerResponse::UserNameOkayNeedPassword).await?;
		let handle = tokio::spawn(async move {
			let password = scanpw!("Password: ");
			return password;
		});
		let password = handle.await.unwrap();
		self.ctrl_connection.sendCommand(ClientCommand::Pass(password), None).await
	}
	
	async fn syst(&mut self) -> FtpResult<()> {
		self.ctrl_connection.sendCommand(ClientCommand::Syst, None).await?;
		if let Some(msg) = self.ctrl_connection.read().await {
			info!("{}", msg);
		}
		Ok(())
	}
	
	async fn handle_commands(&mut self) -> FtpResult<()> {
		let mut command: String;
		loop {
			command = self.cmd_reader.read_line("ftp>  ")?;
			let command = parse_user_command(&command);
			match command {
				UserCommand::Help => { self.help(); }
				UserCommand::Unknown(arg) => { info!("Unknown command {}", arg); }
				UserCommand::Ls(arg) => {
					self.ls(arg).await;
				}
				UserCommand::Pass => { self.pass(); }
				UserCommand::Append(arg) => {
					let (local, remote) = self.cmd_reader.get_two_args(arg, "(local file)", "(remote file)").await?;
					self.append(PathBuf::from(local), PathBuf::from(remote)).await?;
				}
				UserCommand::Bye => {
					self.bye().await;
					return Ok(());
				}
				UserCommand::Cd(arg) => {
					let path = self.cmd_reader.get_one_arg(arg, "(directory)").await?;
					self.cd(PathBuf::from(path)).await?;
				}
				UserCommand::CdUp => {
					self.cdup().await?;
				}
				UserCommand::Delete(arg) => {
					let path = self.cmd_reader.get_one_arg(arg, "(remote file)").await?;
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
					let (remote, local) = self.cmd_reader.get_two_args(arg, "(remote file)", "(local file)").await?;
					self.get(PathBuf::from(remote), PathBuf::from(local)).await?;
				}
				UserCommand::Ascii => {
					self.transfert_type = TransferType::Ascii;
					info!("Set to ASCII transfer type");
				}
				UserCommand::Image => {
					self.transfert_type = TransferType::Binary;
					info!("Set to Binary transfer type");
				}
				UserCommand::Lcd => {
					if let Ok(path) = env::current_dir() {
						info!("Set working directory to {}", path.display());
						self.current_work_directory = Some(path);
					} else {
						error!("Impossible to change working directory")
					}
				}
				UserCommand::Nlist(arg) => {
					self.nlist(arg).await?;
				}
				UserCommand::Put(arg) => {
					let (local, remote) = self.cmd_reader.get_two_args(arg, "(local file)", "(remote file)").await?;
					self.put(PathBuf::from(local), PathBuf::from(remote)).await?;
				}
				UserCommand::Pwd => {
					self.pwd().await?;
				}
				UserCommand::Quit => {
					return self.quit().await;
				}
				UserCommand::Recv(arg) => {
					let (local, remote) = self.cmd_reader.get_two_args(arg, "(local file)", "(remote file)").await?;
					self.recv(PathBuf::from(local), PathBuf::from(remote)).await?;
				}
				UserCommand::Rename(arg) => {
					let (from_name, to_name) = self.cmd_reader.get_two_args(arg, "(from-name)", "(to-name)").await?;
					self.rename(PathBuf::from(from_name), PathBuf::from(to_name)).await?;
				}
			}
		}
	}
	
	fn help(&mut self) {
		info!(" Help message");
		info!(" Available commands: help ls pass append bye cd cdup delete dir exit get ascii image lcd put pwd quit recv rename");
	}
	
	fn pass(&mut self) {
		if self.mode == TransfertMode::Passive {
			self.mode = TransfertMode::Active;
			info!("Set up active mode");
		} else {
			self.mode = TransfertMode::Passive;
			info!("Set up passive mode");
		}
	}
	
	async fn ls(&mut self, path: Option<String>) -> FtpResult<()> {
		if let Some(file) = path {
			self.setup_data_connection(ClientCommand::List(Some(PathBuf::from(file))), None).await?;
		} else {
			self.setup_data_connection(ClientCommand::List(self.current_work_directory.clone()), None).await?;
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
	async fn append(&mut self, local_file: PathBuf, remote_file: PathBuf) -> FtpResult<()> {
		if let Some(local_path) = get_absolut_path(&local_file, self.current_work_directory.as_ref().unwrap()) {
			if local_path.exists() {
				info!("local: {} remote: {}", local_path.to_str().unwrap(), remote_file.to_str().unwrap());
				
				self.setup_data_connection(ClientCommand::Appe(remote_file), Some(ServerResponse::FileStatusOk)).await?;
				
				if self.data_connection.is_some() {
					if let Some(data) = utils::get_file(local_path.as_path()) {
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
		if self.ctrl_connection.sendCommand(ClientCommand::Quit, None).await.is_ok() {
			if let Some(msg) = self.ctrl_connection.read().await {
				info!("{}", msg);
			}
		}
		
		self.ctrl_connection.close().await;
	}
	
	async fn cd(&mut self, path: PathBuf) -> FtpResult<()> {
		self.ctrl_connection.sendCommand(ClientCommand::Cwd(path), Some(ServerResponse::RequestedFileActionOkay)).await
	}
	
	async fn cdup(&mut self) -> FtpResult<()> {
		self.ctrl_connection.sendCommand(ClientCommand::CdUp, Some(ServerResponse::RequestedFileActionOkay)).await
	}
	
	async fn delete(&mut self, path: PathBuf) -> FtpResult<()> {
		self.ctrl_connection.sendCommand(ClientCommand::Dele(path), Some(ServerResponse::RequestedFileActionOkay)).await
	}
	
	async fn dir(&mut self) -> FtpResult<()> {
		self.setup_data_connection(ClientCommand::List(self.current_work_directory.clone()), None).await?;
		
		return if self.data_connection.is_some() {
			self.read_data().await
		} else {
			Err(FtpError::ConnectionError("Failed to initiate data connection".to_string()))
		};
	}
	
	async fn get(&mut self, remote_file: PathBuf, local_file: PathBuf) -> FtpResult<()> {
		if let Some(local_path) = get_absolut_path(&local_file, self.current_work_directory.as_ref().unwrap()) {
			if local_path.exists() {
				info!("local: {} remote: {}", local_path.to_str().unwrap(), remote_file.to_str().unwrap());
				
				self.transfert_type = TransferType::Binary;
				self.setupTransferType(self.transfert_type).await?;
				
				self.setup_data_connection(ClientCommand::Retr(remote_file), Some(ServerResponse::FileStatusOk)).await?;
				
				let file = OpenOptions::new().write(true).append(false).open(local_path)?;
				self.save_data(file).await?;
				
				return self.ctrl_connection.receive(ServerResponse::ClosingDataConnection).await;
			}
		}
		Err(FtpError::InternalError("Failed to get file".to_string()))
	}
	
	async fn nlist(&mut self, arg: Option<String>) -> FtpResult<()> {
		let paths: Vec<&str>;
		let args: String;
		
		if arg.is_some() {
			args = arg.unwrap();
			paths = args.split(" ").collect();
			for path in paths {
				self.setup_data_connection(ClientCommand::Nlist(Some(PathBuf::from(path))), None).await?;
				self.read_data().await?;
			}
		} else {
			self.setup_data_connection(ClientCommand::Nlist(None), None).await?;
			self.read_data().await?;
		}
		
		Ok(())
	}
	
	/**
	 * STOR command
	 */
	async fn put(&mut self, local_file: PathBuf, remote_file: PathBuf) -> FtpResult<()> {
		if let Some(local_path) = get_absolut_path(&local_file, self.current_work_directory.as_ref().unwrap()) {
			if local_path.exists() {
				info!("local: {} remote: {}", local_path.to_str().unwrap(), remote_file.to_str().unwrap());
				
				self.setup_data_connection(ClientCommand::Stor(remote_file), Some(ServerResponse::FileStatusOk)).await?;
				
				if self.data_connection.is_some() {
					if let Some(data) = utils::get_file(local_path.as_path()) {
						return self.send_data(vec![String::from_utf8(data).unwrap()]).await;
					}
				}
			}
		}
		error!("Data connection not initialized");
		Err(FtpError::DataConnectionError)
	}
	
	async fn pwd(&mut self) -> FtpResult<()> {
		self.ctrl_connection.sendCommand(ClientCommand::Pwd, Some(ServerResponse::PathNameCreated)).await
	}
	
	async fn quit(&mut self) -> FtpResult<()> {
		self.ctrl_connection.sendCommand(ClientCommand::Quit, Some(ServerResponse::ServiceClosingControlConnection)).await
	}
	
	/**
	 * RETR command
	 */
	async fn recv(&mut self, local_file: PathBuf, remote_file: PathBuf) -> FtpResult<()> {
		if let Some(local_path) = get_absolut_path(&local_file, self.current_work_directory.as_ref().unwrap()) {
			if local_path.exists() {
				info!("local: {} remote: {}", local_path.to_str().unwrap(), remote_file.to_str().unwrap());
				
				self.transfert_type = TransferType::Binary;
				self.setupTransferType(self.transfert_type).await?;
				
				self.setup_data_connection(ClientCommand::Retr(remote_file), Some(ServerResponse::FileStatusOk)).await?;
				
				let file = OpenOptions::new().write(true).append(true).open(local_path)?;
				self.save_data(file).await?;
				
				return self.ctrl_connection.receive(ServerResponse::ClosingDataConnection).await;
			}
		}
		Err(FtpError::InternalError("Failed to get file".to_string()))
	}
	
	async fn rename(&mut self, from_name: PathBuf, to_name: PathBuf) -> FtpResult<()> {
		self.ctrl_connection.sendCommand(ClientCommand::Rnfr(from_name), Some(ServerResponse::RequestedFileActionPendingFurtherInformation)).await?;
		self.ctrl_connection.sendCommand(ClientCommand::Rnto(to_name), Some(ServerResponse::RequestedFileActionOkay)).await
	}
	
	async fn setup_data_connection(&mut self, command: ClientCommand, expectedResponse: Option<ServerResponse>) -> FtpResult<()> {
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
		debug!("Server listening data on {:?}", socket_addr);
		self.ctrl_connection.sendCommand(ClientCommand::Port(utils::get_addr_msg(socket_addr)), Some(ServerResponse::OK)).await?;
		
		self.ctrl_connection.sendCommand(command, expectedResponse).await?;
		
		debug!("Wait new connection");
		let (stream, addr) = listener.accept().await?;
		debug!("Data connection opened with addr {:?}", addr);
		let (rx, tx) = stream.into_split();
		self.data_connection = Some(Connection::new(rx, tx));
		
		Ok(())
	}
	
	async fn setup_passive_transfert_mode(&mut self, command: ClientCommand, expectedResponse: Option<ServerResponse>) -> FtpResult<()> {
		self.ctrl_connection.write(ClientCommand::Pasv.to_string()).await?;
		if let Some(msg) = self.ctrl_connection.read().await {
			let response = parse_server_response(&msg);
			if response.0 == ServerResponse::EnteringPassiveMode {
				info!("{}", msg);
				if let Some(addr) = parse_port(response.1) {
					debug!("Connect to {} {}", &addr.0, &addr.1);
					let socket = TcpStream::connect(SocketAddr::new(addr.0, addr.1)).await?;
					let (rx, tx) = socket.into_split();
					self.data_connection = Some(Connection::new(rx, tx));
					self.ctrl_connection.sendCommand(command, expectedResponse).await?;
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
		self.ctrl_connection.receive(ServerResponse::FileStatusOk).await?;
		
		let mut msg: Option<String> = data_connection.read().await;
		while msg.is_some() {
			info!("{}", msg.unwrap());
			msg = data_connection.read().await;
		}
		
		self.ctrl_connection.receive(ServerResponse::ClosingDataConnection).await?;
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
			return Ok(());
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
				self.ctrl_connection.receive(ServerResponse::ClosingDataConnection).await?;
				data_connection.close().await;
				self.data_connection = None;
			}
			cmd = self.ctrl_connection.read() => {
				if cmd.is_some() {
					match parse_client_command(&cmd.as_ref().unwrap()) {
						ClientCommand::Abor => {
							data_connection.close().await;
							self.data_connection = None;
							self.ctrl_connection.sendResponse(ServerResponse::ConnectionClosed, "transfer interrupted by ABORD").await?;
							self.ctrl_connection.sendResponse(ServerResponse::ClosingDataConnection, "ABORD: ok").await?;
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