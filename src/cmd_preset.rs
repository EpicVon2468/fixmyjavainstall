use std::io::Result;

use crate::cli::{Cmd, Preset, Software};
use crate::cmd_manage::cmd_manage;
use crate::jvm::jdk::JDK;
use crate::jvm::major_version::MajorVersion;
use crate::jvm::manage_jvm::{Feature, Op};
use crate::wrong_cmd;

pub fn cmd_preset(cmd: Cmd) -> Result<()> {
	let Cmd::Preset {
		preset,
	} = cmd else {
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
	cmd_manage(Cmd::Manage {
		software: Software::JVM {
			op: Op::Install {
				jdk: JDK::JBR,
				arch: crate::arch::SYSTEM,
				#[cfg(feature = "multi_os")]
				operating_system: crate::os::SYSTEM,
				features,
				include_kotlin: true,
				dry_run: false,
				version: MajorVersion::LTS,
			},
		}.into()
	})
}

fn preset_fast(minimal: bool) -> Result<()> {
	let mut features: Vec<Feature> = features(minimal);
	configure_fast(&mut features)?;
	cmd_manage(Cmd::Manage {
		software: Software::JVM {
			op: Op::Install {
				jdk: JDK::JBR,
				arch: crate::arch::SYSTEM,
				#[cfg(feature = "multi_os")]
				operating_system: crate::os::SYSTEM,
				features,
				include_kotlin: false,
				dry_run: false,
				version: MajorVersion::LTS,
			},
		}.into(),
	})
}

fn configure_fast(features: &mut Vec<Feature>) -> Result<()> {
	features.push(Feature::JEP519);
	#[cfg(target_os = "linux")] {
		use std::fs::DirEntry;
		use std::path::Path;
		use std::env::var;

		if Path::new("/proc/driver").read_dir()?.any(|entry: Result<DirEntry>| {
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

		if var("XDG_SESSION_TYPE").unwrap().to_ascii_lowercase().contains("wayland") {
			// WLToolkit also enables Vulkan
			features.push(Feature::WLToolkit);
		} else {
			features.push(Feature::OpenGL);
		};
	};
	#[cfg(target_os = "macos")]
	features.push(Feature::Metal);
	#[cfg(windows)]
	features.push(Feature::OpenGL);
	Ok(())
}

fn preset_latest(minimal: bool) -> Result<()> {
	cmd_manage(Cmd::Manage {
		software: Software::JVM {
			op: Op::Install {
				// FIXME: need to implement JDK::Auto so I don't have to default to JavaSE
				jdk: JDK::JavaSE,
				arch: crate::arch::SYSTEM,
				#[cfg(feature = "multi_os")]
				operating_system: crate::os::SYSTEM,
				features: features(minimal),
				include_kotlin: false,
				dry_run: false,
				version: MajorVersion::Latest,
			},
		}.into(),
	})
}