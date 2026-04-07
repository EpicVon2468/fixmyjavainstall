use std::fs::{create_dir_all, exists, remove_dir_all, rename};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::cmd_link::{link_impl, symlink_link};
use crate::commands::io_failure;
use crate::jvm::jdk::JDK;
use crate::jvm::jdk_generic::{DownloadJDKArgs, DownloadJDKFn};
use crate::jvm::jdk_java_se::download_java_se;
use crate::jvm::jdk_jbr::download_jbr;
use crate::jvm::jdk_liberica::download_liberica;
use crate::jvm::jdk_temurin::download_temurin;
use crate::jvm::major_version::MajorVersion;
use crate::jvm::manage_jvm::{JavaVersion, Op};
use crate::jvm::wrapper::{generate_wrapper, install_wrapper};
use crate::os::OS;
use crate::{FUJI_DIR, wrong_cmd};

pub fn install(op: Op) -> Result<()> {
	let Op::Install {
		jdk,
		arch,
		#[cfg(feature = "multi_os")]
		operating_system,
		features,
		include_kotlin: _include_kotlin,
		dry_run,
		version,
	}: Op = op else {
		wrong_cmd!(install);
	};
	#[cfg(not(feature = "multi_os"))]
	let operating_system: OS = crate::os::SYSTEM;
	// Temurin & Java SE both only need major version, except for LTS/Latest where we return the major version from our endpoint
	let java_version: JavaVersion = if (jdk == JDK::Temurin || jdk == JDK::JavaSE) && let MajorVersion::Number(version) = version {
		JavaVersion {
			major: version.to_string(),
			specific: "".into(),
			revision: "".into(),
		}
	} else {
		let uri: String = format!(
			"https://raw.githubusercontent.com/EpicVon2468/fixmyjavainstall/refs/heads/master/listing/jvm/{jdk}/{version}.json"
		);
		ureq::get(uri)
			.call()
			.context("Couldn't connect to URL!")?
			.body_mut()
			.read_json()
			.context("Couldn't parse JSON from response!")?
	};
	// FUJI_DIR/jvm/{version}
	let java_home: &Path = &Path::new(FUJI_DIR).join("jvm").join(&java_version.major);
	if !dry_run {
		if exists(java_home)? {
			remove_dir_all(java_home)
				.with_context(|| io_failure(java_home, "remove directory"))?;
		};
		create_dir_all(java_home)
			.with_context(|| io_failure(java_home, "create directory"))?;
	};
	let download_jdk: DownloadJDKFn = match jdk {
		JDK::Auto => todo!(),
		JDK::JBR => download_jbr,
		JDK::JavaSE => download_java_se,
		JDK::Temurin => download_temurin,
		JDK::Liberica => download_liberica,
	};
	download_jdk(DownloadJDKArgs {
		arch,
		version: java_version,
		features: &features,
		os: operating_system.clone(),
		java_home,
		dry_run,
	}).context("Couldn't download JDK!")?;
	// https://stackoverflow.com/questions/1997718/difference-between-java-exe-and-javaw-exe
	let is_win: bool = operating_system == OS::Windows;
	let mut executable_suffixes: Vec<&str> = vec![""];
	if is_win {
		executable_suffixes.push("w");
	};
	for suffix in executable_suffixes {
		let suffix: &str = if is_win {
			// `java.exe` & `javaw.exe`
			&format!("{suffix}.exe")
		} else {
			// `java`
			suffix
		};
		// $JAVA_HOME/bin/java(w)(.exe)
		let java_executable: PathBuf = java_home.join("bin").join(format!("java{suffix}"));
		println!("Writing script to {}...", java_executable.display());
		if dry_run {
			continue;
		};
		// move JAVA_HOME/bin/java(w)(.exe) to a 'backup' file so that programs which try to run JAVA_HOME/bin/java(w)(.exe) literally can't skip the run script
		rename(
			&java_executable,
			java_executable.with_added_extension("bak"),
		).context("Couldn't backup java executable!")?;
		let script_file: PathBuf = install_wrapper(
			generate_wrapper(java_home, &features, is_win, suffix),
			java_home,
			suffix,
			is_win,
		).context("Couldn't install JVM wrapper script!")?;
		// link JAVA_HOME/bin/java(w)(.exe) to JAVA_HOME/bin/fuji_jvm_wrapper
		symlink_link(script_file, java_executable).context(
			"Couldn't symbolically link JAVA_HOME/bin/java to point to JAVA_HOME/bin/fuji_jvm_wrapper!",
		)?;
	}
	if dry_run {
		return Ok(());
	};
	// make FUJI_DIR/jvm/latest point to FUJI_DIR/jvm/{version}
	symlink_link(java_home, Path::new(FUJI_DIR).join("jvm").join("latest"))
		.context("Couldn't symbolically link FUJI_DIR/jvm/latest to current install directory!")?;
	println!("Installing {}/bin...", java_home.display());
	link_impl(java_home, "/usr/bin", false)
		.context("Couldn't install JAVA_HOME!")?;
	println!("Done.\n");
	#[cfg(target_os = "linux")] {
		use std::fs::File;
		use std::io::Write;

		let base: &Path = Path::new("/usr/share/applications");
		if !base.exists() {
			return Ok(());
		};
		macro_rules! desktop_entry {
			($output:literal, $ident:ident) => {
				File::create(base.join($output))
					.context(concat!(
						"Couldn't create/write '/usr/share/applications/",
						$output,
						"'!"
					))?
					.write_all($crate::jvm::desktop::$ident.as_bytes())
					.context(concat!(
						"Couldn't write to '/usr/share/applications/",
						$output,
						"'!"
					))?
			};
		}
		desktop_entry!("fuji.java.desktop", FREEDESKTOP_ENTRY);
		desktop_entry!("fuji.java.terminal.desktop", FREEDESKTOP_ENTRY_TERMINAL)
	};
	Ok(())
}