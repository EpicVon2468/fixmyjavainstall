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
	#[clap(hide = cfg!(windows))]
	Link {
		// #[arg(long, value_name = "PATHS", trailing_var_arg = true, num_args = 1..)]
		/// Input directories.  Note that on UNIX, the `/bin` suffix will be appended for each of these by the program
		paths: Vec<String>,

		/// Directory to link files into.  Does nothing on Windows
		#[arg(
			short,
			long,
			value_name = "DIR",
			default_value = if cfg!(windows) { "" } else { "/usr/bin" }
		)]
		link_dir: String,

		/// Whether to use update-alternatives for install.
		#[arg(
			short,
			long,
			default_value = "false",
			hide = cfg!(not(target_os = "linux"))
		)]
		use_update_alternatives: bool,
	},
	Manage {
		#[command(subcommand)]
		software: Option<Software>,
	},
}

#[derive(Subcommand)]
pub enum Software {
	/// Java Virtual Machine – https://www.java.com/
	JVM {
		#[command(subcommand)]
		op: crate::jvm::manage_jvm::Op,
	},
	/// Kotlin – https://kotlinlang.org/
	Kotlin {
	},
	/// Kotlin/Native – https://kotlinlang.org/docs/native-overview.html
	KotlinNative {
	},
}