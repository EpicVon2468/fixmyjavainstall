//! An enumeration of CPU architectures.
//!
//! The static constant [`Arch::SYSTEM`] may provide the host architecture on some targets.
use std::fmt::{Display, Formatter, Result};

use clap::ValueEnum;
use clap::builder::OsStr;

/// An enumeration of CPU architectures
#[derive(ValueEnum, Clone)]
pub enum Arch {
	/// 64-bit (amd64)
	X64,
	/// 64-bit AArch (arm64) – <https://developer.arm.com/Architectures/A64%20Instruction%20Set%20Architecture/>
	Aarch64,
	/// 64-bit RISC-V – <https://riscv.org/>
	Riscv64,
}

impl Arch {

	/// The [`Arch`] of the host – amd64.
	#[cfg(target_arch = "x86_64")]
	pub const SYSTEM: Arch = Arch::X64;
	/// The [`Arch`] of the host – arm64.
	#[cfg(any(target_arch = "aarch64", target_arch = "arm"))]
	pub const SYSTEM: Arch = Arch::Aarch64;
	/// The [`Arch`] of the host – aarch64.
	#[cfg(target_arch = "riscv64")]
	pub const SYSTEM: Arch = Arch::Riscv64;
	/// The [`Arch`] of the host – Unsupported, panic!
	#[cfg(all(
		not(target_arch = "x86_64"),
		not(target_arch = "aarch64"),
		not(target_arch = "arm"),
		not(target_arch = "riscv64"),
	))]
	pub const SYSTEM: Arch = panic!("Unsupported host architecture!");
}

impl From<Arch> for OsStr {

	fn from(value: Arch) -> Self {
		value.to_string().into()
	}
}

impl Display for Arch {

	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		write!(
			f,
			"{}",
			match self {
				Arch::X64 => "x64",
				Arch::Aarch64 => "aarch64",
				Arch::Riscv64 => "riscv64",
			}
		)
	}
}