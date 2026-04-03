use std::fmt::{Display, Formatter, Result};

use clap::builder::PossibleValue;
use clap::ValueEnum;

#[derive(Clone, PartialEq)]
pub enum JDK {
	/// Automagically pick the best JDK based on the requested version and features
	Auto,
	/// JetBrains Runtime by JetBrains – https://github.com/JetBrains/JetBrainsRuntime/
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
	/// Java Platform, Standard Edition by Oracle – https://www.oracle.com/java/
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
	JavaSE,
	/// Temurin (previously AdoptOpenJDK) by Eclipse/Adoptium – https://adoptium.net/
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
	/// * `24`
	/// * `23`
	/// * `22`
	/// * `21`
	/// * `20`
	/// * `19`
	/// * `18`
	/// * `17`
	/// * `16`
	/// * `11`
	/// * `8`
	/// </details>
	Temurin,
	/// Liberica by BellSoft – https://bell-sw.com/libericajdk/
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
	/// * `26`
	/// * `25`
	/// * `24`
	/// * `23`
	/// * `22`
	/// * `21`
	/// * `20`
	/// * `19`
	/// * `18`
	/// * `17`
	/// * `16`
	/// * `15`
	/// * `14`
	/// * `12`
	/// * `11`
	/// * `10`
	/// * `8`
	/// </details>
	Liberica,
}

impl ValueEnum for JDK {

	fn value_variants<'a>() -> &'a [Self] {
		&[Self::Auto, Self::JBR, Self::JavaSE, Self::Temurin, Self::Liberica]
	}

	fn to_possible_value<'a>(&self) -> Option<PossibleValue> {
		macro_rules! doc {
			($name:ident) => {
				doc!($name, '\n')
			};
    		($name:ident, $suffix:literal) => {
				PossibleValue::new(self.to_string())
					.help(concat!(
						include_str!(concat!("../../doc/jdk/", stringify!($name), ".txt")),
						$suffix
					))
			};
		}
		match self {
			Self::Auto => PossibleValue::new("auto")
				.help("Automagically pick the best JDK based on the requested version and features"),
			Self::JBR => doc!(JBR).alias("jetbrains-runtime"),
			Self::JavaSE => doc!(JavaSE),
			Self::Temurin => doc!(Temurin).alias("adoptium"),
			Self::Liberica => doc!(Liberica, ""),
		}.into()
	}
}

impl Display for JDK {

	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		write!(
			f,
			"{}",
			match self {
				JDK::Auto => "auto",
				JDK::JBR => "jbr",
				JDK::JavaSE => "java-se",
				JDK::Temurin => "temurin",
				JDK::Liberica => "liberica"
			}
		)
	}
}