use clap::{Subcommand, ValueEnum};

#[derive(Subcommand)]
pub enum Op {
	Install {
		/// The vendor for the requested JVM
		#[arg(short, long, default_value = "jbr")]
		vendor: Vendor,

		/// The requested JVM version.  Any valid string (or 'latest' for the latest available version)
		// #[clap(default_value_t = String::from("latest"))]
		version: String,

		/// The features for the requested JVM.  Note that not every vendor may support every feature, and some vendors may only offer features for certain versions or with incompatibilities with other features
		#[arg(short, long)]
		features: Vec<Features>
	},
	Remove {
	},
}

#[derive(ValueEnum, Clone)]
pub enum Features {
	/// Java Chromium Embedded Framework - https://github.com/chromiumembedded/java-cef/
	JCEF,
	/// Minimal (non-SDK) JVM (often referred to as a 'JRE')
	MINIMAL,
	/// MUSL libc support - https://musl.libc.org/
	MUSL,
}

#[derive(ValueEnum, Clone)]
pub enum Vendor {
	/// Automagically pick the best vendor based on the requested version and features
	Auto,
	/// JetBrains Runtime - https://github.com/JetBrains/JetBrainsRuntime/
	JBR,
}