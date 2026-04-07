use std::borrow::Cow;
use std::ffi::OsStr;
use std::fs::{remove_dir_all, File};
use std::io::{Error, ErrorKind, Result};
use std::path::{Component, Components, Path, PathBuf};
use std::process::{Child, Command};

use flate2::read::GzDecoder;

use tar::{Archive, Entry};

use which::which;

use crate::wait_and_check_status;

/// Checks if the program `name` exists.  This is equivalent to `which(name).is_ok()`.
pub fn has_program(name: &str) -> bool {
	which(name).is_ok()
}

pub fn require_program(name: &str) -> Result<()> {
	if !has_program(name) {
		Err(Error::new(
			ErrorKind::NotFound,
			format!("Couldn't find program '{name}'!"),
		))
	} else {
		Ok(())
	}
}

/// Extracts `archive` into `dest`, stripping one component.
///
/// Implementation notes:
///
/// * `dest` is [`canonicalised`][`Path::canonicalize`] before use.
/// * No checks are performed to determine if `dest` exists.
/// * If `is_zip` is true, no checks are performed to determine if `archive` ends with `.zip`, and vice versa.
pub fn untar_jdk<S: AsRef<Path>, P: AsRef<Path>>(
	archive: S,
	dest: P,
	is_zip: bool,
	is_mac: bool,
) -> Result<()> {
	if is_zip {
		panic!("no.");
	};
	let dest: PathBuf = dest.as_ref().canonicalize().expect("Couldn't canonicalise destination path!");
	let input: File = File::open(archive.as_ref()).expect("Couldn't open JDK archive!");
	let mut reader: Archive<GzDecoder<File>> = Archive::new(GzDecoder::new(input));
	for entry in reader.entries().expect("Couldn't iterate through JDK archive!") {
		let mut entry: Entry<GzDecoder<File>> = entry.expect("Couldn't get entry in JDK archive!");
		let path: Cow<Path> = entry.path().expect("Couldn't get path for entry in JDK archive!");
		let mut components: Components = path.components();
		// https://stackoverflow.com/questions/845593/how-do-i-untar-a-subdirectory-into-the-current-directory
		// --strip-components 1
		components.next();
		// macOS .tar.gz is laid out differently.  it's a '.app'...
		if is_mac {
			// skip "Contents"
			components.next();
			// only allow paths under "Home"
			if components.next() != Some(Component::Normal("Home".as_ref())) {
				continue;
			};
		};
		entry
			.unpack(dest.join(components.as_path()))
			.expect("Couldn't unpack file from JDK archive!");
	};
	Ok(())
}

/// Downloads a resource from `url` to `dest`.
///
/// Implementation notes:
///
/// * Eagerly checks if `dest` exists.
/// 	* The [`result`][`Result`] of [`Path::try_exists`] is immediately [`unwrapped`][`Result::unwrap`].
/// 	* If `dest` exists and is a directory, [`remove_dir_all`] is called, and [`Path::try_exists`] is re-evaluated.
/// 	* If `dest` does not exist, [`File::create`] is called.
/// * Uses [`cURL`](https://curl.se/) for the HTTP(S) request.
/// 	* [`require_program`] is always called as a safety precaution.
/// 	* If `url` is a redirect, it is followed automagically (via the `-L` flag).
/// 	* If `cURL`'s return value is non-success, [`Err`] is returned with [`Error::other`].
///
/// Platform-specific behaviour:
/// * `cURL` is used on all platforms.
/// 	* `cURL` is called via [`Command`], not via `libcURL`.
/// 		* Windows has had `cURL` bundled for a while.
/// 			* <https://devblogs.microsoft.com/commandline/tar-and-curl-come-to-windows/>
/// 			* <https://techcommunity.microsoft.com/blog/containers/tar-and-curl-come-to-windows/382409/>
/// 			* <https://curl.se/windows/microsoft.html>
/// 		* macOS has had `cURL` bundled for a while.
/// 		* If you don't have `cURL` on Linux... why?
pub fn download<S: AsRef<OsStr>, P: AsRef<Path>>(url: S, dest: P) -> Result<()> {
	require_program("curl")?;
	let dest: &Path = dest.as_ref();
	let mut exists: bool = dest.try_exists()?;
	if exists && dest.is_dir() {
		remove_dir_all(dest).unwrap_or_else(|_| panic!("{}", io_expect(dest, "delete")));
		exists = dest.try_exists()?;
	};
	if !exists {
		File::create(dest).unwrap_or_else(|_| panic!("{}", io_expect(dest, "create")));
	};
	let mut child: Child = Command::new("curl")
		.arg("-L")
		.arg(url)
		.arg("-o")
		.arg(dest.canonicalize()?)
		.spawn()
		.expect("Couldn't start cURL!");
	wait_and_check_status!(child, "cURL");
	Ok(())
}

pub fn io_expect<P: AsRef<Path>, S: AsRef<str>>(dest: P, msg: S) -> String {
	format!(
		"Couldn't {} path '{}'!",
		msg.as_ref(),
		dest.as_ref().display()
	)
}