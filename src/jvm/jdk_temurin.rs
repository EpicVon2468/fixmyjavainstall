use std::io::Result;

use crate::jvm::jdk_generic::{generic_download, DownloadJDKArgs};
use crate::jvm::manage_jvm::Feature;

// https://github.com/adoptium/api.adoptium.net/blob/main/docs/cookbook.adoc
pub fn download_temurin(args: DownloadJDKArgs) -> Result<()> {
	let mut url: String = String::with_capacity(100);
	url.push_str("https://api.adoptium.net/v3/binary/latest/");
	url.push_str(&args.version.major);
	url.push_str("/ga/");
	let os_name: &str = &args.os.to_string();
	url.push_str(match os_name {
		"osx" => "mac",
		_ => os_name,
	});
	url.push('/');
	url.push_str(&args.arch.to_string());
	url.push('/');
	url.push_str(if args.features.contains(&Feature::Minimal) {
		"jre"
	} else {
		"jdk"
	});
	// returns a .zip instead of a .tar.gz for windows
	url.push_str("/hotspot/normal/eclipse");
	generic_download(url, args)
}