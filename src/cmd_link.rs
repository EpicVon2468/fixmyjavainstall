use std::cmp::min;
use std::ffi::OsStr;
use std::fs::{Metadata, ReadDir, remove_dir_all, remove_file};
use std::path::{Path, PathBuf};
use std::process::{Child, Command};

use anyhow::{Context as _, Result};

use indicatif::ProgressBar;

use crate::cli::FujiCmd;
use crate::commands::{has_program, io_failure, progress_bar};
use crate::{wait_and_check_status, wrong_cmd};

#[cfg(any(not(windows), feature = "multi-os"))]
pub fn cmd_link(command: FujiCmd) -> Result<()> {
	#[rustfmt::skip]
	let FujiCmd::Link {
		paths,
		#[cfg(any(not(windows), feature = "multi-os"))]
		link_dir,
		#[cfg(any(target_os = "linux", feature = "multi-os"))]
		use_update_alternatives,
	}: FujiCmd = command else {
		wrong_cmd!(cmd_link);
	};
	#[cfg(all(windows, not(feature = "multi-os")))]
	let link_dir: String = String::new();
	#[cfg(all(not(target_os = "linux"), not(feature = "multi-os")))]
	let use_update_alternatives: bool = false;
	for path in paths {
		println!("Linking {}...", path.display());
		link_impl(&path, &link_dir, use_update_alternatives)
			.with_context(|| format!("Failed to link '{}'!", path.display()))?;
		println!("Done.\n");
	}
	Ok(())
}

pub fn link_impl(
	path: &Path,
	link_dir: &Path,
	use_update_alternatives: bool,
) -> Result<()> {
	let bin: PathBuf = path.join("bin");

	#[cfg(windows)]
	return crate::win_link::win_link(bin.to_str().context("I hate Windows")?);

	#[allow(unreachable_code, reason = "Windows.")]
	let can_use_update_alternatives: bool =
		use_update_alternatives && has_program("update-alternatives");
	if !can_use_update_alternatives && use_update_alternatives {
		#[rustfmt::skip]
		return Err(std::io::Error::new(
			std::io::ErrorKind::NotFound,
			"Couldn't find update-alternatives on system when explicitly requested!",
		).into());
	};
	let max_len: u64 = bin.metadata()?.len();
	let pb: ProgressBar = progress_bar(max_len);
	let mut progress: u64 = 0;
	let entries: ReadDir = bin
		.read_dir()
		.with_context(|| io_failure(&bin, "list directory"))?;
	for entry in entries {
		let file: &Path = &entry?.path();
		if file.is_dir() {
			continue;
		};
		let metadata: Metadata = file.metadata()?;
		#[cfg(unix)]
		{
			use std::os::unix::fs::MetadataExt as _;

			use crate::commands::is_executable;

			if !is_executable(metadata.mode()) {
				continue;
			};
		};
		let filename: &OsStr = file
			.file_name()
			.context("Couldn't get filename for directory entry!")?;
		let dest: PathBuf = link_dir.join(filename);
		if can_use_update_alternatives {
			debian_link(file, filename, dest.as_os_str()).context("Couldn't link with update-alternatives!")?;
		} else {
			symlink_link(file, &dest).context("Couldn't link with symlink!")?;
		};
		progress = min(progress + metadata.len(), max_len);
		pb.set_position(progress);
	}
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
pub fn symlink_link(source: &Path, dest: &Path) -> Result<()> {
	if dest.exists() {
		#[rustfmt::skip]
		if dest.is_file() {
			remove_file(dest)
		} else {
			remove_dir_all(dest)
		}.with_context(|| io_failure(dest, "remove existing"))?;
	};
	symlink_impl(source, dest).with_context(|| {
		format!(
			"Couldn't perform symbolic linking! (source: '{}', dest: '{}')",
			source.display(),
			dest.display()
		)
	})
}

/// Cross-platform function for symbolic linking.
///
/// Platform-specific behaviour:
///
/// * UNIX-likes: Delegates to [`std::os::unix::fs::symlink`].
/// * Windows: Checks if `original` is a directory.  If `true`, delegates to [`std::os::windows::fs::symlink_dir`], else [`std::os::windows::fs::symlink_file`].
#[allow(
	rustdoc::broken_intra_doc_links,
	reason = "Conditionally compiled code."
)]
pub fn symlink_impl(original: &Path, link: &Path) -> Result<()> {
	#[cfg(unix)]
	{
		use std::os::unix::fs::symlink;

		symlink(original, link).context("UNIX symbolic linking failed!")
	}
	#[cfg(windows)]
	{
		use std::os::windows::fs::{symlink_dir, symlink_file};

		return if original.is_dir() {
			// https://doc.rust-lang.org/std/os/windows/fs/fn.symlink_dir.html
			symlink_dir(original, link).context("Windows directory symbolic linking failed!")
		} else {
			// https://doc.rust-lang.org/std/os/windows/fs/fn.symlink_file.html
			symlink_file(original, link).context("Windows file symbolic linking failed!")
		};
	}
}

/// <https://man7.org/linux/man-pages/man1/update-alternatives.1.html>
pub fn debian_link(
	file: &Path,
	filename: &OsStr,
	dest: &OsStr,
) -> Result<()> {
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
