use std::io::Result;
use std::process::{Child, Command};

pub fn has_program(name: &str) -> Result<bool> {
	Ok(
		command_v(name)?
			.wait_with_output()?
			.status
			.success()
	)
}

fn command_v(name: &str) -> Result<Child> {
	Ok(
		Command::new("command")
			.arg("-v")
			.arg(name)
			.spawn()?
	)
}