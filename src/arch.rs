use std::env::consts::ARCH;
use std::fmt::{Display, Formatter};

use clap::ValueEnum;

#[derive(ValueEnum, Clone)]
pub enum Arch {
	/// Automagically determine the system architecture
	System,
	/// 64-bit (amd64)
	X64,
	/// 64-bit AArch (arm64) – https://developer.arm.com/Architectures/A64%20Instruction%20Set%20Architecture/
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