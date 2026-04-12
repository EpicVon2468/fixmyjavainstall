use anyhow::Result;

use fuji::arch::Arch;
use fuji::jvm::jvm_liberica::get_liberica_version;
use fuji::jvm::major_version::MajorVersion;
use fuji::jvm::manage_jvm::Feature;
use fuji::os::OS;

pub fn main() -> Result<()> {
	let string = get_liberica_version(
		&[Feature::Minimal],
		&OS::SYSTEM,
		&Arch::SYSTEM,
		&MajorVersion::Number(19),
	)?;
	dbg!(&string);
	let string1 = ureq::get(string).call()?.into_body().read_to_string()?;
	dbg!(string1);
	Ok(())
}