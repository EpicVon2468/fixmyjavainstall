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
	Install {
		// #[arg(long, value_name = "PATHS", trailing_var_arg = true, num_args = 1..)]
		paths: Vec<String>,

		#[arg(short, long, value_name = "DIR", default_value = "/usr/bin")]
		link_dir: String,

		#[arg(short, long, default_value = "false")]
		use_update_alternatives: bool,
	},
	Foo {},
}