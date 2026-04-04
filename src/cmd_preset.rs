use std::io::Result;

use crate::cli::{Cmd, Preset};
use crate::wrong_cmd;

pub fn cmd_preset(cmd: Cmd) -> Result<()> {
	let Cmd::Preset {
		preset,
	} = cmd else {
		wrong_cmd!(cmd_preset);
	};
	match preset {
		Preset::FastJRE => {
		},
		Preset::FastJDK => {
		},
		Preset::LatestJVM => {
		},
	};
	Ok(())
}