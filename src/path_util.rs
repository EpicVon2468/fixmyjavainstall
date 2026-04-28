use std::fs::{File, read_to_string};
use std::io::Write as _;

use anyhow::{Context as _, Result};

use crate::commands::io_failure;
use crate::{exists, lock, unlock};

#[cfg(windows)]
pub fn add_to_path<P: AsRef<str>>(dir: P) -> Result<()> {
	crate::win_link::win_link(dir)
}

#[cfg(unix)]
pub const FUJI_PATH_FILE: &str = cfg_select! {
	target_os = "macos" => "/etc/paths.d/fuji-managed-paths",
	// Despite being .sh, you don't seem to actually need to make it executable
	target_os = "linux" => "/etc/profile.d/fuji-managed-paths.sh",
	_ => unimplemented!(),
};

#[cfg(unix)]
fn fail(reason: &str) -> String {
	io_failure(FUJI_PATH_FILE, reason)
}

#[cfg(unix)]
pub fn existing_path_file() -> Result<String> {
	read_to_string(FUJI_PATH_FILE).with_context(|| fail("read"))
}

#[cfg(unix)]
fn append_or_create_path_file() -> Result<File> {
	let file: File = File::options()
		.append(true)
		.create(true)
		.open(FUJI_PATH_FILE)
		.with_context(|| fail("append/create"))?;
	Ok(file)
}

#[cfg(target_os = "macos")]
pub fn add_to_path<P: AsRef<str>>(dir: P) -> Result<()> {
	let dir: &str = dir.as_ref();
	if exists!(dir) && existing_path_file()?.contains(dir) {
		println!("Path file already contained directory {dir}!  Continuing!");
		return Ok(());
	};
	let mut file: File = append_or_create_path_file()?;
	lock!(file);
	writeln!(file, "{dir}").with_context(|| fail("write"))?;
	unlock!(file);
	Ok(())
}

#[cfg(target_os = "linux")]
pub const PREPEND_FUNCTION: &str = "\
	prepend_dir_to_path() {\n\
		case \":$PATH:\" in\n\
			*:\"$1\":*)\n\
				;;\n\
			*)\n\
				export PATH=\"$1:$PATH\"\n\
				;;\n\
		esac\n\
	}\n\
";

#[cfg(target_os = "linux")]
pub fn add_to_path<P: AsRef<str>>(dir: P) -> Result<()> {
	let dir: &str = dir.as_ref();
	let exists: bool = exists!(dir);
	if exists && existing_path_file()?.contains(dir) {
		println!("Path file already contained directory {dir}!  Continuing!");
		return Ok(());
	};
	let mut file: File = append_or_create_path_file()?;
	lock!(file);
	if !exists {
		writeln!(file, "#!/usr/bin/env sh").with_context(|| fail("write"))?;
		writeln!(file, "{PREPEND_FUNCTION}").with_context(|| fail("write"))?;
	};
	// If this `test` isn't available, then the user isn't on a UNIX compliant system
	writeln!(file, r#"test -d {dir} && prepend_dir_to_path "{dir}""#)
		.with_context(|| fail("write"))?;
	unlock!(file);
	Ok(())
}
