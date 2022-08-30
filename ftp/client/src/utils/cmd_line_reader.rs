use rustyline::error::ReadlineError;
use rustyline::{Editor};
use crate::utils::error::{FtpError, FtpResult};

pub struct CmdLineReader {
	reader: Editor<()>,
}

impl CmdLineReader {
	pub fn new() -> FtpResult<CmdLineReader> {
		if let Ok(rd) = Editor::<()>::new() {
			return Ok(CmdLineReader {
				reader: rd,
			});
		}
		return Err(FtpError::InternalError("Impossible to create reader".to_string()));
	}
	
	/*
	 * The function read line and save the history
	 */
	pub fn read_line(&mut self, prompt: &str) -> FtpResult<String> {
		let readline = self.reader.readline(prompt);
		match readline {
			Ok(line) => {
				self.reader.add_history_entry(line.as_str());
				return Ok(line);
			}
			Err(ReadlineError::Interrupted) => {
				return Err(FtpError::InternalError("CTRL-C".to_string()));
			}
			Err(ReadlineError::Eof) => {
				return Err(FtpError::InternalError("CTRL-D".to_string()));
			}
			Err(err) => {
				return Err(FtpError::InternalError(err.to_string()));
			}
		}
	}
	
	/*
	 * The function read line without saving the history
	 */
	pub fn get_user_name(&mut self) -> FtpResult<String> {
		let readline = self.reader.readline("Name: ");
		match readline {
			Ok(line) => {
				return Ok(line);
			}
			Err(ReadlineError::Interrupted) => {
				return Err(FtpError::InternalError("CTRL-C".to_string()));
			}
			Err(ReadlineError::Eof) => {
				return Err(FtpError::InternalError("CTRL-D".to_string()));
			}
			Err(err) => {
				return Err(FtpError::InternalError(err.to_string()));
			}
		}
	}
	
	pub async fn get_two_args(&mut self, arg: Option<String>, prompt_1: &str, prompt_2: &str) -> FtpResult<(String, String)> {
		let mut arg_1: String = "".to_string();
		let mut arg_2: String = "".to_string();
		
		if let Some(args) = arg {
			let mut split: Vec<&str> = args.split(" ").collect();
			match split.len() {
				1 => {
					arg_1 = split.get(0).unwrap().to_string();
					arg_2 = arg_1.to_string().clone();
					return Ok((arg_1, arg_2));
				}
				2 => {
					arg_1 = split.get(0).unwrap().to_string();
					arg_2 = split.get(1).unwrap().to_string();
					return Ok((arg_1, arg_2));
				}
				_ => {}
			}
		}
		
		if let Ok(msg) = self.read_line(prompt_1) {
			arg_1 = msg.trim().to_string();
			if let Ok(msg) = self.read_line(prompt_2) {
				arg_2 = msg.trim().to_string();
				return Ok((arg_1, arg_2));
			}
		}
		
		return Err(FtpError::InternalError("Impossible to get args".to_string()));
	}
	
	pub async fn get_one_arg(&mut self, arg: Option<String>, prompt: &str) -> FtpResult<String> {
		if let Some(args) = arg {
			let mut split: Vec<&str> = args.split(" ").collect();
			if split.len() >= 1 {
				return Ok(split.get(0).unwrap().to_string());
			}
		} else {
			if let Ok(msg) = self.read_line(prompt) {
				return Ok(msg.trim().to_string());
			}
		}
		
		return Err(FtpError::InternalError("Impossible to get arg".to_string()));
	}
}