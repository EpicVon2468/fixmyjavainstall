//! Temurin (previously AdoptOpenJDK) by Eclipse/Adoptium – <https://adoptium.net/>.
use std::fmt::Write as _;

use anyhow::Result;

use crate::jvm::feature::Feature;
use crate::jvm::jvm_generic::{DownloadJVMArgs, jvm_download_impl};
use crate::os_name;

// https://github.com/adoptium/api.adoptium.net/blob/main/docs/cookbook.adoc
pub fn download_temurin(args: DownloadJVMArgs) -> Result<()> {
	let mut url: String = format!(
		"https://api.adoptium.net/v3/binary/latest/{}/ga/{}",
		args.version.major,
		os_name!(macOS = "mac"),
	);
	let _ = write!(
		url,
		"/{}/{}/hotspot/normal/eclipse",
		args.arch,
		if args.features.contains(&Feature::Minimal) {
			"jre"
		} else {
			"jdk"
		},
	);
	// returns a .zip instead of a .tar.gz for windows
	jvm_download_impl(url, args)
}
