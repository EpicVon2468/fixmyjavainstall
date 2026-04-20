pub mod cmd_install;

use anyhow::Result;

use clap::Subcommand;

use serde::{Deserialize as Deserialise, Serialize as Serialise};

use crate::cli::Software;
#[cfg(feature = "multi-os")]
use crate::os::OS;
use crate::wrong_cmd;

#[derive(Subcommand)]
#[command(author)]
pub enum Op {
	#[command(author)]
	Install {
		#[cfg(feature = "multi-os")]
		#[arg(short, long, alias = "os", default_value_t)]
		operating_system: OS,

		/// <https://kotlinlang.org/docs/native-overview.html>.
		#[arg(short = 'n', long)]
		include_native: bool,

		// https://semver.org/ XOR 'stable' XOR 'unstable'
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
		Op::Install { .. } => todo!(),
		Op::Remove => todo!("fuji-kt remove"),
	}
}

#[derive(Serialise, Deserialise)]
pub struct KtVersion {
	pub version: String,
}
