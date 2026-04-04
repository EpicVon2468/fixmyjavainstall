use std::io::Result;

use crate::cli::Cmd;
use crate::wrong_cmd;

pub fn cmd_preset(cmd: &Cmd) -> Result<()> {
	let Cmd::Preset {
		..
	} = cmd else {
		wrong_cmd!(cmd_preset);
	};
	Ok(())
}