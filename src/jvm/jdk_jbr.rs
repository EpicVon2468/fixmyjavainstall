use std::io::Result;

use crate::jvm::jdk_generic::{generic_download, DownloadJDKArgs};
use crate::jvm::manage_jvm::{Feature, JavaVersion};

pub fn download_jbr(arg: DownloadJDKArgs) -> Result<()> {
	let features: &[Feature] = arg.features;
	let version: &JavaVersion = &arg.version;
	let mut url: String = String::with_capacity(100);
	url.push_str("https://cache-redirector.jetbrains.com/intellij-jbr/jbr");
	if !features.contains(&Feature::Minimal) {
		url.push_str("sdk");
	};
	if features.contains(&Feature::JCEF) {
		url.push_str("_jcef");
	};
	url.push('-');
	url.push_str(version.specific);
	url.push('-');
	url.push_str(&arg.os.to_string());
	url.push('-');
	#[cfg(any(unix, feature = "multi_os"))]
	if features.contains(&Feature::MUSL) {
		url.push_str("musl-");
	};
	url.push_str(&arg.arch.to_string());
	url.push('-');
	url.push_str(version.revision);
	url.push_str(if arg.is_win() { ".zip" } else { ".tar.gz" });
	generic_download(url, arg.java_home, arg.dry_run, arg.is_win(), arg.is_mac())
}