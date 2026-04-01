use std::io::Result;

use crate::arch::Arch;
use crate::jvm::jdk_generic::generic_download;
use crate::jvm::manage_jvm::{Feature, JavaVersion};

pub fn download_java_se<S: AsRef<str>>(
	arch: &Arch,
	version: JavaVersion,
	_features: &Vec<Feature>,
	output_dir: S,
	dry_run: bool
) -> Result<()> {
	let mut url: String = String::with_capacity(100);
	url.push_str("https://download.oracle.com/java/");
	url.push_str(version.major);
	url.push_str("/latest/jdk-");
	url.push_str(version.major);
	url.push_str("_linux-");
	url.push_str(&arch.to_string());
	url.push_str("_bin.tar.gz");
	generic_download(url, output_dir, dry_run)
}