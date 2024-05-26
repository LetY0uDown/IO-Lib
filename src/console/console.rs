use std::io::{stdin, stdout, Write};
use num_traits::{Num};

pub fn cls() {
	clearscreen::clear().expect("failed to clear screen");
}

pub fn print(msg: String) -> std::io::Result<()> {
	print!("{msg}");
	return stdout().flush();
}

pub fn read_string() -> Result<String, String> {
	let mut input = String::new();

	match stdin().read_line(&mut input) {
		Ok(_)  => { Ok(input.trim().to_string()) }
		Err(_) => { Err("Failed to read from console".to_string()) }
	}
}

pub fn read_number<T: Num>() -> Result<T, String> {
	let input = match read_string() {
		Ok(res) => { res }
		Err(err) => { return Err(err) }
	};

	return match Num::from_str_radix(&*input, 10) {
		Ok(num) => Ok(num),
		Err(_) => Err("Failed to parse your input into a number".to_string())
	}
}