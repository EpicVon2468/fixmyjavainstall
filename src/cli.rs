use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::{LINK_DIR, LONG_VERSION};

/// Fix Ur Java Install – A JVM & Kotlin management utility.
///
/// (Re)writing this in Rust was simpler than debugging and 'fixing' the bash script(s).  I am serious.
#[derive(Parser)]
#[command(
	version,
	long_version = LONG_VERSION,
	author
)]
pub struct FujiArgs {
	#[command(subcommand)]
	pub command: Option<FujiCmd>,
}

#[derive(Subcommand)]
#[command(author)]
pub enum FujiCmd {
	#[cfg(any(not(windows), feature = "multi-os"))]
	#[command(author)]
	Link {
		/// Input directories.  Note that on UNIX, the `/bin` suffix will be appended for each of these by the program.
		paths: Vec<PathBuf>,

		/// Directory to link files into.  Does nothing on Windows.
		#[cfg(any(not(windows), feature = "multi-os"))]
		#[arg(short, long, value_name = "DIR", default_value = LINK_DIR)]
		link_dir: PathBuf,

		// TODO: 'InstallMethod' enumeration
		/// Whether to use update-alternatives for install.
		#[cfg(any(target_os = "linux", feature = "multi-os"))]
		#[arg(short, long, default_value_t)]
		use_update_alternatives: bool,
	},
	/// Manages software.
	#[command(author)]
	Manage {
		#[command(subcommand)]
		software: Software,
	},
	/// UNIX `man` page generation.
	#[command(author, hide = true)]
	Manual {
		#[arg(
			value_name = "DIR",
			default_value = if cfg!(feature = "dev") {
				"./man"
			} else {
				"/usr/share/man"
			},
		)]
		man_dir: PathBuf,
	},
}

#[non_exhaustive]
#[derive(Subcommand)]
#[command(author, subcommand_value_name = "SOFTWARE")]
pub enum Software {
	/// Manages the Java Virtual Machine – <https://www.java.com/>.
	#[command(author, display_name = "fuji-jvm", alias = "java")]
	JVM {
		#[command(subcommand)]
		op: crate::jvm::Op,
	},
	/// Manages the Kotlin Programming Language – <https://kotlinlang.org/>.
	#[command(author, display_name = "fuji-kt", alias = "kt")]
	Kotlin {
		#[command(subcommand)]
		op: crate::kotlin::Op,
	},
}
