use anyhow::Result;

use crate::kotlin::Op;
use crate::wrong_cmd;

pub fn cmd_install(op: Op) -> Result<()> {
	#[rustfmt::skip]
	let Op::Install {
		arch: _arch,
		include_native: _include_native,
		version: _version,
	}: Op = op else {
		wrong_cmd!(cmd_install);
	};
	Ok(())
}
