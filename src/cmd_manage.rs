use anyhow::Result;

use crate::cli::{Cmd, Software};
use crate::jvm::manage_jvm::manage_jvm;
use crate::wrong_cmd;

pub fn cmd_manage(command: Cmd) -> Result<()> {
	let Cmd::Manage {
		software: option,
	}: Cmd = command else {
		wrong_cmd!(cmd_manage);
	};
	if let Some(software) = option {
		match software {
			Software::JVM { .. } => manage_jvm(software)?,
			Software::Kotlin { .. } => todo!(),
			Software::KotlinNative { .. } => todo!(),
		};
	};
	Ok(())
}