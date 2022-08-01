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

extern crate num;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::str::FromStr;
use log::{debug, error};
use regex::Regex;

use self::ServerResponse::*;
use self::ClientCommand::*;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
#[allow(dead_code)]
pub enum ServerResponse {
	RestartMarkerReply = 110,
	ServiceReadInXXXMinutes = 120,
	DataConnectionAlreadyOpen = 125,
	FileStatusOk = 150,
	OK = 200,
	CommandNotImplementedSuperfluousAtThisSite = 202,
	SystemStatus = 211,
	DirectoryStatus = 212,
	FileStatus = 213,
	HelpMessage = 214,
	SystemType = 215,
	ServiceReadyForNewUser = 220,
	ServiceClosingControlConnection = 221,
	DataConnectionOpen = 225,
	ClosingDataConnection = 226,
	EnteringPassiveMode = 227,
	UserLoggedIn = 230,
	RequestedFileActionOkay = 250,
	PathNameCreated = 257,
	UserNameOkayNeedPassword = 331,
	NeedAccountForLogin = 332,
	RequestedFileActionPendingFurtherInformation = 350,
	ServiceNotAvailable = 421,
	// ==> use it for timeout connection !
	CantOpenDataConnection = 425,
	ConnectionClosed = 426,
	FileBusy = 450,
	LocalErrorInProcessing = 451,
	InsufficientStorageSpace = 452,
	UnknownCommand = 500,
	InvalidParameterOrArgument = 501,
	CommandNotImplemented = 502,
	BadSequenceOfCommands = 503,
	CommandNotImplementedForThatParameter = 504,
	AlreadyExists = 521,
	NotLoggedIn = 530,
	NeedAccountForStoringFiles = 532,
	PermissionDenied = 550,
	PageTypeUnknown = 551,
	ExceededStorageAllocation = 552,
	FileNameNotAllowed = 553,
}

impl Display for ServerResponse {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			RestartMarkerReply => { write!(f, "{} Restart ", RestartMarkerReply as i32) }
			ServiceReadInXXXMinutes => { write!(f, "{} Service ready later ", ServiceReadInXXXMinutes as i32) }
			DataConnectionAlreadyOpen => { write!(f, "{} Data connection already open ", DataConnectionAlreadyOpen as i32) }
			FileStatusOk => { write!(f, "{}", FileStatusOk as i32) }
			OK => { write!(f, "{} Ok", OK as i32) }
			CommandNotImplementedSuperfluousAtThisSite => { write!(f, "{} Command not implemented ", CommandNotImplementedSuperfluousAtThisSite as i32) }
			SystemStatus => { write!(f, "{}", SystemStatus as i32) }
			DirectoryStatus => { write!(f, "{} Directory status ", DirectoryStatus as i32) }
			FileStatus => { write!(f, "{}", FileStatus as i32) }
			HelpMessage => { write!(f, "{} Help ", HelpMessage as i32) }
			SystemType => { write!(f, "{} UNIX Type: L8", SystemType as i32) }
			ServiceReadyForNewUser => { write!(f, "{} Welcome to my rust ftp ftp_server. I'm waiting for your user name ", ServiceReadyForNewUser as i32) }
			ServiceClosingControlConnection => { write!(f, "{} Goodbye ", ServiceClosingControlConnection as i32) }
			DataConnectionOpen => { write!(f, "{} Data connection open ", DataConnectionOpen as i32) }
			ClosingDataConnection => { write!(f, "{}", ClosingDataConnection as i32) }
			EnteringPassiveMode => { write!(f, "{} Entering Passive Mode", EnteringPassiveMode as i32) }
			UserLoggedIn => { write!(f, "{} User logged in ", UserLoggedIn as i32) }
			RequestedFileActionOkay => { write!(f, "{}", RequestedFileActionOkay as i32) }
			PathNameCreated => { write!(f, "{}", PathNameCreated as i32) }
			UserNameOkayNeedPassword => { write!(f, "{} Please specify the password ", UserNameOkayNeedPassword as i32) }
			NeedAccountForLogin => { write!(f, "{} Need account for login ", NeedAccountForLogin as i32) }
			RequestedFileActionPendingFurtherInformation => { write!(f, "{}", RequestedFileActionPendingFurtherInformation as i32) }
			ServiceNotAvailable => { write!(f, "{} Timeout ", ServiceNotAvailable as i32) }
			CantOpenDataConnection => { write!(f, "{} Can't open data connection ", CantOpenDataConnection as i32) }
			ConnectionClosed => { write!(f, "{} Connection closed", ConnectionClosed as i32) }
			FileBusy => { write!(f, "{} File busy ", FileBusy as i32) }
			LocalErrorInProcessing => { write!(f, "{} Local error ", LocalErrorInProcessing as i32) }
			InsufficientStorageSpace => { write!(f, "{} No space left ", InsufficientStorageSpace as i32) }
			UnknownCommand => { write!(f, "{} Unknown command ", UnknownCommand as i32) }
			InvalidParameterOrArgument => { write!(f, "{} Invalid argument ", InvalidParameterOrArgument as i32) }
			CommandNotImplemented => { write!(f, "{} Not implemented yet ", CommandNotImplemented as i32) }
			BadSequenceOfCommands => { write!(f, "{} Bad command ", BadSequenceOfCommands as i32) }
			CommandNotImplementedForThatParameter => { write!(f, "{} Not implemented for thet parameter ", CommandNotImplementedForThatParameter as i32) }
			NotLoggedIn => { write!(f, "{} Please login with USER and PASS ", NotLoggedIn as i32) }
			NeedAccountForStoringFiles => { write!(f, "{} need account for storing files ", NeedAccountForStoringFiles as i32) }
			PermissionDenied => { write!(f, "{}", PermissionDenied as i32) }
			PageTypeUnknown => { write!(f, "{} Page type unknown ", PageTypeUnknown as i32) }
			ExceededStorageAllocation => { write!(f, "{} Exceeded space allocated ", ExceededStorageAllocation as i32) }
			FileNameNotAllowed => { write!(f, "{} File name not allowed ", FileNameNotAllowed as i32) }
			AlreadyExists => { write!(f, "{}", AlreadyExists as i32) }
		}
	}
}

pub fn parse_server_response(msg: &String) -> (ServerResponse, String) {
	if let Some(re) = Regex::new(r"^([[:digit:]]{3})( .+)*$").ok() {
		if let Some(cap) = re.captures(msg.as_str()) {
			if let Some(cmd) = cap.get(1) {
				let num = i32::from_str(cmd.as_str()).unwrap();
				let response = match num {
					110 => RestartMarkerReply,
					120 => ServiceReadInXXXMinutes,
					125 => DataConnectionAlreadyOpen,
					150 => FileStatusOk,
					200 => OK,
					202 => CommandNotImplementedSuperfluousAtThisSite,
					211 => SystemStatus,
					212 => DirectoryStatus,
					213 => FileStatus,
					214 => HelpMessage,
					215 => SystemType,
					220 => ServiceReadyForNewUser,
					221 => ServiceClosingControlConnection,
					225 => DataConnectionOpen,
					226 => ClosingDataConnection,
					227 => EnteringPassiveMode,
					230 => UserLoggedIn,
					250 => RequestedFileActionOkay,
					257 => PathNameCreated,
					331 => UserNameOkayNeedPassword,
					332 => NeedAccountForLogin,
					350 => RequestedFileActionPendingFurtherInformation,
					421 => ServiceNotAvailable,
					425 => CantOpenDataConnection,
					426 => ConnectionClosed,
					450 => FileBusy,
					451 => LocalErrorInProcessing,
					452 => InsufficientStorageSpace,
					501 => InvalidParameterOrArgument,
					502 => CommandNotImplemented,
					503 => BadSequenceOfCommands,
					504 => CommandNotImplementedForThatParameter,
					521 => AlreadyExists,
					530 => NotLoggedIn,
					532 => NeedAccountForStoringFiles,
					550 => PermissionDenied,
					551 => PageTypeUnknown,
					552 => ExceededStorageAllocation,
					553 => FileNameNotAllowed,
					_ => UnknownCommand,
				};
				if let Some(args) = cap.get(2) {
					return (response, args.as_str().to_string());
				} else {
					return (response, "".to_string());
				}
			}
		}
	}
	error!("failed to parse response: {}", msg);
	(UnknownCommand, String::new())
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TransferType {
	Ascii,
	Binary,
	Unknown,
}

impl Display for TransferType {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			TransferType::Ascii => { write!(f, "TYPE A") }
			TransferType::Binary => { write!(f, "TYPE I") }
			_ => { write!(f, "Unknown transfert type") }
		}
	}
}

#[derive(PartialEq)]
pub enum TransfertMode {
	Passive,
	Active,
}

impl Display for TransfertMode {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			TransfertMode::Passive => { write!(f, "Passive mode") }
			TransfertMode::Active => { write!(f, "Active mode") }
		}
	}
}

pub const ABOR: &str = "ABOR";
pub const ALLO: &str = "ALLO";
pub const APPE: &str = "APPE";
pub const _AUTH: &str = "AUTH";
pub const ACCT: &str = "ACCT";
pub const CDUP: &str = "CDUP";
pub const CWD: &str = "CWD";
pub const DELE: &str = "DELE";
pub const HELP: &str = "HELP";
pub const LIST: &str = "LIST";
pub const MKD: &str = "MKD";
pub const MODE: &str = "MODE";
pub const NLIST: &str = "NLST";
pub const NOOP: &str = "NOOP";
pub const PASS: &str = "PASS";
pub const PASV: &str = "PASV";
pub const PORT: &str = "PORT";
pub const PWD: &str = "PWD";
pub const QUIT: &str = "QUIT";
pub const REIN: &str = "REIN";
pub const REST: &str = "REST";
pub const RETR: &str = "RETR";
pub const RMD: &str = "RMD";
pub const RNFR: &str = "RNFR";
pub const RNTO: &str = "RNTO";
pub const SITE: &str = "SITE";
pub const SMNT: &str = "SMNT";
pub const STAT: &str = "STAT";
pub const STOR: &str = "STOR";
pub const STOU: &str = "STOU";
pub const STRU: &str = "STRU";
pub const SYST: &str = "SYST";
pub const TYPE: &str = "TYPE";
pub const USER: &str = "USER";
pub const UNKN: &str = "UNKN";

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum ClientCommand {
	Abor,
	Allo(u32),
	Appe(PathBuf),
	Acct(String),
	CdUp,
	Cwd(PathBuf),
	Dele(PathBuf),
	Help(String),
	List(Option<PathBuf>),
	Mkd(PathBuf),
	Mode,
	Nlist(Option<PathBuf>),
	NoOp,
	Pass(String),
	Pasv,
	Port(String),
	Pwd,
	Quit,
	Rein,
	Rest(String),
	Retr(PathBuf),
	Rmd(PathBuf),
	Rnto(PathBuf),
	Rnfr(PathBuf),
	Site(String),
	Smnt(PathBuf),
	Stat(PathBuf),
	Stor(PathBuf),
	Stou(PathBuf),
	Stru,
	Syst,
	Type(TransferType),
	Unknown(String),
	User(String),
}

pub fn parse_client_command(msg: &String) -> ClientCommand {
	debug!("protocol::parse_client_command '{}'", msg);
	if let Some(re) = Regex::new(r"^([[:digit:]]{3,4})( .+)*$").ok() {
		if let Some(cap) = re.captures(msg.as_str()) {
			if let Some(cmd) = cap.get(1) {
				if let Some(args) = cap.get(2) {
					return ClientCommand::new_with_args(cmd.as_str(), args.as_str().to_string().trim());
				} else {
					return ClientCommand::new_without_args(cmd.as_str());
				}
			}
		}
	}
	error!("failed to parse command: {}", msg);
	ClientCommand::Unknown(msg.clone())
}

impl ClientCommand {
	pub fn new_with_args(input: &str, arg: &str) -> ClientCommand {
		debug!("ClientCommant::new {} {}", &input, &arg);
		
		match input {
			ALLO => Allo(arg.to_string().parse::<u32>().unwrap()),
			APPE => Appe(PathBuf::from(arg.to_string())),
			ACCT => Acct(arg.to_string()),
			CWD => Cwd(PathBuf::from(arg.to_string())),
			DELE => Dele(PathBuf::from(arg.to_string())),
			HELP => Help(arg.to_string()),
			LIST => List(Some(PathBuf::from(arg.to_string()))),
			MKD => Mkd(PathBuf::from(arg.to_string())),
			NLIST => Nlist(Some(PathBuf::from(arg.to_string()))),
			PASS => Pass(arg.to_string()),
			PORT => Port(arg.to_string()),
			REST => Rest(arg.to_string()),
			RETR => Retr(PathBuf::from(arg.to_string())),
			RMD => Rmd(PathBuf::from(arg.to_string())),
			RNFR => Rnfr(PathBuf::from(arg.to_string())),
			RNTO => Rnto(PathBuf::from(arg.to_string())),
			SITE => Site(arg.to_string()),
			SMNT => Smnt(PathBuf::from(arg.to_string())),
			STAT => Stat(PathBuf::from(arg.to_string())),
			STOR => Stor(PathBuf::from(arg.to_string())),
			STOU => Stou(PathBuf::from(arg.to_string())),
			TYPE => {
				match arg {
					"A" => Type(TransferType::Ascii),
					"I" => Type(TransferType::Binary),
					_ => Type(TransferType::Unknown),
				}
			}
			USER => User(arg.to_string()),
			_ => {
				Unknown("Unknown command".to_string())
			}
		}
	}

	pub fn new_without_args(input: &str) -> ClientCommand {
		debug!("ClientCommant::new {}", &input);

		match input {
			ABOR => Abor,
			CDUP => CdUp,
			MODE => Mode,
			NOOP => NoOp,
			PWD => Pwd,
			PASV => Pasv,
			QUIT => Quit,
			REIN => Rein,
			STRU => Stru,
			SYST => Syst,
			LIST => List(None),
			NLIST => Nlist(None),
			_ => {
				Unknown("Unknown command".to_string())
			}
		}
	}
}

impl Display for ClientCommand {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Cwd(arg) => write!(f, "{} {}", CWD, arg.as_path().to_str().unwrap()),
			List(arg) => {
				if let Some(path) = arg {
					write!(f, "{} {}", LIST, path.as_path().to_str().unwrap())
				} else {
					write!(f, "{}", LIST)
				}
			},
			Pass(arg) => write!(f, "{} {}", PASS, arg),
			Port(arg) => write!(f, "{} {}", PORT, arg),
			Pwd => write!(f, "{}", PWD),
			Pasv => write!(f, "{}", PASV),
			Quit => write!(f, "{}", QUIT),
			Retr(arg) => write!(f, "{} {}", RETR, arg.as_path().to_str().unwrap()),
			Stor(arg) => write!(f, "{} {}", STOR, arg.as_path().to_str().unwrap()),
			Syst => write!(f, "{}", SYST),
			Type(arg) => write!(f, "{} {}", TYPE, arg),
			User(arg) => write!(f, "{} {}", USER, arg),
			CdUp => write!(f, "{}", CDUP),
			Mkd(arg) => write!(f, "{} {}", MKD, arg.as_path().to_str().unwrap()),
			Rmd(arg) => write!(f, "{} {}", RMD, arg.as_path().to_str().unwrap()),
			NoOp => write!(f, "{}", NOOP),
			Unknown(arg) => write!(f, "{} {}", UNKN, arg), // doesn't exist in the protocol
			Abor => write!(f, "{}", ABOR),
			Allo(arg) => write!(f, "{} {}", ALLO, arg),
			Appe(arg) => write!(f, "{} {}", APPE, arg.as_path().to_str().unwrap()),
			Acct(arg) => write!(f, "{} {}", ACCT, arg),
			Dele(arg) => write!(f, "{} {}", DELE, arg.as_path().to_str().unwrap()),
			Help(arg) => write!(f, "{} {}", HELP, arg),
			Mode => write!(f, "{}", MODE),
			Nlist(arg) => {
				if let Some(path) = arg {
					write!(f, "{} {}", NLIST, path.as_path().to_str().unwrap())
				} else {
					write!(f, "{}", NLIST)
				}
			},
			Rein => write!(f, "{}", REIN),
			Rest(arg) => write!(f, "{} {}", REST, arg),
			Rnto(arg) => write!(f, "{} {}", RNTO, arg.as_path().to_str().unwrap()),
			Rnfr(arg) => write!(f, "{} {}", RNFR, arg.as_path().to_str().unwrap()),
			Site(arg) => write!(f, "{} {}", SITE, arg),
			Smnt(arg) => write!(f, "{} {}", SMNT, arg.as_path().to_str().unwrap()),
			Stat(arg) => write!(f, "{} {}", STAT, arg.as_path().to_str().unwrap()),
			Stou(arg) => write!(f, "{} {}", STOU, arg.as_path().to_str().unwrap()),
			Stru => write!(f, "{}", STRU),
		}
	}
}