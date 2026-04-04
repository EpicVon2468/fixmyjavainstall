use std::env::var;
use std::fs::DirEntry;
use std::io::Result;

use crate::arch::Arch;
use crate::cli::{Cmd, Preset, Software};
use crate::cmd_manage::cmd_manage;
use crate::jvm::jdk::JDK;
use crate::jvm::major_version::MajorVersion;
use crate::jvm::manage_jvm::{Feature, Op};
use crate::os::OS;
use crate::wrong_cmd;

pub fn cmd_preset(cmd: Cmd) -> Result<()> {
	let Cmd::Preset {
		preset,
	} = cmd else {
		wrong_cmd!(cmd_preset);
	};
	match preset {
		Preset::FastJRE => preset_fast(true),
		Preset::FastJDK => preset_fast(false),
		Preset::LatestJRE => preset_latest(true),
		Preset::LatestJDK => preset_latest(false),
	}
}

fn preset_fast(minimal: bool) -> Result<()> {
	let arch: Arch = crate::arch::SYSTEM;
	let os: OS = crate::os::SYSTEM;
	let mut features: Vec<Feature> = vec![Feature::JEP519];
	if minimal {
		features.push(Feature::Minimal);
	};
	#[cfg(target_os = "linux")] {
		use std::path::Path;
		let path: &Path = Path::new("");
		if path.read_dir()?.any(|entry: Result<DirEntry>| {
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
	cmd_manage(
		Cmd::Manage {
			software: Software::JVM {
				op: Op::Install {
					jdk: JDK::JBR,
					arch,
					operating_system: os,
					features: vec![],
					include_kotlin: false,
					dry_run: false,
					version: MajorVersion::LTS,
				},
			}.into()
		}
	)
}

fn preset_latest(minimal: bool) -> Result<()> {
	cmd_manage(
		Cmd::Manage {
			software: Software::JVM {
				op: Op::Install {
					// FIXME: need to implement JDK::Auto so I don't have to default to JavaSE
					jdk: JDK::JavaSE,
					arch: crate::arch::SYSTEM,
					operating_system: crate::os::SYSTEM,
					features: if minimal {
						vec![Feature::Minimal]
					} else {
						vec![]
					},
					include_kotlin: false,
					dry_run: false,
					version: MajorVersion::Latest,
				},
			}.into(),
		}
	)
}