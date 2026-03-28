use std::io::Result;

use crate::cli::Cmd;
use crate::wrong_cmd;

pub fn cmd_manage(command: &Cmd) -> Result<()> {
	let Cmd::Manage {
		..
	} = command else {
		wrong_cmd!(cmd_manage);
	};
	Ok(())
}