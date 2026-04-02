use std::io::Result;

use crate::arch::Arch;
use crate::jvm::jdk_generic::generic_download;
use crate::jvm::manage_jvm::{Feature, JavaVersion};
use crate::os::OS;

pub fn download_java_se(
	arch: &Arch,
	version: JavaVersion,
	_features: &Vec<Feature>,
	os: &OS,
	output_dir: &str,
	dry_run: &bool
) -> Result<()> {
	let mut url: String = String::with_capacity(100);
	url.push_str("https://download.oracle.com/java/");
	url.push_str(version.major);
	url.push_str("/latest/jdk-");
	url.push_str(version.major);
	url.push('_');
	let os_name: &str = &os.to_string();
	url.push_str(
		match os_name {
			"osx" => "macos",
			_ => os_name,
		}
	);
	url.push('-');
	url.push_str(&arch.to_string());
	// doesn't have a .tar.gz for windows
	url.push_str("_bin.tar.gz");
	generic_download(url, output_dir, dry_run)
}