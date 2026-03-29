use std::env::consts::ARCH;
use std::fmt::{Display, Formatter};

use clap::{Subcommand, ValueEnum};

#[derive(Subcommand)]
pub enum Op {
	Install {
		/// The vendor for the requested JVM
		#[arg(short, long, default_value = "jbr")]
		vendor: Vendor,

		/// The architecture for the requested JVM.  Note that not every vendor may support every architecture, and some vendors may not offer certain features for all architectures.  Generally speaking, x64 (amd64) has the highest level of support overall
		#[arg(short, long, default_value="system")]
		arch: Arch,

		/// The features for the requested JVM.  Note that not every vendor may support every feature, and some vendors may only offer features for certain versions or with incompatibilities with other features
		#[arg(short, long)]
		features: Vec<Features>,

		/// The requested JVM version.  Any valid string (or 'latest' for the latest available version)
		// #[clap(default_value_t = String::from("latest"))]
		version: String,
	},
	Remove,
}

#[derive(ValueEnum, Clone)]
pub enum Arch {
	/// Automagically determine the system architecture
	System,
	/// 64-bit (amd64)
	X64,
	/// 64-bit AArch – https://developer.arm.com/Architectures/A64%20Instruction%20Set%20Architecture/
	Aarch64,
	/// 64-bit RISC-V – https://riscv.org/
	Riscv64,
}

impl Display for Arch {

	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Arch::System => match ARCH {
					"x86_64" => "x64",
					"arm" => "arm64",
					_ => ARCH,
				},
				Arch::X64 => "x64",
				Arch::Aarch64 => "aarch64",
				Arch::Riscv64 => "riscv64",
			}
		)
	}
}

#[derive(ValueEnum, Clone)]
pub enum Features {
	/// Minimal (non-SDK/JDK) JVM (often referred to as a 'JRE').  If you don't know what this means & aren't a developer, you probably want this
	MINIMAL,
	/// Java Chromium Embedded Framework – https://github.com/chromiumembedded/java-cef/
	JCEF,
	/// MUSL libc support – https://musl.libc.org/
	MUSL,
}

#[derive(ValueEnum, Clone)]
pub enum Vendor {
	/// Automagically pick the best vendor based on the requested version and features
	Auto,
	/// JetBrains Runtime – https://github.com/JetBrains/JetBrainsRuntime/
	JBR,
	/// What is wrong with you?  Seriously, don't use this! – https://www.oracle.com/java/
	Oracle,
	/// Eclipse Adoptium (previously AdoptOpenJDK) – https://adoptium.net/
	Adoptium,
	/// GraalVM – https://www.graalvm.org/
	GraalVM,
}