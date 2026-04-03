use std::io::Result;

use crate::arch::Arch;
use crate::jvm::jdk_generic::generic_download;
use crate::jvm::manage_jvm::{Feature, JavaVersion};
use crate::os::OS;

// https://github.com/adoptium/api.adoptium.net/blob/main/docs/cookbook.adoc
pub fn download_temurin(
	arch: &Arch,
	version: JavaVersion,
	features: &Vec<Feature>,
	os: &OS,
	java_home: &str,
	dry_run: &bool,
	is_win: bool
) -> Result<()> {
	let mut url: String = String::with_capacity(100);
	url.push_str("https://api.adoptium.net/v3/binary/latest/");
	url.push_str(version.major);
	url.push_str("/ga/");
	let os_name: &str = &os.to_string();
	url.push_str(
		match os_name {
			"osx" => "mac",
			_ => os_name,
		}
	);
	url.push('/');
	url.push_str(&arch.to_string());
	url.push('/');
	url.push_str(if features.contains(&Feature::MINIMAL) { "jre" } else { "jdk" });
	// returns a .zip instead of a .tar.gz for windows
	url.push_str("/hotspot/normal/eclipse");
	generic_download(url, java_home, dry_run, is_win)
}