use std::fmt::{Display, Formatter};

use clap::ValueEnum;

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

impl Display for Vendor {

	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Vendor::Auto => todo!("Automagic vendor selection"),
				Vendor::JBR => "jbr",
				Vendor::Oracle => "oracle",
				Vendor::Adoptium => "adoptium",
				Vendor::GraalVM => "graal-vm",
			}
		)
	}
}