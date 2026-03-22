use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
	version,
	about,
	long_about = "Fix Ur Java Install – A JVM & Kotlin management utility."
)]
pub struct Arguments {
	#[arg(long, value_name = "PATHS", trailing_var_arg = true, num_args = 1..)]
	pub install: Vec<String>,

	#[arg(short, long, value_name = "DIR", default_value = "/usr/bin")]
	pub link_dir: String,

	#[arg(short, long, default_value = "false")]
	pub use_update_alternatives: bool
}

// #[derive(Subcommand)]
// pub enum Install {
//
// }