use std::borrow::Cow;
use std::ffi::OsStr;
use std::fs::{remove_dir_all, File};
use std::io::{Error, ErrorKind, Result};
use std::path::{Component, Components, Path, PathBuf};
use std::process::{Child, Command, ExitStatus, Output, Stdio};

use flate2::read::GzDecoder;

use tar::{Archive, Entry};

use which::which;

use crate::{check_status, wait_and_check_status};

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

// https://stackoverflow.com/questions/845593/how-do-i-untar-a-subdirectory-into-the-current-directory
// sudo tar --strip-components 1 -xvf 25.0.2.tar.gz -C 25.0.2
/// Extracts `archive` into `dest`, stripping one component.
///
/// Implementation notes:
///
/// * `dest` is [`canonicalised`][`Path::canonicalize`] before use.
/// * No checks are performed to determine if `dest` exists.
/// * If `is_zip` is true, no checks are performed to determine if `archive` ends with `.zip`, and vice versa.
pub fn untar_jdk<S: AsRef<Path>, P: AsRef<Path>>(archive: S, dest: P, _is_zip: bool, is_mac: bool) -> Result<()> {
	let dest: PathBuf = dest.as_ref().canonicalize()?;
	let input: File = File::open(archive.as_ref()).expect("foo");
	let mut reader: Archive<GzDecoder<File>> = Archive::new(GzDecoder::new(input));
	for file in reader.entries()? {
		let mut file: Entry<GzDecoder<File>> = file?;
		let cow: Cow<Path> = file.path()?;
		let mut components: Components = cow.components();
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
		let path: &Path = components.as_path();
		file.unpack(dest.join(path))?;
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

/// Fetches a [`String`] resource from `url`.
///
/// Implementation notes:
///
/// * Uses [`cURL`](https://curl.se/) for the HTTP(S) request.
/// 	* [`require_program`] is always called as a safety precaution.
/// 	* If `url` is a redirect, it is followed automagically (via the `-L` flag).
/// 	* The [`standard output stream`][`Command::stdout`] of `cURL` is [`piped`][`Stdio::piped`] and used for the return value.
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
pub fn connect<S: AsRef<OsStr>>(url: S) -> Result<String> {
	require_program("curl")?;
	let child: Child = Command::new("curl")
		.arg("--silent")
		.arg("-L")
		.arg(url)
		.stdout(Stdio::piped())
		.spawn()
		.expect("Couldn't start cURL!");
	let output: Output = child.wait_with_output().expect("cURL never started?");
	let status: ExitStatus = output.status;
	check_status!(status, "cURL");
	Ok(String::from_utf8(output.stdout).unwrap())
}

pub fn io_expect<P: AsRef<Path>, S: AsRef<str>>(dest: P, msg: S) -> String {
	format!(
		"Couldn't {} path '{}'!",
		msg.as_ref(),
		dest.as_ref().display()
	)
}