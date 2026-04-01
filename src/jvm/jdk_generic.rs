use std::ffi::OsStr;
use std::fs::remove_file;
use std::io::Result;

use crate::arch::Arch;
use crate::commands::{download, untar_jdk};
use crate::jvm::manage_jvm::{Feature, JavaVersion};

pub type DownloadJdkFn = fn(&Arch, JavaVersion, &Vec<Feature>, &str, &bool) -> Result<()>;

pub fn generic_download<S: AsRef<OsStr>, P: AsRef<str>>(
	url: S,
	output_dir: P,
	dry_run: &bool
) -> Result<()> {
	let url: &OsStr = url.as_ref();
	let output_dir: &str = output_dir.as_ref();
	let archive: &str = &format!("{output_dir}.tar.gz");

	println!("Downloading JDK: {}...", url.display());
	if *dry_run {
		return Ok(());
	};
	download(url, archive).expect("Couldn't download JDK!");

	println!("Untaring JDK...");
	untar_jdk(archive, output_dir).expect("Couldn't untar JDK!");

	println!("Removing JDK tar...");
	remove_file(archive).expect("Couldn't delete JDK tar!");

	Ok(())
}