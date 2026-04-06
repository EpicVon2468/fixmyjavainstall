use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
	version,
	about,
	long_about = "Fix Ur Java Install – A JVM & Kotlin management utility."
)]
pub struct Arguments {
	#[command(subcommand)]
	pub command: Option<Cmd>,
}

#[derive(Subcommand)]
pub enum Cmd {
	#[cfg(any(not(windows), feature = "multi_os"))]
	Link {
		// #[arg(long, value_name = "PATHS", trailing_var_arg = true, num_args = 1..)]
		// TODO: should this be `Vec<PathBuf>` ?
		/// Input directories.  Note that on UNIX, the `/bin` suffix will be appended for each of these by the program
		paths: Vec<String>,

		/// Directory to link files into.  Does nothing on Windows
		#[cfg(any(not(windows), feature = "multi_os"))]
		#[arg(
			short,
			long,
			value_name = "DIR",
			default_value = "/usr/bin",
		)]
		link_dir: PathBuf,

		/// Whether to use update-alternatives for install.
		#[cfg(any(target_os = "linux", feature = "multi_os"))]
		#[arg(short, long, default_value = "false")]
		use_update_alternatives: bool,
	},
	Manage {
		#[command(subcommand)]
		software: Option<Software>,
	},
	Preset {
		#[command(subcommand)]
		preset: Preset,
	},
	/// UNIX `man` page generation
	#[clap(hide = true)]
	Manual {

		#[clap(
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
#[clap(subcommand_value_name = "PRESET")]
pub enum Preset {
	/// All the recommended defaults + optimisations for your system – Java Runtime Environment edition
	RecommendedJRE,
	/// All the recommended defaults + optimisations for your system – Java Development Kit edition
	RecommendedJDK,
	/// (Almost) all the optimisations – Java Runtime Environment edition; For the performance-wary user
	FastJRE,
	/// (Almost) all the optimisations – Java Development Kit edition; For the performance-wary developer
	FastJDK,
	/// Bleeding-edge & unstable, you say?
	LatestJRE,
	/// Bleeding-edge & unstable, you say?
	LatestJDK,
}

#[derive(Subcommand)]
pub enum Software {
	/// Manages the Java Virtual Machine – <https://www.java.com/>
	#[clap(display_name = "fuji-jvm")]
	JVM {
		#[command(subcommand)]
		op: crate::jvm::manage_jvm::Op,
	},
	/// Manages the Kotlin Programming Language – <https://kotlinlang.org/>
	#[clap(display_name = "fuji-kt")]
	Kotlin {
	},
	/// Manages the Kotlin/Native compiler – <https://kotlinlang.org/docs/native-overview.html>
	KotlinNative {
	},
}