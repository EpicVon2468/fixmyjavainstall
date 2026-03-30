use std::fmt::{Display, Formatter};

use clap::builder::PossibleValue;
use clap::ValueEnum;

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
	/// * `DCEVM`
	/// * `WLToolkit`
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

impl ValueEnum for Vendor {

	fn value_variants<'a>() -> &'a [Self] {
		&[Self::Auto, Self::JBR, Self::Oracle, Self::Adoptium, Self::GraalVM]
	}

	fn to_possible_value<'a>(&self) -> Option<PossibleValue> {
		macro_rules! doc {
			($name:ident) => {
				doc!($name, '\n')
			};
    		($name:ident, $suffix:literal) => {
				PossibleValue::new(self.to_string())
					.help(concat!(
						include_str!(concat!("../../doc/vendor/", stringify!($name), ".txt")),
						$suffix
					))
					.into()
			};
		}
		match self {
			Self::Auto => PossibleValue::new("auto")
				.help("Automagically pick the best vendor based on the requested version and features")
				.into(),
			Self::JBR => doc!(JBR),
			Self::Oracle => doc!(Oracle),
			Self::Adoptium => doc!(Adoptium),
			Self::GraalVM => doc!(GraalVM, ""),
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
				Vendor::GraalVM => "graalvm",
			}
		)
	}
}