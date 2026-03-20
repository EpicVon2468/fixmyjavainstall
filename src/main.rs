use std::env;
use std::ffi::OsStr;
use std::fs::remove_file;
use std::io::{ErrorKind, Result};
use std::os::unix::fs::symlink;
use std::path::PathBuf;
use std::process::{Child, Command};

fn main() {
	for arg in env::args().skip(1) {
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
			let error = result.unwrap_err();
			if error.kind() == ErrorKind::AlreadyExists {
				println!("File already exists: {}", usr_bin_path);
				remove_file(&usr_bin_path)?;
				symlink(file, &usr_bin_path)?;
			} else {
				return Err(error)
			}
		}
		let mut x: Child = Command::new("update-alternatives")
			.arg("--install")
			.arg(usr_bin_path)
			.arg(filename)
			.arg(file)
			.arg("4000")
			.spawn()?;
		x.wait()?;
	};
	Ok(())
}