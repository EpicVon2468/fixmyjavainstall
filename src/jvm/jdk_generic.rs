use std::ffi::OsStr;
use std::fs::remove_file;
use std::io::Result;

use crate::commands::{download, untar_jdk};

pub fn generic_download<S: AsRef<OsStr>, P: AsRef<str>>(
	url: S,
	output_dir: P
) -> Result<()> {
	let url: &OsStr = url.as_ref();
	let output_dir: &str = output_dir.as_ref();
	let archive: String = format!("{output_dir}.tar.gz");

	println!("Downloading JDK: {}...", url.display());
	download(url, &archive).expect("Couldn't download JDK!");

	println!("Untaring JDK...");
	untar_jdk(&archive, output_dir).expect("Couldn't untar JDK!");

	println!("Removing JDK tar...");
	remove_file(archive).expect("Couldn't delete JDK tar!");

	Ok(())
}