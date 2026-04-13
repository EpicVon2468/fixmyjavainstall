//! Liberica by BellSoft – <https://bell-sw.com/libericajdk/>
use std::fmt::Write as _;

use anyhow::{Context as _, Result};

use serde::{Deserialize as Deserialise, Serialize as Serialise};

use crate::arch::Arch;
use crate::jvm::Feature;
use crate::jvm::jvm_generic::{DownloadJVMArgs, jvm_download_impl};
use crate::jvm::major_version::MajorVersion;
use crate::os::OS;

pub fn download_liberica(args: DownloadJVMArgs) -> Result<()> {
	jvm_download_impl(args.version.major.clone(), args)
}

pub fn get_liberica_download(
	features: &[Feature],
	os: &OS,
	arch: &Arch,
	version: &MajorVersion,
) -> Result<String> {
	let uri: String = get_liberica_endpoint(features, os, arch, version)?;
	let values: Vec<LibericaReleaseInfo> = ureq::get(uri)
		.call()
		.context("No Liberica JVM was available for the provided request!")?
		.body_mut()
		.read_json()
		.context("Couldn't read Liberica JVM version information!")?;
	let the_one: &LibericaReleaseInfo = values
		.first()
		.context("No Liberica JVM was available for the provided request!")?;
	if the_one.EOL {
		eprintln!("The requested JVM is at End Of Life!  Consider upgrading to a newer version!");
	};
	Ok(the_one.downloadUrl.clone())
}

pub fn get_liberica_endpoint(
	features: &[Feature],
	os: &OS,
	arch: &Arch,
	version: &MajorVersion,
) -> Result<String> {
	let mut url: String = String::with_capacity(175);
	let _ = write!(
		url,
		"https://api.bell-sw.com/v1/liberica/releases?bundle-type={}&bitness=64&version-modifier=latest&os=",
		if features.contains(&Feature::Minimal) {
			"jre"
		} else {
			"jdk"
		},
	);
	let os_name: &str = &os.to_string();
	url.push_str(match os_name {
		"osx" => "macos",
		_ => os_name,
	});
	#[cfg(any(target_env = "musl", feature = "multi-os"))]
	if features.contains(&Feature::MUSL) {
		url.push_str("-musl");
	};
	match *version {
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

/// 1:1 mapping of Liberica's endpoint @ <https://api.bell-sw.com/v1/liberica/releases/>
#[allow(non_snake_case, clippy::struct_excessive_bools)]
#[derive(Serialise, Deserialise)]
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