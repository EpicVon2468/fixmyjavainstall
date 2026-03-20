use std::ffi::OsStr;
use std::fs::File;
use std::io::Result;
use std::path::Path;
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

pub fn download<S: AsRef<OsStr>, P: AsRef<Path>>(url: S, file: P) -> Result<()> {
	let path: &Path = file.as_ref();
	if !path.exists() {
		File::create(&file)?;
	}
	let mut child: Child = Command::new("curl")
		.arg("-L")
		.arg(url)
		.arg("-o")
		.arg(path.canonicalize()?)
		.spawn()?;
	child.wait()?;
	Ok(())
}