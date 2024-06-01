use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read, Write};
use std::path::Path;

pub enum FileAccess {
	Read,
	Write,
	ReadWrite
}

pub fn open_file(path: &str, access: FileAccess, rewrite: bool) -> Result<File, String> {
	if !file_exists(path) {
		return Err(format!("File '{}' does not exist", path));
	}

	let file = match access {
		FileAccess::Read => {
			OpenOptions::new().read(true).clone()
		}
		FileAccess::Write => {
			OpenOptions::new().write(true)
			                  .truncate(rewrite)
							  .clone()
		}
		FileAccess::ReadWrite => {
			OpenOptions::new().read(true)
			                  .write(true)
			                  .truncate(rewrite)
			                  .clone()
		}
	};

	return match file.open(path) {
		Ok(file) => { Ok(file) }
		Err(_) => { Err(format!("Could not open file '{}'", path)) }
	}
}

pub fn file_exists(path: &str) -> bool {
	return Path::new(path).exists();
}

pub fn read_file(path: &str) -> Result<String, String> {
	let file = match open_file(path, FileAccess::Read, false) {
		Ok(res) => { res }
		Err(str) => { return Err(str) }
	};

	let mut reader = BufReader::new(file);
	let mut content = String::new();
	return match reader.read_to_string(&mut content) {
		Ok(_) => Ok(content),
		Err(_) => Err(format!("Failed to read '{}'", path))
	}
}

pub fn create_file(path: &str) -> Result<File, String> {
	if file_exists(path) {
		return open_file(path, FileAccess::ReadWrite, true);
	}

	return match File::create(path) {
		Ok(file) => Ok(file),
		Err(_) => Err(format!("Failed to create file '{}'", path))
	};
}

pub fn rewrite_file(path: &str, content: String) -> Result<(), String> {
	let mut file = match open_file(path, FileAccess::Write, true) {
		Ok(res) => res,
		Err(str) => return Err(str)
	};

	return match file.write_all(content.as_ref()) {
		Ok(_) => Ok(()),
		Err(_) => Err(format!("Could not write to file {}", path))
	}
}