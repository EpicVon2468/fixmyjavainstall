mod commands;

use std::env;
use std::env::args;
use std::ffi::OsStr;
use std::fs::remove_file;
use std::io::{Error, ErrorKind, Result};
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};
use std::process::{Child, Command};

use crate::commands::{download, has_program};

// I'll think about it.
#[cfg(not(windows))]
fn main() {
	// Doing this when trying to run the binary didn't work
	unsafe {
		env::set_var("RUST_BACKTRACE", "1");
	}
	for arg in args().skip(1) {
		link_bin((&arg).into()).expect(
			format!("Failed to install '{}'!", &arg).as_str()
		);
	};
}

fn link_bin(path: PathBuf) -> Result<()> {
	println!("Installing path: {}", path.display());
	let bin: PathBuf = path.join("bin");
	for entry in bin.read_dir().expect("Couldn't read directory!") {
		let file: &PathBuf = &entry?.path();
		println!("\n{}", file.display());
		if file.is_dir() {
			continue;
		};
		let name: Option<&OsStr> = file.file_name();
		if name.is_none() {
			println!("Filename was none! '{}'", file.display());
			continue;
		};
		let filename: &OsStr = name.unwrap();
		let usr_bin_path: String = format!("/usr/bin/{}", filename.display());
		let result: Result<()> = symlink(file, &usr_bin_path);
		if result.is_err() {
			let error: Error = result.unwrap_err();
			if error.kind() == ErrorKind::AlreadyExists {
				println!("Removing existing file: {}", usr_bin_path);
				remove_file(&usr_bin_path).expect("Couldn't remove old file from /usr/bin!");
				symlink(file, &usr_bin_path).expect("Symbolic linking failed second time, panicking!");
			} else {
				return Err(error);
			};
		};
		debian_install(file, filename, usr_bin_path).expect("Couldn't install with update-alternatives!");
	};
	Ok(())
}

fn debian_install<P, S, S2>(file: P, filename: S, usr_bin_path: S2) -> Result<()>
where
	P: AsRef<Path>,
	S: AsRef<OsStr>,
	S2: AsRef<OsStr>
{
	if !has_program("update-alternatives")? {
		return Ok(());
	};
	let mut install_child: Child = Command::new("update-alternatives")
		.arg("--install")
		.arg(usr_bin_path)
		.arg(&filename)
		.arg(file.as_ref())
		.arg("4000")
		.spawn().expect("Couldn't start update-alternatives!");
	install_child.wait().expect("update-alternatives never started?");
	let mut set_child: Child = Command::new("update-alternatives")
		.arg("--set")
		.arg(&filename)
		.arg(file.as_ref())
		.spawn().expect("Couldn't start update-alternatives!");
	set_child.wait().expect("update-alternatives never started?");
	Ok(())
}