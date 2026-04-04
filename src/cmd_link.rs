use std::ffi::OsStr;
use std::fs::{remove_dir_all, remove_file};
use std::io::{Error, ErrorKind, Result};
use std::path::MAIN_SEPARATOR;
use std::path::{Path, PathBuf};
use std::process::{Child, Command};

use crate::cli::Cmd;
use crate::commands::{has_program, io_expect};
use crate::{wait_and_check_status, wrong_cmd};

#[cfg(any(not(windows), feature = "multi_os"))]
pub fn cmd_link(command: Cmd) -> Result<()> {
	let Cmd::Link {
		paths,
		#[cfg(any(not(windows), feature = "multi_os"))]
		link_dir,
		#[cfg(any(target_os = "linux", feature = "multi_os"))]
		use_update_alternatives,
	} = command else {
		wrong_cmd!(cmd_link);
	};
	#[cfg(all(windows, not(feature = "multi_os")))]
	let link_dir: &str = "";
	#[cfg(all(not(target_os = "linux"), not(feature = "multi_os")))]
	let use_update_alternatives: bool = false;
	for path in &paths {
		link_impl(
			path,
			&link_dir,
			use_update_alternatives
		).unwrap_or_else(|_| panic!("Failed to link '{path}'!"));
	};
	Ok(())
}

#[allow(unused_variables)]
pub fn link_impl<P: AsRef<Path>, S: AsRef<str>>(path: P, link_dir: S, use_update_alternatives: bool) -> Result<()> {
	let path: &Path = path.as_ref();
	println!("Linking path: {}", path.display());
	let bin: PathBuf = path.join("bin");

	#[cfg(windows)]
	return crate::win_link::win_link(bin);

	#[allow(unreachable_code)]
	let can_use_update_alternatives: bool = cfg!(target_os = "linux") && use_update_alternatives && has_program("update-alternatives");
	if !can_use_update_alternatives && use_update_alternatives {
		println!("Couldn't find update-alternatives on system when explicitly requested!");
		return Err(
			Error::new(
				ErrorKind::NotFound,
				"Couldn't find update-alternatives on system when explicitly requested!"
			)
		);
	};
	for entry in bin.read_dir().unwrap_or_else(|_| { panic!("{}", io_expect(bin, "list directory")) }) {
		let file: &PathBuf = &entry?.path();
		// TODO: '--quiet'
		println!("\n{}", file.display());
		if file.is_dir() {
			continue;
		};
		let Some(filename): Option<&OsStr> = file.file_name() else {
			println!("Filename was none! '{}'", file.display());
			continue;
		};
		let dest: String = format!("{}{MAIN_SEPARATOR}{}", link_dir.as_ref(), filename.display());
		if can_use_update_alternatives {
			debian_link(file, filename, dest).expect("Couldn't link with update-alternatives!");
		} else {
			symlink_link(file, dest).expect("Couldn't link with symlink!");
		};
	};
	Ok(())
}

/// Attempts to symbolically link `dest` to `source`.
///
/// Implementation notes:
///
/// * if `dest` already exists, it will be removed and [`symlink_impl`] will be called again.
/// 	* `dest` is not eagerly removed ([`symlink_impl`] is called without checking if `dest` exists already).
/// 	* If the delegate of [`symlink_impl`] doesn't fail with [`ErrorKind::AlreadyExists`], `dest` is not deleted.
/// 	* If `dest` exists and is a directory, [`remove_dir_all`] is used.
/// 	* If `dest` exists and is a file, [`remove_file`] is used.
/// 	* if [`symlink_impl`] fails a second time, this function panics.
pub fn symlink_link<P: AsRef<Path>, S: AsRef<Path>>(source: P, dest: S) -> Result<()> {
	let source: &Path = source.as_ref();
	let dest: &Path = dest.as_ref();
	let result: Result<()> = symlink_impl(source, dest);
	if let Err(error) = result {
		if error.kind() == ErrorKind::AlreadyExists {
			println!("Removing existing path: {}", dest.display());

			let remove: Result<()> = if dest.is_file() {
				remove_file(dest)
			} else {
				remove_dir_all(dest)
			};
			remove.unwrap_or_else(|_| { panic!("{}", io_expect(dest, "remove")) });

			symlink_impl(source, dest).expect("Symbolic linking failed second time, panicking!");
		} else {
			return Err(error);
		};
	};
	Ok(())
}

/// Cross-platform function for symbolic linking.
///
/// Platform-specific behaviour:
///
/// * UNIX-likes: Delegates to [`std::os::unix::fs::symlink`].
/// * Windows: Checks if `original` is a directory.  If `true`, delegates to [`std::os::windows::fs::symlink_dir`], else [`std::os::windows::fs::symlink_file`].
#[allow(rustdoc::broken_intra_doc_links)]
pub fn symlink_impl<P: AsRef<Path>, Q: AsRef<Path>>(original: P, link: Q) -> Result<()> {
	#[cfg(unix)] {
		use std::os::unix::fs::symlink;
		symlink(original, link)
	}
	#[cfg(windows)] {
		use std::os::windows::fs::{symlink_dir, symlink_file};
		return if original.as_ref().is_dir() {
			// https://doc.rust-lang.org/std/os/windows/fs/fn.symlink_dir.html
			symlink_dir(original, link)
		} else {
			// https://doc.rust-lang.org/std/os/windows/fs/fn.symlink_file.html
			symlink_file(original, link)
		};
	}
}

/// <https://man7.org/linux/man-pages/man1/update-alternatives.1.html>
pub fn debian_link<P: AsRef<Path>, S: AsRef<OsStr>, S2: AsRef<OsStr>>(
	file: P,
	filename: S,
	dest: S2
) -> Result<()> {
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