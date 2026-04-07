use std::borrow::Cow;
use std::fs::{remove_dir_all, File};
use std::io::{copy, Result};
use std::path::{Component, Components, Path, PathBuf};

use flate2::read::GzDecoder;

use tar::{Archive, Entry};

use ureq::{get, BodyReader};

use which::which;

/// Checks if the program `name` exists.  This is equivalent to `which(name).is_ok()`.
pub fn has_program(name: &str) -> bool {
	which(name).is_ok()
}

/// Extracts `archive` into `dest`, stripping one component.
///
/// Implementation notes:
///
/// * `dest` is [`canonicalised`][`Path::canonicalize`] before use.
/// * No checks are performed to determine if `dest` exists – however, [`canonicalise`][`Path::canonicalize`] will panic if it does not.
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

// TODO: add progress bar of some kind + test functionality
/// Downloads a resource from `url` to `dest`.
///
/// Implementation notes:
///
/// * Eagerly checks if `dest` exists.
/// 	* The [`result`][`Result`] of [`Path::try_exists`] is immediately [`unwrapped`][`Result::unwrap`].
/// 	* If `dest` exists and is a directory, [`remove_dir_all`] is called, and [`Path::try_exists`] is re-evaluated.
/// 	* If `dest` does not exist, [`File::create`] is called.
pub fn download<S: AsRef<str>, P: AsRef<Path>>(url: S, dest: P) -> Result<()> {
	let dest: &Path = dest.as_ref();
	let mut exists: bool = dest.exists();
	if exists && dest.is_dir() {
		remove_dir_all(dest).unwrap_or_else(|_| panic!("{}", io_expect(dest, "delete")));
		exists = dest.exists();
	};
	if !exists {
		File::create(dest).unwrap_or_else(|_| panic!("{}", io_expect(dest, "create")));
	};
	let mut resource: BodyReader = get(url.as_ref())
		.call()
		.expect("Couldn't download resource!")
		.into_body()
		.into_reader();
	let mut dest: File = File::open(dest).expect("Couldn't open destination file for download!");
	copy(&mut resource, &mut dest).expect("Couldn't download resource from URL!");
	Ok(())
}

pub fn io_expect<P: AsRef<Path>, S: AsRef<str>>(dest: P, msg: S) -> String {
	format!(
		"Couldn't {} path '{}'!",
		msg.as_ref(),
		dest.as_ref().display()
	)
}