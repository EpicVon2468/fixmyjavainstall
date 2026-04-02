use std::io::Result;

use crate::arch::Arch;
use crate::jvm::jdk_generic::generic_download;
use crate::jvm::manage_jvm::{Feature, JavaVersion};
use crate::os::OS;

pub fn download_liberica(
	arch: &Arch,
	version: JavaVersion,
	features: &Vec<Feature>,
	os: &OS,
	output_dir: &str,
	dry_run: &bool
) -> Result<()> {
	let mut url: String = String::with_capacity(100);
	url.push_str("https://download.bell-sw.com/java/");
	let target: &str = &format!("{}{}", version.specific, version.revision);
	url.push_str(target);
	url.push_str("/bellsoft-");
	url.push_str(if features.contains(&Feature::MINIMAL) { "jre" } else { "jdk" });
	url.push_str(target);
	url.push('-');
	let os_name: &str = &os.to_string();
	url.push_str(
		match os_name {
			"osx" => "macos",
			_ => os_name,
		}
	);
	url.push('-');
	let arch_name: &str = &arch.to_string();
	url.push_str(
		match arch_name {
			"x64" => "amd64",
			_ => arch_name,
		}
	);
	// doesn't have a .tar.gz for windows
	url.push_str(".tar.gz");
	generic_download(url, output_dir, dry_run)
}