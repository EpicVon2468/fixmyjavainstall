//! Liberica by BellSoft – <https://bell-sw.com/libericajdk/>
use std::fmt::Write;

use anyhow::Result;

use serde::{Deserialize as Deserialise, Serialize as Serialise};

use crate::arch::Arch;
use crate::jvm::jvm_generic::{jvm_download_impl, DownloadJVMArgs};
use crate::jvm::major_version::MajorVersion;
use crate::jvm::manage_jvm::Feature;
use crate::os::OS;

pub fn download_liberica(args: DownloadJVMArgs) -> Result<()> {
	jvm_download_impl(args.version.major.clone(), args)
}

pub fn get_liberica_endpoint(
	features: &[Feature],
	os: &OS,
	arch: &Arch,
	version: &MajorVersion,
) -> Result<String> {
	let mut url: String = String::with_capacity(150);
	let _ = write!(url, "https://api.bell-sw.com/v1/liberica/releases?bundle-type=");
	url.push_str(if features.contains(&Feature::Minimal) {
		"jre"
	} else {
		"jdk"
	});
	url.push_str("&bitness=64");
	url.push_str("&version-modifier=latest");
	url.push_str("&os=");
	let os_name: &str = &os.to_string();
	url.push_str(match os_name {
		"osx" => "macos",
		_ => os_name,
	});
	#[cfg(any(target_env = "musl", feature = "multi-os"))]
	if features.contains(&Feature::MUSL) {
		url.push_str("-musl");
	};
	match version {
		MajorVersion::Number(num) => {
			let _ = write!(url, "&version-feature={num}");
		},
		MajorVersion::Latest => (),
		MajorVersion::LTS => url.push_str("&release-type=lts"),
	};
	url.push_str("&arch=");
	let arch_name: &str = &arch.to_string();
	url.push_str(match arch_name {
		"x64" => "x86",
		"aarch64" => "arm",
		"riscv64" => "riscv",
		_ => arch_name,
	});
	url.push_str("&package-type=");
	url.push_str(if os == &OS::Windows { "zip" } else { "tar.gz" });
	url.push_str("&installation-type=archive");
	Ok(url)
}

#[allow(non_snake_case, clippy::struct_excessive_bools)]
#[derive(Serialise, Deserialise, Debug)]
pub struct LibericaReleaseInfo {
	pub bitness: u8,
	pub latestLTS: bool,
	pub updateVersion: i32,
	pub downloadUrl: String,
	pub latestInFeatureVersion: bool,
	pub LTS: bool,
	pub bundleType: String,
	pub featureVersion: u32,
	pub packageType: String,
	pub FX: bool,
	pub GA: bool,
	pub architecture: String,
	pub latest: bool,
	pub extraVersion: i32,
	pub buildVersion: i32,
	pub EOL: bool,
	pub os: String,
	pub interimVersion: i32,
	pub version: String,
	pub sha1: String,
	pub filename: String,
	pub installationType: String,
	pub size: u64,
	pub patchVersion: i32,
	pub TCK: bool,
	pub updateType: String,
}