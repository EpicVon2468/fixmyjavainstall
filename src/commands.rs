use std::ffi::OsStr;
use std::fs::{remove_file, File};
use std::io::{Error, ErrorKind, Result};
use std::path::Path;
use std::process::{Child, Command};

pub fn has_program(name: &str) -> Result<bool> {
	Ok(command_v(name)?.wait()?.success())
}

fn command_v(name: &str) -> Result<Child> {
	Ok(
		Command::new("command")
			.arg("-v")
			.arg(name)
			.spawn()?
	)
}

pub fn require_program(name: &str) -> Result<()> {
	if !has_program(name)? {
		Err(
			Error::new(
				ErrorKind::NotFound,
				format!("Couldn't find program '{}'!", name)
			)
		)
	} else {
		Ok(())
	}
}

pub fn download<S: AsRef<OsStr>, P: AsRef<Path>>(url: S, dest: P) -> Result<()> {
	require_program("curl")?;
	let file: &Path = dest.as_ref();
	let mut exists: bool = file.try_exists()?;
	if exists && file.is_dir() {
		remove_file(file)?;
		exists = file.try_exists()?;
	};
	if !exists {
		File::create(file)?;
	};
	let mut child: Child = Command::new("curl")
		.arg("-L")
		.arg(url)
		.arg("-o")
		.arg(file.canonicalize()?)
		.spawn()?;
	child.wait()?;
	Ok(())
}