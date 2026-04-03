use std::ffi::OsStr;
use std::fs::remove_file;
use std::io::Result;

use crate::arch::Arch;
use crate::commands::{download, untar_jdk};
use crate::jvm::manage_jvm::{Feature, JavaVersion};
use crate::os::OS;

pub type DownloadJdkFn = fn(arch: &Arch, version: JavaVersion, features: &Vec<Feature>, os: &OS, java_home: &str, dry_run: &bool) -> Result<()>;

// TODO: fix windows downloads returning .zip
pub fn generic_download<S: AsRef<OsStr>, P: AsRef<str>>(
	url: S,
	java_home: P,
	dry_run: &bool
) -> Result<()> {
	let url: &OsStr = url.as_ref();
	let java_home: &str = java_home.as_ref();
	let archive: &str = &format!("{java_home}.tar.gz");

	println!("Downloading JDK: {}...", url.display());
	if *dry_run {
		return Ok(());
	};
	download(url, archive).expect("Couldn't download JDK!");

	println!("Untaring JDK...");
	untar_jdk(archive, java_home).expect("Couldn't untar JDK!");

	println!("Removing JDK tar...");
	remove_file(archive).expect("Couldn't delete JDK tar!");

	Ok(())
}