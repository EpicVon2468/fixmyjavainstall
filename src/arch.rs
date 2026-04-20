//! An enumeration of CPU architectures.
//!
//! The trait [`Default`] may provide the host architecture on some targets.
use clap::ValueEnum;

use crate::value_enum_extensions;

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

value_enum_extensions!(
	Arch,
	cfg_select! {
		target_arch = "x86_64" => Self::X64,
		any(target_arch = "aarch64", target_arch = "arm") => Self::Aarch64,
		target_arch = "riscv64" => Self::Riscv64,
		_ => panic!("Unsupported host!"),
	},
	match *self {
		Self::X64 => "x64",
		Self::Aarch64 => "aarch64",
		Self::Riscv64 => "riscv64",
	},
);
