use std::io::Result;

use crate::arch::Arch;
use crate::jvm::jdk_generic::generic_download;
use crate::jvm::manage_jvm::{Feature, JavaVersion};
use crate::os::OS;

pub fn download_jbr(
	arch: Arch,
	version: JavaVersion,
	features: &Vec<Feature>,
	os: OS,
	java_home: &str,
	dry_run: bool,
	is_win: bool
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
	url.push('-');
	url.push_str(&os.to_string());
	url.push('-');
	#[cfg(any(unix, feature = "multi_os"))]
	if features.contains(&Feature::MUSL) {
		url.push_str("musl-");
	};
	url.push_str(&arch.to_string());
	url.push('-');
	url.push_str(version.revision);
	url.push_str(if is_win { ".zip" } else { ".tar.gz" });
	generic_download(url, java_home, dry_run, is_win)
}