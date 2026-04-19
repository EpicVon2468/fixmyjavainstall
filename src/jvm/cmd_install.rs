use std::fs::{create_dir_all, remove_dir_all, remove_file, rename};
use std::path::{Path, PathBuf};

use anyhow::{Context as _, Result};

use crate::cmd_link::{link_impl, symlink_link};
use crate::commands::io_failure;
use crate::jvm::jvm::JVM;
use crate::jvm::jvm_generic::{DownloadJVMArgs, DownloadJVMFn};
use crate::jvm::jvm_java_se::download_java_se;
use crate::jvm::jvm_jbr::download_jbr;
use crate::jvm::jvm_liberica::{download_liberica, get_liberica_download};
use crate::jvm::jvm_temurin::download_temurin;
use crate::jvm::major_version::MajorVersion;
use crate::jvm::wrapper::{gen_wrapper, install_wrapper};
use crate::jvm::{Feature, JavaVersion, Op};
use crate::os::OS;
use crate::{FUJI_DIR, wrong_cmd};

pub fn cmd_install(op: Op) -> Result<()> {
	#[rustfmt::skip]
	let Op::Install {
		jvm,
		arch,
		#[cfg(feature = "multi-os")]
		operating_system: os,
		features,
		dry_run,
		version,
	}: Op = op else {
		wrong_cmd!(cmd_install);
	};
	#[cfg(not(feature = "multi-os"))]
	let os: OS = OS::default();
	let java_version: JavaVersion = if jvm == JVM::Liberica {
		let download_uri: String = get_liberica_download(&features, &os, &arch, &version)?;
		JavaVersion {
			major: download_uri,
			specific: String::new(),
			revision: String::new(),
		}
	} else if (jvm == JVM::Temurin || jvm == JVM::JavaSE)
		&& let MajorVersion::Number(num) = version
	{
		// Temurin & Java SE both only need major version, except for LTS/Latest where we return the major version from our endpoint
		JavaVersion {
			major: num.to_string(),
			specific: String::new(),
			revision: String::new(),
		}
	} else {
		let uri: String = format!(
			"https://raw.githubusercontent.com/EpicVon2468/fixmyjavainstall/refs/heads/master/listing/jvm/{jvm}/{version}.json"
		);
		ureq::get(uri)
			.call()
			.context("No JVM was available for the provided request!")?
			.into_body()
			.read_json()
			.context("Couldn't read JVM version information!")?
	};
	// FUJI_DIR/jvm/{version}
	let java_home: &Path = &Path::new(FUJI_DIR).join("jvm").join(&java_version.major);
	if !dry_run {
		clean_java_home(java_home).context("Couldn't clean JAVA_HOME!")?;
	};
	#[allow(unreachable_patterns)]
	let download_jvm: DownloadJVMFn = match jvm {
		JVM::Auto => todo!(),
		JVM::JBR => download_jbr,
		JVM::JavaSE => download_java_se,
		JVM::Temurin => download_temurin,
		JVM::Liberica => download_liberica,
		_ => todo!("Not implemented!"),
	};
	#[rustfmt::skip]
	download_jvm(DownloadJVMArgs {
		arch,
		version: java_version,
		features: &features,
		os: os.clone(),
		java_home,
		dry_run,
	}).context("Couldn't download JVM!")?;
	// https://stackoverflow.com/questions/1997718/difference-between-java-exe-and-javaw-exe
	let is_win: bool = os == OS::Windows;
	let mut executable_suffixes: Vec<&str> = vec![""];
	if is_win {
		executable_suffixes.pop();
		// executable_suffixes.push("w");
	};
	wrap_executables(&features, dry_run, java_home, is_win, executable_suffixes)?;
	if dry_run {
		return Ok(());
	};
	// make FUJI_DIR/jvm/latest point to FUJI_DIR/jvm/{version}
	symlink_link(java_home, Path::new(FUJI_DIR).join("jvm").join("latest"))
		.context("Couldn't symbolically link FUJI_DIR/jvm/latest to current install directory!")?;
	println!("Installing {}/bin...", java_home.display());
	link_impl(java_home, "/usr/bin", false).context("Couldn't install JAVA_HOME!")?;
	println!("Done.\n");

	#[cfg(target_os = "linux")]
	crate::jvm::desktop::install_desktop_entries().context("Couldn't install .desktop entries!")?;

	Ok(())
}

fn wrap_executables(
	features: &[Feature],
	dry_run: bool,
	java_home: &Path,
	is_win: bool,
	executable_suffixes: Vec<&str>,
) -> Result<()> {
	for suffix in executable_suffixes {
		let suffix: &str = if is_win {
			// `java.exe` & `javaw.exe`
			&format!("{suffix}.exe")
		} else {
			// `java`
			suffix
		};
		// $JAVA_HOME/bin/java(w)(.exe)
		let java_executable: &Path = &java_home.join("bin").join(format!("java{suffix}"));
		println!("Writing script to {}...", java_executable.display());
		if dry_run {
			continue;
		};
		// move JAVA_HOME/bin/java(w)(.exe) to a 'backup' file so that programs which try to run JAVA_HOME/bin/java(w)(.exe) literally can't skip the run script
		rename(java_executable, java_executable.with_added_extension("bak"))
			.context("Couldn't backup java executable!")?;
		#[rustfmt::skip]
		let script_file: PathBuf = install_wrapper(
			gen_wrapper(java_home, features, is_win, suffix).as_str(),
			java_home,
			suffix,
			is_win,
		).context("Couldn't install JVM wrapper script!")?;
		// link JAVA_HOME/bin/java(w)(.exe) to JAVA_HOME/bin/fuji_jvm_wrapper
		symlink_link(script_file, java_executable).context(
			"Couldn't symbolically link JAVA_HOME/bin/java to point to JAVA_HOME/bin/fuji_jvm_wrapper!",
		)?;
		println!("Done.\n");
	}
	Ok(())
}

fn clean_java_home(java_home: &Path) -> Result<()> {
	if java_home.exists() {
		#[rustfmt::skip]
		let result: Result<()> = if java_home.is_dir() {
			remove_dir_all(java_home)
		} else {
			remove_file(java_home)
		}.with_context(|| io_failure(java_home, "remove"));
		result.context("Couldn't remove entry which was occupying the new JAVA_HOME!")?;
	};
	create_dir_all(java_home).with_context(|| io_failure(java_home, "create directory"))
}
