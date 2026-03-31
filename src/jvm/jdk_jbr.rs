use std::io::Result;

use crate::arch::Arch;
use crate::jvm::jdk_generic::generic_download;
use crate::jvm::manage_jvm::{Feature, JavaVersion};

pub fn download_jbr<S: AsRef<str>>(
	arch: &Arch,
	version: &JavaVersion,
	features: &Vec<Feature>,
	output_dir: S
) -> Result<()> {
	let mut url: String = String::with_capacity(100);
	url.push_str("https://cache-redirector.jetbrains.com/intellij-jbr/jbr");
	if !features.contains(&Feature::MINIMAL) {
		url.push_str("sdk");
	};
	if features.contains(&Feature::JCEF) {
		url.push_str("_jcef");
	};
	url.push('-');
	url.push_str(version.specific);
	url.push_str("-linux-");
	if features.contains(&Feature::MUSL) {
		url.push_str("musl-");
	};
	url.push_str(arch.to_string().as_str());
	url.push('-');
	url.push_str(version.revision);
	url.push_str(".tar.gz");
	generic_download(url, output_dir)
}