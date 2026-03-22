mod commands;
mod cli;

use std::env::set_var;
use std::ffi::OsStr;
use std::fs::remove_file;
use std::io::{Error, ErrorKind, Result};
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitStatus};

use clap::Parser;

use crate::cli::{Arguments, Cmd};
use crate::commands::{has_program, io_expect};

// I'll think about it.
#[cfg(not(windows))]
fn main() {
	// Doing this when trying to run the binary didn't work
	unsafe {
		set_var("RUST_BACKTRACE", "1");
	};
	let arguments: Arguments = Arguments::parse();
	if let Some(command) = &arguments.command {
		match command {
			Cmd::Install { .. } => {
				do_install(command).unwrap();
			},
			Cmd::Foo { .. } => todo!(),
		};
	};
}

fn do_install(command: &Cmd) -> Result<()> {
	let Cmd::Install {
		paths,
		link_dir,
		use_update_alternatives,
	} = command else {
		return Err(
			Error::new(
				ErrorKind::InvalidInput,
				"Function do_install() had wrong parameter!"
			)
		);
	};
	for path in paths {
		install(path, link_dir, *use_update_alternatives).expect(
			format!("Failed to install '{path}'!").as_str()
		);
	};
	Ok(())
}

fn install<P: AsRef<Path>, S: AsRef<str>>(path: P, link_dir: S, use_update_alternatives: bool) -> Result<()> {
	let path: &Path = path.as_ref();
	println!("Installing path: {}", path.display());
	let bin: PathBuf = path.join("bin");
	let can_use_update_alternatives: bool = use_update_alternatives && has_program("update-alternatives")?;
	if !can_use_update_alternatives && use_update_alternatives {
		println!("Couldn't find update-alternatives on system when explicitly requested!");
		return Err(
			Error::new(
				ErrorKind::NotFound,
				"Couldn't find update-alternatives on system when explicitly requested!"
			)
		);
	};
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
		let dest: String = format!("{}/{}", link_dir.as_ref(), filename.display());
		if can_use_update_alternatives {
			debian_install(file, filename, dest).expect("Couldn't install with update-alternatives!");
		} else {
			symlink_install(file, dest).expect("Couldn't install with symlink!");
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
			println!("Removing existing file: {}", dest.display());
			remove_file(dest).expect(io_expect(dest, "remove").as_str());
			symlink(source, dest).expect("Symbolic linking failed second time, panicking!");
		} else {
			return Err(error);
		};
	};
	Ok(())
}

fn debian_install<P, S, S2>(file: P, filename: S, dest: S2) -> Result<()>
where
	P: AsRef<Path>,
	S: AsRef<OsStr>,
	S2: AsRef<OsStr>
{
	let file: &Path = file.as_ref();
	let filename: &OsStr = filename.as_ref();
	let mut install_child: Child = Command::new("update-alternatives")
		.arg("--install")
		.arg(dest)
		.arg(filename)
		.arg(file)
		.arg("4000")
		.spawn()
		.expect("Couldn't start update-alternatives!");
	let install_status: ExitStatus = install_child.wait().expect("update-alternatives never started?");
	if let Some(error) = check_status(install_status) {
		return Err(error);
	};
	let mut set_child: Child = Command::new("update-alternatives")
		.arg("--set")
		.arg(filename)
		.arg(file)
		.spawn()
		.expect("Couldn't start update-alternatives!");
	let set_status: ExitStatus = set_child.wait().expect("update-alternatives never started?");
	if let Some(error) = check_status(set_status) {
		return Err(error);
	};
	Ok(())
}

fn check_status(status: ExitStatus) -> Option<Error> {
	if !status.success() {
		Some(
			Error::other(
				format!(
					"update-alternatives failed with exit code: {}",
					status.code().unwrap_or(1)
				)
			)
		)
	} else {
		None
	}
}