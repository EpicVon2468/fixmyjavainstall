mod commands;

use std::env;
use std::env::args;
use std::ffi::os_str::Display;
use std::ffi::OsStr;
use std::fs::remove_file;
use std::io::{Error, ErrorKind, Result};
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};
use std::process::{Child, Command};

use crate::commands::{download, has_program, io_expect};

// I'll think about it.
#[cfg(not(windows))]
fn main() {
	// Doing this when trying to run the binary didn't work
	unsafe {
		env::set_var("RUST_BACKTRACE", "1");
	}
	for arg in args().skip(1) {
		link_bin(&arg).expect(
			format!("Failed to install '{}'!", &arg).as_str()
		);
	};
}

fn link_bin<P: AsRef<Path>>(path: P) -> Result<()> {
	let path: &Path = path.as_ref();
	println!("Installing path: {}", path.display());
	let bin: PathBuf = path.join("bin");
	let use_update_alternatives: bool = has_program("update-alternatives")?;
	for entry in bin.read_dir().expect(io_expect(bin, "list directory").as_str()) {
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
		if use_update_alternatives {
			debian_install(file, filename, usr_bin_path).expect("Couldn't install with update-alternatives!");
		} else {
			symlink_install(file, &usr_bin_path).expect("Couldn't install with symlink!");
		};
	};
	Ok(())
}

fn symlink_install<P: AsRef<Path>, S: AsRef<OsStr>>(source: P, dest: S) -> Result<()> {
	let source: &Path = source.as_ref();
	let dest: &OsStr = dest.as_ref();
	let result: Result<()> = symlink(source, dest);
	if result.is_err() {
		let error: Error = result.unwrap_err();
		if error.kind() == ErrorKind::AlreadyExists {
			let display: Display = dest.display();
			println!("Removing existing file: {}", display);
			remove_file(dest).expect(io_expect(dest, "remove").as_str());
			symlink(source, dest).expect("Symbolic linking failed second time, panicking!");
		} else {
			return Err(error);
		};
	};
	Ok(())
}

fn debian_install<P, S, S2>(file: P, filename: S, usr_bin_path: S2) -> Result<()>
where
	P: AsRef<Path>,
	S: AsRef<OsStr>,
	S2: AsRef<OsStr>
{
	let file: &Path = file.as_ref();
	let mut install_child: Child = Command::new("update-alternatives")
		.arg("--install")
		.arg(usr_bin_path)
		.arg(&filename)
		.arg(file)
		.arg("4000")
		.spawn().expect("Couldn't start update-alternatives!");
	install_child.wait().expect("update-alternatives never started?");
	let mut set_child: Child = Command::new("update-alternatives")
		.arg("--set")
		.arg(&filename)
		.arg(file)
		.spawn().expect("Couldn't start update-alternatives!");
	set_child.wait().expect("update-alternatives never started?");
	Ok(())
}