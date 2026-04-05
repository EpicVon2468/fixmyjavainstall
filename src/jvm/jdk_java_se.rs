use std::io::Result;

use crate::jvm::jdk_generic::{generic_download, DownloadJDKArgs};

pub fn download_java_se(arg: DownloadJDKArgs) -> Result<()> {
	let version = &arg.version;
	let mut url: String = String::with_capacity(100);
	url.push_str("https://download.oracle.com/java/");
	url.push_str(version.major);
	url.push_str("/latest/jdk-");
	url.push_str(version.major);
	url.push('_');
	let os_name: &str = &arg.os.to_string();
	url.push_str(match os_name {
		"osx" => "macos",
		_ => os_name,
	});
	url.push('-');
	url.push_str(&arg.arch.to_string());
	url.push_str("_bin.");
	url.push_str(if arg.is_win() { "zip" } else { "tar.gz" });
	generic_download(url, arg.java_home, arg.dry_run, arg.is_win(), arg.is_mac())
}