//! Liberica by BellSoft – <https://bell-sw.com/libericajdk/>
use anyhow::Result;

use crate::jvm::jvm_generic::{DownloadJVMArgs, jvm_download_impl};
use crate::jvm::manage_jvm::{Feature, JavaVersion};

// TODO: https://api.bell-sw.com/
pub fn download_liberica(args: DownloadJVMArgs) -> Result<()> {
	let mut url: String = String::with_capacity(100);
	let version: &JavaVersion = &args.version;
	let target: &str = &format!("{}{}", version.specific, version.revision);
	url.push_str(&format!(
		"https://download.bell-sw.com/java/{target}/bellsoft-"
	));
	url.push_str(if args.features.contains(&Feature::Minimal) {
		"jre"
	} else {
		"jdk"
	});
	url.push_str(target);
	url.push('-');
	let os_name: &str = &args.os.to_string();
	url.push_str(match os_name {
		"osx" => "macos",
		_ => os_name,
	});
	url.push('-');
	let arch_name: &str = &args.arch.to_string();
	url.push_str(match arch_name {
		"x64" => "amd64",
		_ => arch_name,
	});
	url.push_str(if args.is_win() { ".zip" } else { ".tar.gz" });
	jvm_download_impl(url, args)
}