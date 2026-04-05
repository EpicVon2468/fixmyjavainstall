use std::ffi::OsStr;
use std::fs::remove_file;
use std::io::Result;

use crate::arch::Arch;
use crate::commands::{download, untar_jdk};
use crate::jvm::manage_jvm::{Feature, JavaVersion};
use crate::os::OS;

pub type DownloadJDKFn = fn(arg: DownloadJDKArgs) -> Result<()>;

pub struct DownloadJDKArgs<'a> {
	pub arch: Arch,
	pub version: JavaVersion<'a>,
	pub features: &'a [Feature],
	pub os: OS,
	pub java_home: &'a str,
	pub dry_run: bool,
}

impl<'a> DownloadJDKArgs<'a> {

	pub fn is_win(&self) -> bool {
		self.os == OS::Windows
	}

	pub fn is_mac(&self) -> bool {
		self.os == OS::OSX
	}
}

pub fn generic_download<S: AsRef<OsStr>, P: AsRef<str>>(
	url: S,
	java_home: P,
	dry_run: bool,
	is_win: bool,
	is_mac: bool,
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
	untar_jdk(archive, java_home, is_win, is_mac).expect("Couldn't untar JDK!");

	println!("Removing JDK archive...");
	remove_file(archive).expect("Couldn't delete JDK archive!");

	Ok(())
}