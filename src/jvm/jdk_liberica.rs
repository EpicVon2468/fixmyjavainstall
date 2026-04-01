use std::io::Result;

use crate::arch::Arch;
use crate::jvm::jdk_generic::generic_download;
use crate::jvm::manage_jvm::{Feature, JavaVersion};

pub fn download_liberica<S: AsRef<str>>(
	arch: &Arch,
	version: JavaVersion,
	features: &Vec<Feature>,
	output_dir: S
) -> Result<()> {
	let mut url: String = String::with_capacity(100);
	url.push_str("https://download.bell-sw.com/java/");
	let target: &str = &format!("{}{}", version.specific, version.revision);
	url.push_str(target);
	url.push_str("/bellsoft-");
	url.push_str(if features.contains(&Feature::MINIMAL) { "jre" } else { "jdk" });
	url.push_str(target);
	url.push_str("-linux-");
	let arch_str: &str = &arch.to_string();
	url.push_str(
		match arch_str {
			"x64" => "amd64",
			_ => arch_str,
		}
	);
	url.push_str(".tar.gz");
	generic_download(url, output_dir)
}