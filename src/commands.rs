use std::ffi::OsStr;
use std::fs::{remove_file, File};
use std::io::{Error, ErrorKind, Result};
use std::path::Path;
use std::process::{Child, Command, Stdio};

use crate::wait_and_check_status;

pub fn has_program(name: &str) -> Result<bool> {
	Ok(command_v(name)?.wait()?.success())
}

fn command_v(name: &str) -> Result<Child> {
	Ok(
		Command::new("which")
			.arg(name)
			.stdout(Stdio::null())
			.spawn()
			.expect("Couldn't start which!")
	)
}

pub fn require_program(name: &str) -> Result<()> {
	if !has_program(name)? {
		Err(
			Error::new(
				ErrorKind::NotFound,
				format!("Couldn't find program '{name}'!")
			)
		)
	} else {
		Ok(())
	}
}

pub fn download<S: AsRef<OsStr>, P: AsRef<Path>>(url: S, dest: P) -> Result<()> {
	require_program("curl")?;
	let dest: &Path = dest.as_ref();
	let mut exists: bool = dest.try_exists()?;
	if exists && dest.is_dir() {
		remove_file(dest).expect(io_expect(dest, "delete").as_str());
		exists = dest.try_exists()?;
	};
	if !exists {
		File::create(dest).expect(io_expect(dest, "create").as_str());
	};
	let mut child: Child = Command::new("curl")
		.arg("-L")
		.arg(url)
		.arg("-o")
		.arg(dest.canonicalize()?)
		.spawn()
		.expect("Couldn't start cURL!");
	wait_and_check_status!(child, "cURL");
	Ok(())
}

pub fn io_expect<P: AsRef<Path>, S: AsRef<str>>(dest: P, msg: S) -> String {
	format!(
		"Couldn't {} path '{}'!",
		msg.as_ref(),
		dest.as_ref().display()
	)
}