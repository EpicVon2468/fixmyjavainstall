//! JetBrains Runtime by JetBrains – <https://github.com/JetBrains/JetBrainsRuntime/>.
use std::fmt::Write as _;

use anyhow::Result;

use crate::jvm::JavaVersion;
use crate::jvm::feature::Feature;
use crate::jvm::jvm_generic::{DownloadJVMArgs, jvm_download_impl};

pub fn download_jbr(args: DownloadJVMArgs) -> Result<()> {
	let features: &[Feature] = args.features;
	let version: &JavaVersion = &args.version;
	let mut url: String = String::with_capacity(100);
	url.push_str("https://cache-redirector.jetbrains.com/intellij-jbr/jbr");
	if !features.contains(&Feature::Minimal) {
		url.push_str("sdk");
	};
	if features.contains(&Feature::JCEF) {
		url.push_str("_jcef");
	};
	let _ = write!(url, "-{}-{}-", version.specific, args.os);
	#[cfg(any(target_env = "musl", feature = "multi-os"))]
	if features.contains(&Feature::MUSL) {
		url.push_str("musl-");
	};
	let _ = write!(
		url,
		"{}-{}.{}",
		args.arch,
		version.revision,
		if args.is_win() { "zip" } else { "tar.gz" },
	);
	jvm_download_impl(url, args)
}
