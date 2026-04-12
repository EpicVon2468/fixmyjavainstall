use anyhow::Result;

use fuji::arch::Arch;
use fuji::jvm::jvm_liberica::{get_liberica_version, LibericaReleaseInfo};
use fuji::jvm::major_version::MajorVersion;
use fuji::jvm::manage_jvm::Feature;
use fuji::os::OS;

#[allow(clippy::unnecessary_operation)]
pub fn main() -> Result<()> {
	let url: String = get_liberica_version(
		&[Feature::Minimal],
		&OS::SYSTEM,
		&Arch::SYSTEM,
		&MajorVersion::Number(25),
	)?;
	dbg!(&url);
	let response: String = ureq::get(url).call()?.into_body().read_to_string()?;
	dbg!(&response);
	let values: Vec<LibericaReleaseInfo> = serde_json::from_str::<Vec<LibericaReleaseInfo>>(&response)?;
	dbg!(&values);
	let value: &LibericaReleaseInfo = values.first().unwrap();
	dbg!(value);
	Ok(())
}