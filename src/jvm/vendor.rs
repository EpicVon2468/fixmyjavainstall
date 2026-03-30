use std::fmt::{Display, Formatter};

use clap::builder::PossibleValue;
use clap::ValueEnum;

// FIXME: Clap isn't picking up any extra lines
// 	https://github.com/clap-rs/clap/issues/6096
// 	https://github.com/clap-rs/clap/discussions/6070
#[derive(Clone)]
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
	///
	/// <details><summary>Supported versions:</summary>
	///
	/// * `25`
	/// * `21`
	/// * `17`
	/// </details>
	JBR,
	/// What is wrong with you?  Seriously, don't use this! – https://www.oracle.com/java/
	///
	/// <details><summary>Supported arches:</summary>
	///
	/// * `x64`
	/// * `aarch64`
	/// </details>
	///
	/// <details><summary>Supported features:</summary>
	///
	/// </details>
	///
	/// <details><summary>Supported versions:</summary>
	///
	/// * `26`
	/// * `25`
	/// * `21`
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
	///
	/// <details><summary>Supported versions:</summary>
	///
	/// * `25`
	/// * `21`
	/// * `17`
	/// * `11`
	/// * `8`
	/// </details>
	Adoptium,
	/// GraalVM – https://www.graalvm.org/
	///
	/// <details><summary>Supported arches:</summary>
	///
	/// * `x64`
	/// * `aarch64`
	/// </details>
	///
	/// <details><summary>Supported features:</summary>
	///
	/// </details>
	///
	/// <details><summary>Supported versions:</summary>
	///
	/// * `25`
	/// * `21`
	/// </details>
	GraalVM,
}

#[automatically_derived]
impl ValueEnum for Vendor {

	fn value_variants<'a>() -> &'a [Self] {
		&[Self::Auto, Self::JBR, Self::Oracle, Self::Adoptium, Self::GraalVM]
	}

	fn to_possible_value<'a>(&self) -> Option<PossibleValue> {
		match self {
			Self::Auto => PossibleValue::new("auto")
				.help("Automagically pick the best vendor based on the requested version and features")
				.into(),
			Self::JBR => PossibleValue::new("jbr")
				.help("JetBrains Runtime – https://github.com/JetBrains/JetBrainsRuntime/\nSupported arches:\n - x64\n - aarch64\n\nSupported features:\n - Minimal\n - JCEF\n - MUSL\n\nSupported versions:\n - 25\n - 21\n - 17\n")
				.into(),
			Self::Oracle => PossibleValue::new("oracle")
				.help("What is wrong with you?  Seriously, don't use this! – https://www.oracle.com/java/\nSupported arches:\n - x64\n - aarch64\n\nSupported versions:\n - 26\n - 25\n - 21\n")
				.into(),
			Self::Adoptium => PossibleValue::new("adoptium")
				.help("Eclipse Adoptium (previously AdoptOpenJDK) – https://adoptium.net/\nSupported arches:\n - x64\n - aarch64\n - riscv64\n\nSupported features:\n - Minimal\n\nSupported versions:\n - 25\n - 21\n - 17\n - 11\n - 8\n")
				.into(),
			Self::GraalVM => PossibleValue::new("graal-vm")
				.help("GraalVM – https://www.graalvm.org/\nSupported arches:\n - x64\n - aarch64\n\nSupported versions:\n - 25\n - 21")
				.into(),
		}
	}
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