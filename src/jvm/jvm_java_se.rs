//! Java Platform, Standard Edition by Oracle – <https://www.oracle.com/java/>.
use std::fmt::Write as _;

use anyhow::Result;

use crate::jvm::jvm_generic::{DownloadJVMArgs, jvm_download_impl};

pub fn download_java_se(args: DownloadJVMArgs) -> Result<()> {
	let mut url: String = String::with_capacity(100);
	let major: &str = &args.version.major;
	let _ = write!(
		url,
		"https://download.oracle.com/java/{major}/latest/jdk-{major}_"
	);
	let os_name: &str = &args.os.to_string();
	url.push_str(match os_name {
		"osx" => "macos",
		_ => os_name,
	});
	url.push('-');
	url.push_str(&args.arch.to_string());
	url.push_str("_bin.");
	url.push_str(if args.is_win() { "zip" } else { "tar.gz" });
	jvm_download_impl(url, args)
}
