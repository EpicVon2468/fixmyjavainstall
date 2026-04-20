pub mod cmd_install;

use anyhow::{Context as _, Result};

use clap::Subcommand;

use serde::{Deserialize as Deserialise, Serialize as Serialise};

use crate::arch::Arch;
use crate::cli::Software;
#[cfg(feature = "multi-os")]
use crate::os::OS;
use crate::wrong_cmd;

#[derive(Subcommand)]
#[command(author)]
pub enum Op {
	#[command(author)]
	Install {
		#[arg(short, long, default_value_t)]
		arch: Arch,

		#[cfg(feature = "multi-os")]
		#[arg(short, long, visible_alias = "os", default_value_t)]
		operating_system: OS,

		/// Whether to bundle Kotlin/Native with the installation – <https://kotlinlang.org/docs/native-overview.html>.
		#[arg(short, long)]
		include_native: bool,

		/// <https://semver.org/> XOR 'stable' XOR 'unstable'.
		#[arg(default_value = "stable")]
		version: String,
	},
	#[command(author, visible_alias = "uninstall")]
	Remove,
}

pub fn manage_kotlin(software: Software) -> Result<()> {
	let Software::Kotlin { op }: Software = software else {
		wrong_cmd!(manage_kt);
	};
	match op {
		Op::Install { .. } => cmd_install::cmd_install(op).context("Couldn't install Kotlin!"),
		Op::Remove => todo!("fuji-kt remove"),
	}
}

#[derive(Serialise, Deserialise)]
pub struct KtVersion {
	pub version: String,
}
