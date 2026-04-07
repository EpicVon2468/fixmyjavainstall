use std::io::Result;

use crate::jvm::jdk_generic::{generic_download, DownloadJDKArgs};

pub fn download_java_se(args: DownloadJDKArgs) -> Result<()> {
	let mut url: String = String::with_capacity(100);
	let major: &str = &args.version.major;
	url.push_str(&format!("https://download.oracle.com/java/{major}/latest/jdk-{major}_"));
	let os_name: &str = &args.os.to_string();
	url.push_str(match os_name {
		"osx" => "macos",
		_ => os_name,
	});
	url.push('-');
	url.push_str(&args.arch.to_string());
	url.push_str("_bin.");
	url.push_str(if args.is_win() { "zip" } else { "tar.gz" });
	generic_download(url, args)
}