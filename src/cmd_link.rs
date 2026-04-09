use std::cmp::min;
use std::ffi::OsStr;
use std::fs::{remove_dir_all, remove_file};
use std::path::{Path, PathBuf};
use std::process::{Child, Command};

use anyhow::{Context, Result};

use indicatif::ProgressBar;

use crate::cli::Cmd;
use crate::commands::{has_program, io_failure, progress_bar};
use crate::{wait_and_check_status, wrong_cmd};

#[cfg(any(not(windows), feature = "multi-os"))]
pub fn cmd_link(command: Cmd) -> Result<()> {
	let Cmd::Link {
		paths,
		#[cfg(any(not(windows), feature = "multi-os"))]
		link_dir,
		#[cfg(any(target_os = "linux", feature = "multi-os"))]
		use_update_alternatives,
	}: Cmd = command else {
		wrong_cmd!(cmd_link);
	};
	#[cfg(all(windows, not(feature = "multi-os")))]
	let link_dir: String = "".into();
	#[cfg(all(not(target_os = "linux"), not(feature = "multi-os")))]
	let use_update_alternatives: bool = false;
	for path in paths {
		println!("Linking {}...", path.display());
		link_impl(
			&path,
			&link_dir,
			use_update_alternatives,
		).with_context(|| format!("Failed to link '{}'!", path.display()))?;
		println!("Done.\n");
	}
	Ok(())
}

pub fn link_impl<P: AsRef<Path>, S: AsRef<Path>>(
	path: P,
	link_dir: S,
	use_update_alternatives: bool,
) -> Result<()> {
	let path: &Path = path.as_ref();
	println!("Linking path: {}", path.display());
	let bin: PathBuf = path.join("bin");

	#[cfg(windows)]
	return crate::win_link::win_link(bin.to_str().context("I hate Windows")?);

	#[allow(unreachable_code)]
	let can_use_update_alternatives: bool = cfg!(target_os = "linux") && use_update_alternatives && has_program("update-alternatives");
	if !can_use_update_alternatives && use_update_alternatives {
		return Err(std::io::Error::new(
			std::io::ErrorKind::NotFound,
			"Couldn't find update-alternatives on system when explicitly requested!",
		).into());
	};
	let max_len: u64 = bin.metadata()?.len();
	let pb: ProgressBar = progress_bar(max_len);
	let mut progress: u64 = 0;
	for entry in bin.read_dir().with_context(|| io_failure(bin, "list directory"))? {
		let file: &Path = &entry?.path();
		if file.is_dir() {
			continue;
		};
		let filename: &OsStr = file.file_name().context("Couldn't get filename for directory entry!")?;
		let dest: PathBuf = link_dir.as_ref().join(filename);
		if can_use_update_alternatives {
			debian_link(file, filename, dest).context("Couldn't link with update-alternatives!")?;
		} else {
			symlink_link(file, dest).context("Couldn't link with symlink!")?;
		};
		progress = min(progress + file.metadata()?.len(), max_len);
		pb.set_position(progress);
	};
	pb.finish();
	Ok(())
}

/// Attempts to symbolically link `dest` to `source`.
///
/// Implementation notes:
///
/// * if `dest` already exists, it will be eagerly removed before [`symlink_impl`] is called.
/// 	* If `dest` exists and is a directory, [`remove_dir_all`] is used.
/// 	* If `dest` exists and is a file, [`remove_file`] is used.
pub fn symlink_link<P: AsRef<Path>, S: AsRef<Path>>(source: P, dest: S) -> Result<()> {
	let source: &Path = source.as_ref();
	let dest: &Path = dest.as_ref();
	if dest.exists() {
		if dest.is_file() {
			remove_file(dest)
		} else {
			remove_dir_all(dest)
		}.with_context(|| format!("Couldn't remove existing path '{}'!", dest.display()))?;
	};
	symlink_impl(source, dest).with_context(||
		format!(
			"Couldn't perform symbolic linking! (source: '{}', dest: '{}')",
			source.display(), dest.display()
		)
	)
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
		symlink(original, link).context("UNIX symbolic linking failed!")
	}
	#[cfg(windows)] {
		use std::os::windows::fs::{symlink_dir, symlink_file};
		return if original.as_ref().is_dir() {
			// https://doc.rust-lang.org/std/os/windows/fs/fn.symlink_dir.html
			symlink_dir(original, link).context("Windows directory symbolic linking failed!")
		} else {
			// https://doc.rust-lang.org/std/os/windows/fs/fn.symlink_file.html
			symlink_file(original, link).context("Windows file symbolic linking failed!")
		};
	}
}

/// <https://man7.org/linux/man-pages/man1/update-alternatives.1.html>
pub fn debian_link<P: AsRef<Path>, S: AsRef<OsStr>, S2: AsRef<OsStr>>(
	file: P,
	filename: S,
	dest: S2,
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
		.context("Couldn't start update-alternatives!")?;
	wait_and_check_status!(install_child, "update-alternatives");
	let mut set_child: Child = Command::new("update-alternatives")
		.arg("--set")
		.arg(filename)
		.arg(file)
		.spawn()
		.context("Couldn't start update-alternatives!")?;
	wait_and_check_status!(set_child, "update-alternatives");
	Ok(())
}