use std::fs::{remove_dir_all, remove_file};
use std::io::Result;
use std::path::Path;

use crate::arch::Arch;
use crate::commands::{download, untar_jdk};
use crate::jvm::manage_jvm::{Feature, JavaVersion};
use crate::os::OS;

pub type DownloadJDKFn = fn(arg: DownloadJDKArgs) -> Result<()>;

pub struct DownloadJDKArgs<'a> {
	pub arch: Arch,
	pub version: JavaVersion,
	pub features: &'a [Feature],
	pub os: OS,
	pub java_home: &'a Path,
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

pub fn generic_download<S: AsRef<str>>(url: S, args: DownloadJDKArgs) -> Result<()> {
	let url: &str = url.as_ref();
	let java_home: &Path = args.java_home;
	let is_win: bool = args.is_win();
	let archive: &Path = &java_home.with_added_extension(if is_win { "zip" } else { "tar.gz" });

	println!("Downloading JDK: {url}...");
	if args.dry_run {
		return Ok(());
	};
	// Might exist from a failed previous install
	if archive.exists() {
		if archive.is_dir() {
			remove_dir_all(archive)
		} else {
			remove_file(archive)
		}.expect("Couldn't remove unexpected pre-existing JDK archive!")
	};
	download(url, archive).expect("Couldn't download JDK!");

	println!("Untaring JDK...");
	untar_jdk(archive, java_home, is_win, args.is_mac()).expect("Couldn't untar JDK!");

	println!("Removing JDK archive...");
	remove_file(archive).expect("Couldn't delete JDK archive!");
	println!("Done.\n");

	Ok(())
}