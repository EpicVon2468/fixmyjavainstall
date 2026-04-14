use anyhow::{Context as _, Result};

use crate::arch::Arch;
use crate::cli::{FujiCmd, Preset, Software};
use crate::cmd_manage::cmd_manage;
use crate::jvm::jvm::JVM;
use crate::jvm::major_version::MajorVersion;
use crate::jvm::{Feature, Op};
#[cfg(feature = "multi-os")]
use crate::os::OS;
use crate::wrong_cmd;

pub fn cmd_preset(op: Op) -> Result<()> {
	let Op::Preset { preset }: Op = op else {
		wrong_cmd!(cmd_preset);
	};
	match preset {
		Preset::RecommendedJRE => preset_recommended(true),
		Preset::RecommendedJDK => preset_recommended(false),
		Preset::FastJRE => preset_fast(true),
		Preset::FastJDK => preset_fast(false),
		Preset::LatestJRE => preset_latest(true),
		Preset::LatestJDK => preset_latest(false),
	}
}

fn features(minimal: bool) -> Vec<Feature> {
	if minimal {
		vec![Feature::Minimal]
	} else {
		vec![]
	}
}

fn preset_recommended(minimal: bool) -> Result<()> {
	let mut features: Vec<Feature> = features(minimal);
	if minimal {
		features.push(Feature::Minimal);
	};
	configure_fast(&mut features)?;
	features.push(Feature::AllowNative);
	features.push(Feature::AllowUnsafe);
	features.push(Feature::FontAntiAliasing);
	cmd_manage(FujiCmd::Manage {
		software: Software::JVM {
			op: Op::Install {
				jvm: JVM::JBR,
				arch: Arch::SYSTEM,
				#[cfg(feature = "multi-os")]
				operating_system: OS::SYSTEM,
				features,
				include_kotlin: true,
				dry_run: false,
				version: MajorVersion::LTS,
			},
		},
	})
}

fn preset_fast(minimal: bool) -> Result<()> {
	let mut features: Vec<Feature> = features(minimal);
	configure_fast(&mut features)?;
	cmd_manage(FujiCmd::Manage {
		software: Software::JVM {
			op: Op::Install {
				jvm: JVM::JBR,
				arch: Arch::SYSTEM,
				#[cfg(feature = "multi-os")]
				operating_system: OS::SYSTEM,
				features,
				include_kotlin: false,
				dry_run: false,
				version: MajorVersion::LTS,
			},
		},
	})
}

fn configure_fast(features: &mut Vec<Feature>) -> Result<()> {
	features.push(Feature::JEP519);
	#[cfg(target_os = "linux")]
	{
		use std::env::var;
		use std::fs::{DirEntry, ReadDir};
		use std::path::Path;

		use crate::commands::io_failure;

		let path: &str = "/proc/driver";
		let mut dir: ReadDir = Path::new(path)
			.read_dir()
			.with_context(|| io_failure(path, "list directory"))?;
		if dir.any(|entry: std::io::Result<DirEntry>| {
			entry
				.expect("Couldn't check for NVIDIA drivers!")
				.file_name()
				.to_ascii_uppercase()
				.to_str()
				.unwrap()
				.contains("NVIDIA")
		}) {
			features.push(Feature::NVIDIAFixes);
		};

		// WLToolkit also enables Vulkan
		features.push(if var("WAYLAND_DISPLAY").is_ok() {
			Feature::WLToolkit
		} else {
			Feature::OpenGL
		});
	};
	#[cfg(target_env = "musl")]
	features.push(Feature::MUSL);
	#[cfg(target_os = "macos")]
	features.push(Feature::Metal);
	#[cfg(windows)]
	features.push(Feature::OpenGL);
	Ok(())
}

fn preset_latest(minimal: bool) -> Result<()> {
	cmd_manage(FujiCmd::Manage {
		software: Software::JVM {
			op: Op::Install {
				// FIXME: need to implement JVM::Auto so I don't have to default to JavaSE
				jvm: JVM::JavaSE,
				arch: Arch::SYSTEM,
				#[cfg(feature = "multi-os")]
				operating_system: OS::SYSTEM,
				features: features(minimal),
				include_kotlin: false,
				dry_run: false,
				version: MajorVersion::Latest,
			},
		},
	})
}
