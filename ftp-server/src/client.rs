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

use std::borrow::Borrow;
use std::fmt::format;
use std::fs;
use std::io;
use std::io::{Error, ErrorKind};
use std::net::{IpAddr, SocketAddr};
use std::path::{Path, PathBuf};
use crate::protocol_codes::*;
use regex::{Regex};

use log::{debug, error, info, warn};
use tokio::net::{TcpListener, TcpStream};
use crate::{ADDR, connection, utils};
use crate::connection::Connection;
use crate::ftp_error::{FtpError, FtpResult};
use portpicker::pick_unused_port;
use regex::internal::Input;

use users::{get_user_by_name, User};
use users::os::unix::UserExt;
use crate::protocol_codes::TransfertMode::*;

pub struct Client {
	ctrl_connection: Connection,
	data_connection: Option<Connection>,
	transfert_mode: TransfertMode,
	transfert_type: TransferType,
	user: Option<User>,
	current_work_directory: Option<PathBuf>,
	current_working_path: Option<PathBuf>,
	id: i32,
}

impl Client {
	pub fn new(connection: Connection, id: i32) -> Self {
		Client {
			ctrl_connection: connection,
			data_connection: None,
			transfert_mode: Active,
			transfert_type: TransferType::Ascii,
			user: None,
			current_work_directory: None,
			current_working_path: None,
			id,
		}
	}
	
	pub async fn run(&mut self) -> std::io::Result<()> {
		if let Err(e) = self.ctrl_connection.write(ServerResponse::ServiceReadyForNewUser.to_string()).await {
			return Err(Error::new(ErrorKind::NotConnected, e.to_string()));
		}
		
		if self.connect().await {
			info!("Connected {}", self.user.as_ref().unwrap().name().to_str().unwrap());
			if let Err(e) = self.command().await {
				error!("{}", e);
			}
		} else {
			error!("Not connected");
		}
		
		self.close_connection().await;
		Ok(())
	}
	
	async fn connect(&mut self) -> bool {
		match self.user().await {
			Some(login) => {
				info!("Login: {}", login);
				if let Err(e) = self.ctrl_connection.write(ServerResponse::UserNameOkayNeedPassword.to_string()).await {
					error!("Not connected {:?}", e);
				}
				if self.password().await.is_some() {
					info!("Password: \"x\"");
					let user = get_user_by_name(login.as_str());
					if user.is_some() {
						self.user = user.clone();
						self.current_work_directory = Some(user.unwrap().home_dir().to_path_buf());
						if let Err(e) = self.ctrl_connection.write(ServerResponse::UserLoggedIn.to_string()).await {
							error!("Not connected {:?}", e);
						}
						return true;
					}
				}
			}
			_ => {}
		}
		if let Err(e) = self.ctrl_connection.write(ServerResponse::NotLoggedIn.to_string()).await {
			error!("Not connected {:?}", e);
		}
		false
	}
	
	async fn user(&mut self) -> Option<String> {
		debug!("client::user");
		let msg = self.ctrl_connection.read().await?;
		
		return match self.parse_command(&msg) {
			ClientCommand::User(args) => {
				if self.check_word(&args) {
					Some(args.clone())
				} else {
					error!("User name error: {}", args);
					None
				}
			}
			err => {
				error!("Unexpected command: {}", err);
				None
			}
		};
	}
	
	async fn password(&mut self) -> Option<String> {
		debug!("client::password");
		let msg = self.ctrl_connection.read().await?;
		return match self.parse_command(&msg) {
			ClientCommand::Pass(args) => {
				if self.check_word(&args) {
					info!("PASSWORD xxx");
					Some(args.clone())
				} else {
					error!("Password error: {}", args);
					None
				}
			}
			_ => {
				error!("Unexpected command: {}", msg);
				None
			}
		};
	}
	
	fn parse_command(&self, msg: &String) -> ClientCommand {
		debug!("client::parse_command '{}'", msg);
		if let Some(re) = Regex::new(r"^([[:upper:]]{3,4})( .+)*$").ok() {
			if let Some(cap) = re.captures(msg.as_str()) {
				if let Some(cmd) = cap.get(1) {
					if let Some(args) = cap.get(2) {
						return ClientCommand::new(cmd.as_str(), args.as_str().to_string().trim());
					} else {
						return ClientCommand::new(cmd.as_str(), "");
					}
				}
			}
		}
		error!("failed to parse command: {}", msg);
		ClientCommand::Unknown(msg.clone())
	}
	
	fn check_word(&self, username: &String) -> bool {
		let re = Regex::new(r"^([[:word:]]+)$").unwrap();
		re.captures(username.as_str()).is_some()
	}
	
	async fn command(&mut self) -> FtpResult<()> {
		debug!("client::command");
		let mut msg = self.ctrl_connection.read().await;
		while msg.is_some() {
			debug!("Message received: {:?}", msg);
			match self.parse_command(&msg.as_ref().unwrap()) {
				ClientCommand::Abor => {
					self.abor().await?;
				}
				ClientCommand::Acct(arg) => {
					self.acct(arg).await?;
				}
				ClientCommand::Allo(arg) => {
					self.allo(arg).await?;
				}
				ClientCommand::Appe(arg) => {
					self.appe(arg).await?;
				}
				ClientCommand::CdUp => {
					self.cdup().await?;
				}
				ClientCommand::Cwd(arg) => {
					self.cwd(arg).await?;
				}
				ClientCommand::Dele(arg) => {
					self.dele(arg).await?;
				}
				ClientCommand::Help(arg) => {
					self.help(arg).await?;
				}
				ClientCommand::List(arg) => {
					self.list(arg).await?;
				}
				ClientCommand::Mkd(arg) => {
					self.mkdir(arg).await?;
				}
				ClientCommand::Mode => {
					self.mode().await?;
				}
				ClientCommand::Nlst(arg) => {
					self.nlst(arg).await?;
				}
				ClientCommand::NoOp => {
					self.noop().await?;
				}
				ClientCommand::Pass(arg) => {
					// See connect() function
				}
				ClientCommand::Pasv => {
					self.pasv().await?;
				}
				ClientCommand::Port(arg) => {
					self.port(arg).await?;
				}
				ClientCommand::Pwd => {
					self.pwd().await?;
				}
				ClientCommand::Quit => {
					self.ctrl_connection.write(ServerResponse::ServiceClosingControlConnection.to_string()).await?;
					self.user = None;
					self.ctrl_connection.close();
					return Ok(());
				}
				ClientCommand::Rein => {
					self.rein().await?;
				}
				ClientCommand::Rest(arg) => {
					self.rest(arg).await?;
				}
				ClientCommand::Retr(arg) => {
					self.retr(arg).await?;
				}
				ClientCommand::Rmd(arg) => {
					self.rmdir(arg).await?;
				}
				ClientCommand::Rnfr(arg) => {
					self.rnfr(arg).await?;
				}
				ClientCommand::Rnto(arg) => {
					self.rnto(arg).await?;
				}
				ClientCommand::Site(arg) => {
					self.site(arg).await?;
				}
				ClientCommand::Smnt(arg) => {
					self.smnt(arg).await?;
				}
				ClientCommand::Stat(arg) => {
					self.stat(arg).await?;
				}
				ClientCommand::Stor(arg) => {
					self.stor(arg).await?;
				}
				ClientCommand::Stou => {
					self.stou().await?;
				}
				ClientCommand::Stru => {
					self.stru().await?;
				}
				ClientCommand::Syst => {
					self.syst().await?;
				}
				ClientCommand::Type(arg) => {
					self.transfert_type(arg).await?;
				}
				ClientCommand::Unknown(arg) => {
					self.unknown(arg).await?;
				}
				ClientCommand::User(arg) => {
					// See connect() function
				}
			}
			msg = self.ctrl_connection.read().await;
		}
		Ok(())
	}
	
	/**
	 * Cancel the current data transfer
	 * The function is partially implemented: the function does not support receiving ABOR during a data transfer process !
	 * To do it a solution is tokio::select to transfer data and listening control socket at the same time.
	 */
	async fn abor(&mut self) -> FtpResult<()> {
		if self.data_connection.is_some() {
			self.data_connection.take().unwrap().close();
			self.data_connection = None;
		}
		let msg = format!("{} ABORD: data connection closed", ServerResponse::ClosingDataConnection.to_string());
		self.ctrl_connection.write(msg).await
	}
	
	/**
	 * Setup the account of a user
	 */
	async fn acct(&mut self, arg: String) -> FtpResult<()> {
		self.ctrl_connection.write(ServerResponse::CommandNotImplemented.to_string()).await
	}
	
	/**
	 * Books free space to save data later.
	 */
	async fn allo(&mut self, arg: u32) -> FtpResult<()> {
		self.ctrl_connection.write(ServerResponse::CommandNotImplemented.to_string()).await
	}
	
	/**
	 * Same to STOR, but if the file exists, the data are not removed.
	 */
	async fn appe(&mut self, arg: PathBuf) -> FtpResult<()> {
		self.ctrl_connection.write(ServerResponse::CommandNotImplemented.to_string()).await
	}
	
	async fn cdup(&mut self) -> FtpResult<()> {
		let path = self.current_work_directory.as_ref().unwrap().parent().unwrap().to_path_buf();
		if let Ok(_) = fs::read_dir(path.clone()) {
			return self.ctrl_connection.write(ServerResponse::RequestedFileActionOkay.to_string()).await;
		}
		return self.ctrl_connection.write(ServerResponse::InvalidParameterOrArgument.to_string()).await;
	}
	
	async fn cwd(&mut self, arg: PathBuf) -> FtpResult<()> {
		let mut message = "".to_string();
		let absolut_path = utils::get_absolut_path(&arg, &self.current_work_directory.as_ref().unwrap());
		if absolut_path.is_some() {
			let path = absolut_path.unwrap();
			if let Ok(_) = fs::read_dir(path.clone()) {
				self.current_work_directory = Some(path);
				message = format!("{} Directory successfully changed", ServerResponse::RequestedFileActionOkay.to_string());
			} else {
				message = format!("{} Failed to change directory", ServerResponse::PermissionDenied.to_string());
			}
		} else {
			error!("CWD unknown error, arg: {}", arg.to_str().unwrap());
			message = format!("{} - \"{}\" error", ServerResponse::InvalidParameterOrArgument, arg.to_str().unwrap());
		}
		self.ctrl_connection.write(message).await
	}
	
	async fn dele(&mut self, arg: PathBuf) -> FtpResult<()> {
		info!("Remove file {}", arg.to_str().unwrap());
		let mut message = "".to_string();
		if let Some(path) = utils::get_absolut_path(&arg, &self.current_work_directory.as_ref().unwrap()) {
			if let Err(e) = fs::remove_file(path.as_path()) {
				match e.kind() {
					ErrorKind::PermissionDenied => {
						message = format!("{} - \"{}\" Permission denied", ServerResponse::PermissionDenied.to_string(), path.to_str().unwrap());
					}
					_ => {
						error!("DELE unknown error: {}", e);
						message = format!("{} - \"{}\" error", ServerResponse::BadSequenceOfCommands.to_string(), path.to_str().unwrap());
					}
				}
			} else {
				message = format!("{} {}", ServerResponse::RequestedFileActionOkay.to_string(), path.to_str().unwrap());
			}
		} else {
			error!("DELE unknown error, arg: {}", arg.to_str().unwrap());
			message = format!("{} - \"{}\" error", ServerResponse::InvalidParameterOrArgument, arg.to_str().unwrap());
		}
		self.ctrl_connection.write(message).await
	}
	
	async fn help(&mut self, arg: String) -> FtpResult<()> {
		let mut message: String = "".to_string();
		message.push_str("214-The following commands are recognized.\n");
		message.push_str(" CDUP CWD DELE HELP LIST MKD PASS PASV PORT PWD QUIT RETR RMD SYST USER\n");
		message.push_str(" RNFR RNTO NOOP NLST STAT\n");
		message.push_str("214 Help OK");
		self.ctrl_connection.write(message).await
	}
	
	async fn list(&mut self, arg: PathBuf) -> FtpResult<()> {
		if self.data_connection.is_some() {
			if let Some(path) = utils::get_absolut_path(&arg, self.current_work_directory.as_ref().unwrap()) {
				let msg = format!("{} Here comes the directory listing", ServerResponse::FileStatusOk.to_string());
				self.ctrl_connection.write(msg).await?;
				
				if self.send_data(utils::get_ls(path.as_path())).await.is_ok() {
					let msg = format!("{} Directory send OK", ServerResponse::ClosingDataConnection.to_string());
					self.ctrl_connection.write(msg).await?;
				}
			}
			
			Ok(())
		} else {
			error!("Data connection not initialized");
			Err(FtpError::DataConnectionError)
		}
	}
	
	async fn mkdir(&mut self, arg: PathBuf) -> FtpResult<()> {
		info!("Create directory {}", arg.to_str().unwrap());
		let mut message = "".to_string();
		if let Some(path) = utils::get_absolut_path(&arg, &self.current_work_directory.as_ref().unwrap()) {
			if let Err(e) = fs::create_dir(path.as_path()) {
				match e.kind() {
					ErrorKind::AlreadyExists => {
						message = format!("{} - \"{}\" Directory already exists", ServerResponse::AlreadyExists.to_string(), path.to_str().unwrap());
					}
					ErrorKind::PermissionDenied => {
						message = format!("{} - \"{}\" Permission denied", ServerResponse::PermissionDenied.to_string(), path.to_str().unwrap());
					}
					_ => {
						error!("MKD unknown error: {}", e);
						message = format!("{} - \"{}\" error", ServerResponse::BadSequenceOfCommands.to_string(), path.to_str().unwrap());
					}
				}
			} else {
				message = format!("{} {}", ServerResponse::PathNameCreated.to_string(), path.to_str().unwrap());
			}
		} else {
			error!("MKD unknown error, arg: {}", arg.to_str().unwrap());
			message = format!("{} - \"{}\" error", ServerResponse::InvalidParameterOrArgument, arg.to_str().unwrap());
		}
		self.ctrl_connection.write(message).await
	}
	
	/**
	 * Set transfer mode
	 */
	async fn mode(&mut self) -> FtpResult<()> {
		self.ctrl_connection.write(ServerResponse::CommandNotImplemented.to_string()).await
	}
	
	async fn noop(&mut self) -> FtpResult<()> {
		self.ctrl_connection.write(format!("{} NOOP", ServerResponse::OK.to_string())).await
	}
	
	async fn nlst(&mut self, arg: PathBuf) -> FtpResult<()> {
		if self.data_connection.is_some() {
			if let Some(path) = utils::get_absolut_path(&arg, self.current_work_directory.as_ref().unwrap()) {
				let msg = format!("{} Here comes the directory listing", ServerResponse::FileStatusOk.to_string());
				self.ctrl_connection.write(msg).await?;
				
				if self.send_data(utils::get_nls(path.as_path(), arg.as_path().to_str().unwrap())).await.is_ok() {
					let msg = format!("{} Directory send OK", ServerResponse::ClosingDataConnection.to_string());
					self.ctrl_connection.write(msg).await?;
				}
			}
			
			Ok(())
		} else {
			error!("Data connection not initialized");
			Err(FtpError::DataConnectionError)
		}
	}
	
	async fn pasv(&mut self) -> FtpResult<()> {
		debug!("Client::pasv");
		
		self.transfert_mode = Passive;
		
		let port: u16 = pick_unused_port().expect("No ports free");
		let listener = TcpListener::bind(format!("{}:{}", ADDR, port)).await?;
		let socket_addr = listener.local_addr()?;
		info!("Server listening data on {:?}", socket_addr);
		
		let message = format!("{} {}", ServerResponse::EnteringPassiveMode.to_string(), utils::get_addr_msg(socket_addr));
		self.ctrl_connection.write(message).await?;
		
		let (stream, addr) = listener.accept().await?;
		info!("Data connection open with addr {:?}", addr);
		let (rx, tx) = stream.into_split();
		self.data_connection = Some(Connection::new(rx, tx));
		
		Ok(())
	}
	
	async fn port(&mut self, arg: String) -> FtpResult<()> {
		if let Some(addr) = utils::parse_port(arg) {
			let socket = TcpStream::connect(SocketAddr::new(addr.0, addr.1)).await?;
			let (rx, tx) = socket.into_split();
			self.data_connection = Some(Connection::new(rx, tx));
			
			let message = format!("{} PORT command successful. Consider using PASV", ServerResponse::OK.to_string());
			self.ctrl_connection.write(message).await?;
			Ok(())
		} else {
			Err(FtpError::DataConnectionError)
		}
	}
	
	async fn pwd(&mut self) -> FtpResult<()> {
		let message = format!("{} \"{}\" is the current directory", ServerResponse::PathNameCreated.to_string(), self.current_work_directory.as_ref().unwrap().to_str().unwrap());
		self.ctrl_connection.write(message).await
	}
	
	/**
	 * Restart a user connection
	 */
	async fn rein(&mut self) -> FtpResult<()> {
		self.ctrl_connection.write(ServerResponse::CommandNotImplemented.to_string()).await
	}
	
	/**
	 * Restart a data transfer process
	 */
	async fn rest(&mut self, arg: String) -> FtpResult<()> {
		self.ctrl_connection.write(ServerResponse::CommandNotImplemented.to_string()).await
	}
	
	async fn retr(&mut self, arg: PathBuf) -> FtpResult<()> {
		if self.data_connection.is_some() {
			
			if let Some(path) = utils::get_absolut_path(&arg, self.current_work_directory.as_ref().unwrap()) {
				if let Some(data) = utils::get_file(path.as_path()) {
					let msg = format!("{} Start transfer file {}", ServerResponse::FileStatusOk.to_string(), path.to_str().unwrap());
					self.ctrl_connection.write(msg).await?;
					
					if self.send_data(vec![String::from_utf8(data).unwrap()]).await.is_ok() {
						let msg = format!("{} Transfer complete", ServerResponse::ClosingDataConnection.to_string());
						self.ctrl_connection.write(msg).await?;
					}
					
					return Ok(());
				}
			}
			let msg = format!("{} Failed to open file {}", ServerResponse::PermissionDenied.to_string(), arg.to_str().unwrap());
			self.ctrl_connection.write(msg).await?;
			Ok(())
		} else {
			error!("Data connection not initialized");
			Err(FtpError::DataConnectionError)
		}
	}
	
	async fn rmdir(&mut self, arg: PathBuf) -> FtpResult<()> {
		info!("Remove directory {}", arg.to_str().unwrap());
		let mut message = "".to_string();
		if let Some(path) = utils::get_absolut_path(&arg, &self.current_work_directory.as_ref().unwrap()) {
			if let Err(e) = fs::remove_dir(path.as_path()) {
				match e.kind() {
					ErrorKind::PermissionDenied => {
						message = format!("{} - \"{}\" Permission denied", ServerResponse::PermissionDenied.to_string(), path.to_str().unwrap());
					}
					_ => {
						error!("RMDIR unknown error: {}", e);
						message = format!("{} - \"{}\" error", ServerResponse::BadSequenceOfCommands.to_string(), path.to_str().unwrap());
					}
				}
			} else {
				message = format!("{} {}", ServerResponse::RequestedFileActionOkay.to_string(), path.to_str().unwrap());
			}
		} else {
			error!("RMDIR unknown error, arg: {}", arg.to_str().unwrap());
			message = format!("{} - \"{}\" error", ServerResponse::InvalidParameterOrArgument, arg.to_str().unwrap());
		}
		self.ctrl_connection.write(message).await
	}
	
	async fn rnfr(&mut self, arg: PathBuf) -> FtpResult<()> {
		let mut message;
		if let Some(path) = utils::get_absolut_path(&arg, self.current_work_directory.as_ref().unwrap()) {
			if path.exists() {
				self.current_working_path = Some(path);
				message = format!("{} Ready for RNTO", ServerResponse::RequestedFileActionPendingFurtherInformation);
				return self.ctrl_connection.write(message).await;
			}
		}
		message = format!("{} RNFR {} command failed", ServerResponse::PermissionDenied, arg.to_str().unwrap());
		self.ctrl_connection.write(message).await
	}
	
	async fn rnto(&mut self, arg: PathBuf) -> FtpResult<()> {
		let mut message;
		if let Some(origin_path) = self.current_working_path.as_ref() {
			if let Some(working_path) = utils::get_absolut_path(&arg, self.current_work_directory.as_ref().unwrap()) {
				if fs::rename(origin_path, working_path).is_ok() {
					self.current_working_path = None;
					message = format!("{} Rename successful", ServerResponse::RequestedFileActionOkay);
					return self.ctrl_connection.write(message).await;
				}
			}
		}
		message = format!("{} RNTO {} command failed", ServerResponse::PermissionDenied, arg.to_str().unwrap());
		self.ctrl_connection.write(message).await
	}
	
	/**
	 * Specific commands for this site
	 */
	async fn site(&mut self, arg: String) -> FtpResult<()> {
		self.ctrl_connection.write(ServerResponse::CommandNotImplemented.to_string()).await
	}
	
	/**
	 * Mount a file system
	 */
	async fn smnt(&mut self, arg: PathBuf) -> FtpResult<()> {
		self.ctrl_connection.write(ServerResponse::CommandNotImplemented.to_string()).await
	}
	
	async fn stat(&mut self, arg: PathBuf) -> FtpResult<()> {
		let mut message = "".to_string();
		
		if arg.as_path().to_str().unwrap().is_empty() {
			message.push_str(format!("{} Server status \r\n", ServerResponse::SystemStatus.to_string()).as_str());
			
			message.push_str(format!("   Connected to {} \r\n", ADDR).as_str());
			message.push_str(format!("   Logged in as {} \r\n", self.user.as_ref().unwrap().name().to_str().unwrap()).as_str());
			message.push_str(format!("   Type {} \r\n", self.transfert_type).as_str());
			message.push_str("   No session bandwidth limit\r\n");
			message.push_str(format!("   Session timeout in seconds is {} \r\n", connection::TIME_OUT).as_str());
			message.push_str("   Control connection is plain text\r\n");
			message.push_str("   Data connection will be plain text\r\n");
			message.push_str(format!("   At session startup, client count was {} \r\n", self.id).as_str());
			message.push_str("   FTP server version 0.0.1\r\n");
			
			message.push_str(format!("{} End of status \r\n", ServerResponse::SystemStatus.to_string()).as_str());
		} else {
			if let Some(path) = utils::get_absolut_path(&arg, self.current_work_directory.as_ref().unwrap()) {
				message.push_str(format!("{} Status follows \r\n", ServerResponse::FileStatus.to_string()).as_str());
				
				for msg in utils::get_ls(path.as_path()) {
					message.push_str(format!("{}\r\n", msg).as_str());
				}
				
				message.push_str(format!("{} End of status", ServerResponse::FileStatus.to_string()).as_str());
			}
		}
		
		self.ctrl_connection.write(message).await
	}
	
	/**
	 * Save data in a file. The data are sent through the control socket.
	 * If the file exists, the data are removed.
	 */
	async fn stor(&mut self, arg: PathBuf) -> FtpResult<()> {
		self.ctrl_connection.write(ServerResponse::CommandNotImplemented.to_string()).await
	}
	
	/**
	 * Same to STOR, but it save the data in one unique file. The data are sent through the control socket.
	 */
	async fn stou(&mut self) -> FtpResult<()> {
		self.ctrl_connection.write(ServerResponse::CommandNotImplemented.to_string()).await
	}
	
	/**
	 * Set File Structure
	 */
	async fn stru(&mut self) -> FtpResult<()> {
		self.ctrl_connection.write(ServerResponse::CommandNotImplemented.to_string()).await
	}
	
	async fn syst(&mut self) -> FtpResult<()> {
		info!("SYST command");
		self.ctrl_connection.write(ServerResponse::SystemType.to_string()).await
	}
	
	async fn transfert_type(&mut self, arg: TransferType) -> FtpResult<()> {
		let mut message = "".to_string();
		match arg {
			TransferType::Unknown => {
				message = format!("{} Transfert type unknown", ServerResponse::InvalidParameterOrArgument.to_string());
			}
			_ => {
				self.transfert_type = arg;
				message = format!("{} Switch to {}", ServerResponse::OK.to_string(), arg.to_string());
			}
		}
		self.ctrl_connection.write(message).await
	}
	
	async fn unknown(&mut self, arg: String) -> FtpResult<()> {
		self.ctrl_connection.write(ServerResponse::CommandNotImplemented.to_string()).await
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
				data_connection.close().await;
				self.data_connection = None;
			}
			cmd = self.ctrl_connection.read() => {
				if cmd.is_some() {
					match self.parse_command(&cmd.as_ref().unwrap()) {
						ClientCommand::Abor => {
							data_connection.close().await;
							self.data_connection = None;
							let msg = format!("{} transfer interrupted by ABORD", ServerResponse::ConnectionClosed.to_string());
							self.ctrl_connection.write(msg).await?;
							let msg = format!("{} ABORD: ok", ServerResponse::ClosingDataConnection.to_string());
							self.ctrl_connection.write(msg).await?;
							return Err(FtpError::Abord);
						}
						_ => { }
					}
				}
			}
		}
		Ok(())
	}
	
	pub async fn close_connection(&mut self) {
		info!("Close client connection");
		if self.user.is_some() {
			if let Err(e) = self.ctrl_connection.write(ServerResponse::ConnectionClosed.to_string()).await {
				error!("Failed to close connection with client: {:?}", e);
			}
			self.user = None;
		}
		self.ctrl_connection.close().await;
	}
}