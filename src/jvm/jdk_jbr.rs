use std::fs::remove_file;

use crate::arch::Arch;
use crate::commands::{download, untar_jdk};
use crate::jvm::manage_jvm::{Feature, JavaVersion};

pub fn download_jbr<T: AsRef<str>>(
	arch: &Arch,
	version: &JavaVersion,
	features: &Vec<Feature>,
	output_dir: T
) -> std::io::Result<()> {
	let output_dir: &str = output_dir.as_ref();
	let mut url: String = String::with_capacity(100);
	url.push_str("https://cache-redirector.jetbrains.com/intellij-jbr/jbr");
	if !features.contains(&Feature::MINIMAL) {
		url.push_str("sdk");
	};
	if features.contains(&Feature::JCEF) {
		url.push_str("_jcef");
	};
	url.push('-');
	url.push_str(version.major);
	url.push_str("-linux-");
	if features.contains(&Feature::MUSL) {
		url.push_str("musl-");
	};
	url.push_str(arch.to_string().as_str());
	url.push('-');
	url.push_str(version.revision);
	url.push_str(".tar.gz");
	println!("Downloading JDK: {url}...");
	let archive: String = format!("{output_dir}.tar.gz");
	download(url, &archive).expect("Couldn't download JDK!");
	println!("Untaring JDK...");
	untar_jdk(&archive, output_dir).expect("Couldn't untar JDK!");
	println!("Removing JDK tar...");
	remove_file(archive).expect("Couldn't delete JDK tar!");
	Ok(())
}