use std::ffi::OsStr;
use std::fs::remove_file;
use std::io::Result;

use crate::arch::Arch;
use crate::commands::{download, untar_jdk};
use crate::jvm::manage_jvm::{Feature, JavaVersion};
use crate::os::OS;

pub type DownloadJdkFn = fn(
	arch: Arch,
	version: JavaVersion,
	features: &Vec<Feature>,
	os: OS,
	java_home: &str,
	dry_run: bool,
	is_win: bool
) -> Result<()>;

pub fn generic_download<S: AsRef<OsStr>, P: AsRef<str>>(
	url: S,
	java_home: P,
	dry_run: bool,
	is_win: bool
) -> Result<()> {
	let url: &OsStr = url.as_ref();
	let java_home: &str = java_home.as_ref();
	let archive: &str = &format!("{java_home}.{}", if is_win { "zip" } else { "tar.gz" });

	println!("Downloading JDK: {}...", url.display());
	if dry_run {
		return Ok(());
	};
	download(url, archive).expect("Couldn't download JDK!");

	println!("Untaring JDK...");
	untar_jdk(archive, java_home, is_win).expect("Couldn't untar JDK!");

	println!("Removing JDK archive...");
	remove_file(archive).expect("Couldn't delete JDK archive!");

	Ok(())
}