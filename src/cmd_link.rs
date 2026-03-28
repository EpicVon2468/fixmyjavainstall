use std::ffi::OsStr;
use std::fs::remove_file;
use std::io::{Error, ErrorKind, Result};
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};
use std::process::{Child, Command};

use crate::cli::Cmd;
use crate::commands::{has_program, io_expect};
use crate::{wait_and_check_status, wrong_cmd};

pub fn cmd_link(command: &Cmd) -> Result<()> {
	let Cmd::Link {
		paths,
		link_dir,
		use_update_alternatives,
	} = command else {
		wrong_cmd!(cmd_link);
	};
	for path in paths {
		link(path, link_dir, *use_update_alternatives).expect(
			format!("Failed to link '{path}'!").as_str()
		);
	};
	Ok(())
}

pub fn link<P: AsRef<Path>, S: AsRef<str>>(path: P, link_dir: S, use_update_alternatives: bool) -> Result<()> {
	let path: &Path = path.as_ref();
	println!("Linking path: {}", path.display());
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
		let Some(filename): Option<&OsStr> = file.file_name() else {
			println!("Filename was none! '{}'", file.display());
			continue;
		};
		let dest: String = format!("{}/{}", link_dir.as_ref(), filename.display());
		if can_use_update_alternatives {
			debian_link(file, filename, dest).expect("Couldn't link with update-alternatives!");
		} else {
			symlink_link(file, dest).expect("Couldn't link with symlink!");
		};
	};
	Ok(())
}

fn symlink_link<P: AsRef<Path>, S: AsRef<OsStr>>(source: P, dest: S) -> Result<()> {
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

fn debian_link<P, S, S2>(file: P, filename: S, dest: S2) -> Result<()>
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
	wait_and_check_status!(install_child, "update-alternatives");
	let mut set_child: Child = Command::new("update-alternatives")
		.arg("--set")
		.arg(filename)
		.arg(file)
		.spawn()
		.expect("Couldn't start update-alternatives!");
	wait_and_check_status!(set_child, "update-alternatives");
	Ok(())
}