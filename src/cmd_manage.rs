use anyhow::Result;

use crate::cli::{FujiCmd, Software};
use crate::jvm::manage_jvm;
use crate::wrong_cmd;

pub fn cmd_manage(command: FujiCmd) -> Result<()> {
	let FujiCmd::Manage { software }: FujiCmd = command else {
		wrong_cmd!(cmd_manage);
	};
	match software {
		Software::JVM { .. } => manage_jvm(software)?,
		Software::Kotlin { .. } => todo!(),
		Software::KotlinNative { .. } => todo!(),
	};
	Ok(())
}
