use anyhow::Result;

use crate::kotlin::Op;
use crate::wrong_cmd;

pub fn cmd_install(op: Op) -> Result<()> {
	#[rustfmt::skip]
	let Op::Install {
		arch,
		#[cfg(feature = "multi-os")]
		operating_system: os,
		include_native,
		version,
	}: Op = op else {
		wrong_cmd!(cmd_install);
	};
	#[cfg(not(feature = "multi-os"))]
	let os: crate::os::OS = Default::default();
	Ok(())
}
