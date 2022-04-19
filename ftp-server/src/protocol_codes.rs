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
use std::env;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::str::FromStr;
use log::debug;
use crate::ftp_error::FtpError;

use self::ServerResponse::*;
use self::ClientCommand::*;

#[derive(Debug, Clone, Copy)]
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
	PATHNAMECreated = 257,
	UserNameOkayNeedPassword = 331,
	NeedAccountForLogin = 332,
	RequestedFileActionPendingFurtherInformation = 350,
	ServiceNotAvailable = 421, // ==> use it for timeout connection !
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
	NotLoggedIn = 530,
	NeedAccountForStoringFiles = 532,
	FileNotFound = 550,
	PageTypeUnknown = 551,
	ExceededStorageAllocation = 552,
	FileNameNotAllowed = 553,
}

impl Display for ServerResponse {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			RestartMarkerReply => { write!(f, "{} Restart ", RestartMarkerReply as i32) },
			ServiceReadInXXXMinutes => { write!(f, "{} Service ready later ", ServiceReadInXXXMinutes as i32) },
			DataConnectionAlreadyOpen => { write!(f, "{} Data connection already open ", DataConnectionAlreadyOpen as i32) },
			FileStatusOk => { write!(f, "{}", FileStatusOk as i32) },
			OK => { write!(f, "{} Ok", OK as i32) },
			CommandNotImplementedSuperfluousAtThisSite => { write!(f, "{} Command not implemented ", CommandNotImplementedSuperfluousAtThisSite as i32) },
			SystemStatus => { write!(f, "{} System status ", SystemStatus as i32) },
			DirectoryStatus => { write!(f, "{} Directory status ", DirectoryStatus as i32) },
			FileStatus => { write!(f, "{} File status ", FileStatus as i32) },
			HelpMessage => { write!(f, "{} Help ", HelpMessage as i32) },
			SystemType => { write!(f, "{} UNIX Type: L8", SystemType as i32) },
			ServiceReadyForNewUser => { write!(f, "{} Welcome to my rust ftp server. I'm waiting for your user name ", ServiceReadyForNewUser as i32) },
			ServiceClosingControlConnection => { write!(f, "{} Goodbye ", ServiceClosingControlConnection as i32) },
			DataConnectionOpen => { write!(f, "{} Data connection open ", DataConnectionOpen as i32) },
			ClosingDataConnection => { write!(f, "{}", ClosingDataConnection as i32) },
			EnteringPassiveMode => { write!(f, "{} Entering Passive Mode", EnteringPassiveMode as i32) },
			UserLoggedIn => { write!(f, "{} User logged in ", UserLoggedIn as i32) },
			RequestedFileActionOkay => { write!(f, "{} Request file action ok ", RequestedFileActionOkay as i32) },
			PATHNAMECreated => { write!(f, "{} Path created ", PATHNAMECreated as i32) },
			UserNameOkayNeedPassword => { write!(f, "{} Please specify the password ", UserNameOkayNeedPassword as i32) },
			NeedAccountForLogin => { write!(f, "{} Need account for login ", NeedAccountForLogin as i32) },
			RequestedFileActionPendingFurtherInformation => { write!(f, "{} Request further information ", RequestedFileActionPendingFurtherInformation as i32) },
			ServiceNotAvailable => { write!(f, "{} Timeout ", ServiceNotAvailable as i32) },
			CantOpenDataConnection => { write!(f, "{} Can't open data connection ", CantOpenDataConnection as i32) },
			ConnectionClosed => { write!(f, "{} Connection closed ", ConnectionClosed as i32) },
			FileBusy => { write!(f, "{} File busy ", FileBusy as i32) },
			LocalErrorInProcessing => { write!(f, "{} Local error ", LocalErrorInProcessing as i32) },
			InsufficientStorageSpace => { write!(f, "{} No space left ", InsufficientStorageSpace as i32) },
			UnknownCommand => { write!(f, "{} Unknown command ", UnknownCommand as i32) },
			InvalidParameterOrArgument => { write!(f, "{} Invalid argument ", InvalidParameterOrArgument as i32) },
			CommandNotImplemented => { write!(f, "{} Not implemented yet ", CommandNotImplemented as i32) },
			BadSequenceOfCommands => { write!(f, "{} Bad command ", BadSequenceOfCommands as i32) },
			CommandNotImplementedForThatParameter => { write!(f, "{} Not implemented for thet parameter ", CommandNotImplementedForThatParameter as i32) },
			NotLoggedIn => { write!(f, "{} Please login with USER and PASS ", NotLoggedIn as i32) },
			NeedAccountForStoringFiles => { write!(f, "{} need account for storing files ", NeedAccountForStoringFiles as i32) },
			FileNotFound => { write!(f, "{} File not found ", FileNotFound as i32) },
			PageTypeUnknown => { write!(f, "{} Page type unknown ", PageTypeUnknown as i32) },
			ExceededStorageAllocation => { write!(f, "{} Exceeded space allocated ", ExceededStorageAllocation as i32) },
			FileNameNotAllowed => { write!(f, "{} File name not allowed ", FileNameNotAllowed as i32) },
		}
	}
}

pub const ASCII: &str = "Ascii";
pub const Binary: &str = "Binary";
pub const UNKNOWN: &str = "Unknown";

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TransferType {
	Ascii,
	Binary,
	Unknown,
}

impl Display for TransferType {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			TransferType::Ascii => { write!(f, "Ascii mod") }
			TransferType::Binary => { write!(f, "Binary mod") }
			TransferType::Unknown => { write!(f, "Unknown") }
		}
	}
}

pub const AUTH: &str = "AUTH";
pub const CWD: &str = "CWD";
pub const LIST: &str = "LIST";
pub const PASS: &str = "PASS";
pub const PASV: &str = "PASV";
pub const PORT: &str = "PORT";
pub const PWD: &str = "PWD";
pub const QUIT: &str = "QUIT";
pub const RETR: &str = "RETR";
pub const STOR: &str = "STOR";
pub const SYST: &str = "SYST";
pub const TYPE: &str = "TYPE";
pub const USER: &str = "USER";
pub const CDUP: &str = "CDUP";
pub const MKD: &str = "MKD";
pub const RMD: &str = "RMD";
pub const NOOP: &str = "NOOP";
pub const UNKN: &str = "UNKN";

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum ClientCommand {
	Auth,
	Cwd(PathBuf),
	List(PathBuf),
	Mkd(PathBuf),
	NoOp,
	Port(String),
	Pass(String),
	Pasv,
	Pwd,
	Quit,
	Retr(PathBuf),
	Rmd(PathBuf),
	Stor(PathBuf),
	Syst,
	Type(TransferType),
	CdUp,
	Unknown(String),
	User(String),
}

impl ClientCommand {
	
	pub fn new(input: &str, arg: &str) -> ClientCommand {
		
		debug!("ClientCommant::new {} {}", &input, &arg);
		
		match input {
			AUTH => Auth,
			CWD => Cwd(PathBuf::from(arg.to_string())),
			LIST => List(PathBuf::from(arg.to_string())),
			PASS => Pass(arg.to_string()),
			PORT => Port(arg.to_string()),
			PWD => Pwd,
			PASV => Pasv,
			QUIT => Quit,
			RETR => Retr(PathBuf::from(arg.to_string())),
			STOR => Stor(PathBuf::from(arg.to_string())),
			SYST => Syst,
			TYPE => {
				match arg {
					"A" => Type(TransferType::Ascii),
					Binary => Type(TransferType::Binary),
					_ => Type(TransferType::Unknown),
				}
			},
			USER => User(arg.to_string()),
			CDUP => CdUp,
			MKD => Mkd(PathBuf::from(arg.to_string())),
			RMD => Rmd(PathBuf::from(arg.to_string())),
			NOOP => NoOp,
			_ => {
				dbg!("Unknown");
				Unknown(arg.to_string())
			},
		}
	}
}

impl Display for ClientCommand {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Auth => write!(f, "{}", AUTH),
			Cwd(arg) => write!(f, "{} {}", CWD, arg.as_path().to_str().unwrap()),
			List(arg) => write!(f, "{} {}", LIST, arg.as_path().to_str().unwrap()),
			Pass(arg) => write!(f, "{} xxxx", PASS),
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
		}
	}
}
