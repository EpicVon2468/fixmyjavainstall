//! An enumeration of CPU architectures.
//!
//! The static constant [`Arch::SYSTEM`] may provide the host architecture on some targets.
use clap::ValueEnum;

use crate::fuji_value_enum;

/// An enumeration of CPU architectures.
#[non_exhaustive]
#[derive(ValueEnum, Clone)]
pub enum Arch {
	/// 64-bit extensions for x86 – <https://docs.amd.com/v/u/en-US/40332_4.09_APM_PUB>.
	#[value(aliases = vec!["amd", "amd64", "intel", "intel64", "x86_64", "x86-64"])]
	X64,
	/// 64-bit ARM – <https://developer.arm.com/Architectures/A64%20Instruction%20Set%20Architecture/>.
	#[value(aliases = vec!["arm", "arm64", "aarch"])]
	Aarch64,
	/// 64-bit RISC-V – <https://docs.riscv.org/reference/isa/index.html>.
	#[value(aliases = vec!["riscv", "risc-v"])]
	Riscv64,
}

impl Arch {
	/// The [`Arch`] of the host – `x64`.
	#[cfg(target_arch = "x86_64")]
	pub const SYSTEM: Self = Self::X64;
	/// The [`Arch`] of the host – `aarch64`.
	#[cfg(any(target_arch = "aarch64", target_arch = "arm"))]
	pub const SYSTEM: Self = Self::Aarch64;
	/// The [`Arch`] of the host – `riscv64`.
	#[cfg(target_arch = "riscv64")]
	pub const SYSTEM: Self = Self::Riscv64;
	/// The [`Arch`] of the host – Unsupported, panic!
	#[cfg(all(
		not(target_arch = "x86_64"),
		not(target_arch = "aarch64"),
		not(target_arch = "arm"),
		not(target_arch = "riscv64"),
	))]
	pub const SYSTEM: Self = panic!("Unsupported host!");
}

fuji_value_enum!(
	Arch,
	match *self {
		Self::X64 => "x64",
		Self::Aarch64 => "aarch64",
		Self::Riscv64 => "riscv64",
	}
);
