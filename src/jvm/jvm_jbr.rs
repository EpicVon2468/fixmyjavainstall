//! JetBrains Runtime by JetBrains – <https://github.com/JetBrains/JetBrainsRuntime/>
use anyhow::Result;

use crate::jvm::jvm_generic::{DownloadJVMArgs, jvm_download_impl};
use crate::jvm::{Feature, JavaVersion};

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
	url.push('-');
	url.push_str(&version.specific);
	url.push('-');
	url.push_str(&args.os.to_string());
	url.push('-');
	#[cfg(any(target_env = "musl", feature = "multi-os"))]
	if features.contains(&Feature::MUSL) {
		url.push_str("musl-");
	};
	url.push_str(&args.arch.to_string());
	url.push('-');
	url.push_str(&version.revision);
	url.push('.');
	url.push_str(if args.is_win() { "zip" } else { "tar.gz" });
	jvm_download_impl(url, args)
}