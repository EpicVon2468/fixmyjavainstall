use std::ffi::OsStr;
use std::fs::{remove_file, File};
use std::io::{Error, ErrorKind, Result};
use std::path::Path;
use std::process::{Child, Command, ExitStatus, Output, Stdio};

use crate::{check_status, wait_and_check_status};

pub fn has_program(name: &str) -> Result<bool> {
	Ok(command_v(name)?.wait()?.success())
}

fn command_v(name: &str) -> Result<Child> {
	#[cfg(unix)]
	let command_v: &str = "which";
	#[cfg(windows)]
	let command_v: &str = "where.exe";
	Ok(
		Command::new(command_v)
			.arg(name)
			.stdout(Stdio::piped())
			.spawn()
			.expect(&format!("Couldn't start {command_v}!"))
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

// https://stackoverflow.com/questions/845593/how-do-i-untar-a-subdirectory-into-the-current-directory
// sudo tar --strip-components 1 -xvf 25.0.2.tar.gz -C 25.0.2
// macOS & Linux both have tar, but not Windows
pub fn untar_jdk<S: AsRef<OsStr>, P: AsRef<Path>>(archive: S, dest: P) -> Result<()> {
	require_program("tar")?;
	let mut child: Child = Command::new("tar")
		.arg("--strip-components")
		.arg("1")
		.arg("-xvf")
		.arg(archive)
		.arg("-C")
		.arg(dest.as_ref().canonicalize()?)
		.stdout(Stdio::null())
		.spawn()
		.expect("Couldn't start tar!");
	wait_and_check_status!(child, "tar");
	Ok(())
}

// TODO: https://curl.se/windows/ && https://curl.se/windows/latest.cgi?p=win64-mingw.zip

pub fn download<S: AsRef<OsStr>, P: AsRef<Path>>(url: S, dest: P) -> Result<()> {
	require_program("curl")?;
	let dest: &Path = dest.as_ref();
	let mut exists: bool = dest.try_exists()?;
	if exists && dest.is_dir() {
		remove_file(dest).expect(&io_expect(dest, "delete"));
		exists = dest.try_exists()?;
	};
	if !exists {
		File::create(dest).expect(&io_expect(dest, "create"));
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

pub fn connect<S: AsRef<OsStr>>(url: S) -> Result<String> {
	require_program("curl")?;
	let child: Child = Command::new("curl")
		.arg("--silent")
		.arg("-L")
		.arg(url)
		.stdout(Stdio::piped())
		.spawn()
		.expect("Couldn't start cURL!");
	let output: Output = child.wait_with_output().expect("cURL never started?");
	let status: ExitStatus = output.status;
	check_status!(status, "cURL");
	Ok(String::from_utf8(output.stdout).unwrap())
}

pub fn io_expect<P: AsRef<Path>, S: AsRef<str>>(dest: P, msg: S) -> String {
	format!(
		"Couldn't {} path '{}'!",
		msg.as_ref(),
		dest.as_ref().display()
	)
}