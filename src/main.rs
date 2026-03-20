mod commands;

use std::env::args;
use std::ffi::OsStr;
use std::fs::remove_file;
use std::io::{Error, ErrorKind, Result};
use std::os::unix::fs::symlink;
use std::path::PathBuf;
use std::process::{Child, Command};

use crate::commands::{download, has_program};

// I'll think about it.
#[cfg(not(windows))]
fn main() {
	for arg in args().skip(1) {
		link_bin((&arg).into()).expect(
			format!("Failed to install '{}'!", &arg).as_str()
		);
	};
}

fn link_bin(path: PathBuf) -> Result<()> {
	println!("Installing path: {}", path.display());
	let bin: PathBuf = path.join("bin");
	for entry in bin.read_dir()? {
		let file: &PathBuf = &entry?.path();
		println!("\n{}", file.display());
		if file.is_dir() {
			continue;
		}
		let name: Option<&OsStr> = file.file_name();
		if name.is_none() {
			println!("Filename was none! '{}'", file.display());
			continue;
		}
		let filename: &OsStr = name.unwrap();
		let usr_bin_path: String = format!("/usr/bin/{}", filename.display());
		let result: Result<()> = symlink(file, &usr_bin_path);
		if result.is_err() {
			let error: Error = result.unwrap_err();
			if error.kind() == ErrorKind::AlreadyExists {
				println!("Removing existing file: {}", usr_bin_path);
				remove_file(&usr_bin_path)?;
				symlink(file, &usr_bin_path)?;
			} else {
				return Err(error)
			}
		}
		update_alternatives(file, filename, usr_bin_path)?;
	};
	Ok(())
}

fn update_alternatives(file: &PathBuf, filename: &OsStr, usr_bin_path: String) -> Result<()> {
	if !has_program("update-alternatives")? {
		return Ok(())
	}
	let mut child: Child = Command::new("update-alternatives")
		.arg("--install")
		.arg(usr_bin_path)
		.arg(filename)
		.arg(file)
		.arg("4000")
		.spawn()?;
	child.wait()?;
	Ok(())
}