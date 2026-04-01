use std::io::Result;

use crate::arch::Arch;
use crate::jvm::jdk_generic::generic_download;
use crate::jvm::manage_jvm::{Feature, JavaVersion};

// https://github.com/adoptium/api.adoptium.net/blob/main/docs/cookbook.adoc
pub fn download_temurin<S: AsRef<str>>(
	arch: &Arch,
	version: JavaVersion,
	features: &Vec<Feature>,
	output_dir: S,
	dry_run: bool
) -> Result<()> {
	let mut url: String = String::with_capacity(100);
	url.push_str("https://api.adoptium.net/v3/binary/latest/");
	url.push_str(version.major);
	url.push_str("/ga/linux/");
	url.push_str(&arch.to_string());
	url.push('/');
	url.push_str(if features.contains(&Feature::MINIMAL) { "jre" } else { "jdk" });
	url.push_str("/hotspot/normal/eclipse");
	generic_download(url, output_dir, dry_run)
}