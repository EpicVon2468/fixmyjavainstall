use std::fmt::{Display, Formatter};

use clap::ValueEnum;

// FIXME: Clap isn't picking up any extra lines
// 	https://github.com/clap-rs/clap/issues/6096
// 	https://github.com/clap-rs/clap/discussions/6070
#[derive(ValueEnum, Clone)]
pub enum Vendor {
	/// Automagically pick the best vendor based on the requested version and features
	Auto,
	/// JetBrains Runtime – https://github.com/JetBrains/JetBrainsRuntime/
	///
	/// <details><summary>Supported arches:</summary>
	///
	/// * `x64`
	/// * `aarch64`
	/// </details>
	///
	/// <details><summary>Supported features:</summary>
	///
	/// * `Minimal`
	/// * `JCEF`
	/// * `MUSL`
	/// </details>
	JBR,
	/// What is wrong with you?  Seriously, don't use this! – https://www.oracle.com/java/
	///
	/// <details><summary>Supported arches:</summary>
	///
	/// * `x64`
	/// * `aarch64`
	/// </details>
	/// <details><summary>Supported features:</summary>
	///
	/// </details>
	Oracle,
	/// Eclipse Adoptium (previously AdoptOpenJDK) – https://adoptium.net/
	///
	/// <details><summary>Supported arches:</summary>
	///
	/// * `x64`
	/// * `aarch64`
	/// * `riscv64`
	/// </details>
	///
	/// <details><summary>Supported features:</summary>
	///
	/// * `Minimal`
	/// </details>
	Adoptium,
	/// GraalVM – https://www.graalvm.org/
	///
	/// <details><summary>Supported arches:</summary>
	///
	/// * `x64`
	/// * `aarch64`
	/// </details>
	/// <details><summary>Supported features:</summary>
	///
	/// </details>
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