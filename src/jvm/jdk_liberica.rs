use std::io::Result;

use crate::jvm::jdk_generic::{generic_download, DownloadJDKArgs};
use crate::jvm::manage_jvm::{Feature, JavaVersion};

pub fn download_liberica(arg: DownloadJDKArgs) -> Result<()> {
	let mut url: String = String::with_capacity(100);
	let version: &JavaVersion = &arg.version;
	let target: &str = &format!("{}{}", version.specific, version.revision);
	url.push_str(&format!("https://download.bell-sw.com/java/{target}/bellsoft-"));
	url.push_str(if arg.features.contains(&Feature::Minimal) {
		"jre"
	} else {
		"jdk"
	});
	url.push_str(target);
	url.push('-');
	let os_name: &str = &arg.os.to_string();
	url.push_str(match os_name {
		"osx" => "macos",
		_ => os_name,
	});
	url.push('-');
	let arch_name: &str = &arg.arch.to_string();
	url.push_str(match arch_name {
		"x64" => "amd64",
		_ => arch_name,
	});
	url.push_str(if arg.is_win() { ".zip" } else { ".tar.gz" });
	generic_download(url, arg.java_home, arg.dry_run, arg.is_win(), arg.is_mac())
}