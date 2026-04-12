//! An enumeration of JDKs
use std::fmt::{Display, Formatter, Result};

use clap::ValueEnum;

macro_rules! clap_doc {
	($name:ident) => {
		clap_doc!($name, '\n')
	};
	($name:ident, $suffix:literal) => {
		concat!(
			include_str!(concat!("../../doc/jdk/", stringify!($name), ".txt")),
			$suffix
		)
	};
}

/// An enumeration of JDKs
#[derive(ValueEnum, Clone, PartialEq)]
pub enum JDK {
	/// Automagically pick the best JDK based on the requested version and features
	Auto,
	/// JetBrains Runtime by JetBrains – <https://github.com/JetBrains/JetBrainsRuntime/>
	///
	/// <details><summary>Supported architectures:</summary>
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
	#[clap(help = clap_doc!(JBR), alias = "jetbrains-runtime")]
	JBR,
	/// Java Platform, Standard Edition by Oracle – <https://www.oracle.com/java/>
	///
	/// <details><summary>Supported architectures:</summary>
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
	#[clap(help = clap_doc!(JavaSE))]
	JavaSE,
	/// Temurin (previously AdoptOpenJDK) by Eclipse/Adoptium – <https://adoptium.net/>
	///
	/// <details><summary>Supported architectures:</summary>
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
	/// * `11`
	/// * `8`
	/// </details>
	#[clap(help = clap_doc!(Temurin), alias = "adoptium")]
	Temurin,
	/// Liberica by BellSoft – <https://bell-sw.com/libericajdk/>
	///
	/// <details><summary>Supported architectures:</summary>
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
	#[clap(help = clap_doc!(Liberica, ""))]
	Liberica,
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
				JDK::Liberica => "liberica",
			}
		)
	}
}