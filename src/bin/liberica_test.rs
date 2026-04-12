use anyhow::Result;

use fuji::arch::Arch;
use fuji::jvm::jvm_liberica::{get_liberica_version, LibericaReleaseInfo};
use fuji::jvm::major_version::MajorVersion;
use fuji::jvm::manage_jvm::Feature;
use fuji::os::OS;

#[allow(clippy::unnecessary_operation)]
pub fn main() -> Result<()> {
	let string = get_liberica_version(
		&[Feature::Minimal],
		&OS::SYSTEM,
		&Arch::SYSTEM,
		&MajorVersion::Number(25),
	)?;
	dbg!(&string);
	let string1 = ureq::get(string).call()?.into_body().read_to_string()?;
	dbg!(&string1);
	let release_info = LibericaReleaseInfo {
		bitness: 64,
		latestLTS: true,
		updateVersion: 2,
		downloadUrl: String::from("https://github.com/bell-sw/Liberica/releases/download/25.0.2+12/bellsoft-jre25.0.2+12-linux-amd64.tar.gz"),
		latestInFeatureVersion: true,
		LTS: true,
		bundleType: String::from("jre"),
		featureVersion: 25,
		packageType: String::from("tar.gz"),
		FX: false,
		GA: true,
		architecture: String::from("x86"),
		latest: false,
		extraVersion: 0,
		buildVersion: 12,
		EOL: false,
		os: String::from("linux"),
		interimVersion: 0,
		version: String::from("25.0.2+12"),
		sha1: String::from("b9eed82f76e1f06cc4ed0b51cf60d007b0e75f90"),
		filename: String::from("bellsoft-jre25.0.2+12-linux-amd64.tar.gz"),
		installationType: String::from("archive"),
		size: 83672430,
		patchVersion: 0,
		TCK: true,
		updateType: String::from("psu"),
	};
	let string2 = serde_json::to_string(&vec![release_info])?;
	dbg!(&string2);
	dbg!(string1 == string2);
	Ok(())
}