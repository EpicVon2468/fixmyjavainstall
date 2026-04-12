//! Liberica by BellSoft – <https://bell-sw.com/libericajdk/>
use std::fmt::Write;

use anyhow::Result;

use crate::arch::Arch;
use crate::jvm::jvm_generic::{DownloadJVMArgs, jvm_download_impl};
use crate::jvm::major_version::MajorVersion;
use crate::jvm::manage_jvm::{Feature, JavaVersion};
use crate::os::OS;

// TODO: https://api.bell-sw.com/
pub fn download_liberica(args: DownloadJVMArgs) -> Result<()> {
	let mut url: String = String::with_capacity(100);
	let version: &JavaVersion = &args.version;
	let target: &str = &format!("{}{}", version.specific, version.revision);
	let _ = write!(url, "https://download.bell-sw.com/java/{target}/bellsoft-");
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

pub fn get_liberica_version(
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