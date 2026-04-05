use std::fs::{create_dir_all, exists, remove_dir_all, rename};
use std::io::Result;
use std::path::MAIN_SEPARATOR;

use crate::cmd_link::{link_impl, symlink_link};
use crate::commands::{connect, io_expect};
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
	} = op else {
		wrong_cmd!(install);
	};
	#[cfg(not(feature = "multi_os"))]
	let operating_system: OS = crate::os::SYSTEM;
	// Temurin & Java SE both only need major version, except for LTS/Latest where we return the major version from our endpoint
	let json: String = if (jdk == JDK::Temurin || jdk == JDK::JavaSE) && let MajorVersion::Number(version) = version {
		format!(r#"{{"major": "{version}", "specific":"", "revision": ""}}"#)
	} else {
		connect(format!(
			"https://raw.githubusercontent.com/EpicVon2468/fixmyjavainstall/refs/heads/master/listing/jvm/{}/{}.json",
			jdk, version
		))?
	};
	let java_version: JavaVersion = serde_json::from_str(&json).expect("JSON failed to parse!");
	// FUJI_DIR/jvm/{version}
	let java_home: &str = &format!(
		"{FUJI_DIR}{MAIN_SEPARATOR}jvm{MAIN_SEPARATOR}{}",
		java_version.major
	);
	if !dry_run {
		if exists(java_home)? {
			remove_dir_all(java_home)
				.unwrap_or_else(|_| panic!("{}", io_expect(java_home, "remove directory")));
		};
		create_dir_all(java_home)
			.unwrap_or_else(|_| panic!("{}", io_expect(java_home, "create directory")));
	};
	let is_win: bool = operating_system == OS::Windows;
	let is_mac: bool = operating_system == OS::OSX;
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
		os: operating_system,
		java_home,
		dry_run,
	})?;
	println!();
	// https://stackoverflow.com/questions/1997718/difference-between-java-exe-and-javaw-exe
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
		let java_executable: String = format!("{java_home}{MAIN_SEPARATOR}bin{MAIN_SEPARATOR}java{suffix}");
		let script: String = generate_wrapper(java_home, &features, is_win, suffix);
		println!("Writing script to {java_executable}...");
		println!("'''\n{script}\n'''");
		println!();
		if dry_run {
			continue;
		};
		// move $JAVA_HOME/bin/java(w)(.exe) to a 'backup' file so that programs which try to run $JAVA_HOME/bin/java(w)(.exe) literally can't skip the run script
		rename(&java_executable, format!("{java_executable}.bak"))?;
		let script_file: String = install_wrapper(script, java_home, suffix, is_win);
		// link $JAVA_HOME/bin/java(w)(.exe) to $JAVA_HOME/bin/fuji_jvm_wrapper
		symlink_link(&script_file, java_executable)?;
	};
	if dry_run {
		return Ok(());
	};
	// make FUJI_DIR/jvm/latest point to output_dir
	#[cfg(not(windows))]
	symlink_link(
		java_home,
		format!("{FUJI_DIR}{MAIN_SEPARATOR}jvm{MAIN_SEPARATOR}latest"),
	)?;
	// link all of $JAVA_HOME/bin
	link_impl(
		java_home,
		if is_mac {
			"/usr/local/bin"
		} else {
			"/usr/bin"
		},
		false,
	)
}