//! Java Platform, Standard Edition by Oracle – <https://www.oracle.com/java/>.
use std::fmt::Write as _;

use anyhow::Result;

use crate::jvm::jvm_generic::{DownloadJVMArgs, jvm_download_impl};
use crate::{os_archive, os_name};

pub fn download_java_se(args: DownloadJVMArgs) -> Result<()> {
	let major: &str = &args.version.major;
	let mut url: String = format!(
		"https://download.oracle.com/java/{major}/latest/jdk-{major}_{}",
		os_name!(macOS = "macos"),
	);
	let _ = write!(url, "-{}_bin.{}", args.arch, os_archive!());
	jvm_download_impl(url, args)
}
