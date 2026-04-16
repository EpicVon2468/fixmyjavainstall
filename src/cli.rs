use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
	version,
	long_version = "0.2.1 – \"Move at a reasonable pace and pretty please can GitHub Actions work this time...?\"",
	author
)]
/// Fix Ur Java Install – A JVM & Kotlin management utility.
///
/// (Re)writing this in Rust was simpler than debugging and 'fixing' the bash script(s).  I am serious.
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
		#[arg(short, long, value_name = "DIR", default_value = "/usr/bin")]
		link_dir: PathBuf,

		// TODO: 'InstallMethod' enumeration
		/// Whether to use update-alternatives for install.
		#[cfg(any(target_os = "linux", feature = "multi-os"))]
		#[arg(short, long, default_value = "false")]
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

#[derive(Subcommand)]
#[command(subcommand_value_name = "PRESET")]
pub enum Preset {
	/// All the recommended defaults + optimisations for your system – Java Runtime Environment edition.
	RecommendedJRE,
	/// All the recommended defaults + optimisations for your system – Java Development Kit edition.
	RecommendedJDK,
	/// (Almost) all the optimisations – Java Runtime Environment edition; For the performance-wary user.
	FastJRE,
	/// (Almost) all the optimisations – Java Development Kit edition; For the performance-wary developer.
	FastJDK,
	/// Bleeding-edge & unstable, you say?
	LatestJRE,
	/// Bleeding-edge & unstable, you say?
	LatestJDK,
}

#[derive(Subcommand)]
#[command(author, subcommand_value_name = "SOFTWARE")]
pub enum Software {
	/// Manages the Java Virtual Machine – <https://www.java.com/>.
	#[command(display_name = "fuji-jvm", alias = "java", author)]
	JVM {
		#[command(subcommand)]
		op: crate::jvm::Op,
	},
	/// Manages the Kotlin Programming Language – <https://kotlinlang.org/>.
	#[command(display_name = "fuji-kt", alias = "kt", author)]
	Kotlin {},
	// TODO: merge into Kotlin
	/// Manages the Kotlin/Native compiler – <https://kotlinlang.org/docs/native-overview.html>.
	#[command(author, hide = true)]
	KotlinNative {},
}
