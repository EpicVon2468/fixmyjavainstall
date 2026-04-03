use std::fs::{create_dir_all, exists, remove_dir_all, rename};
use std::io::Result;
use std::path::MAIN_SEPARATOR;

use crate::cli::Cmd;
use crate::cmd_link::{cmd_link, symlink_link};
use crate::commands::{connect, io_expect};
use crate::jvm::jdk::JDK;
use crate::jvm::jdk_generic::DownloadJdkFn;
use crate::jvm::jdk_java_se::download_java_se;
use crate::jvm::jdk_jbr::download_jbr;
use crate::jvm::jdk_liberica::download_liberica;
use crate::jvm::jdk_temurin::download_temurin;
use crate::jvm::manage_jvm::{JavaVersion, Op};
use crate::jvm::wrapper::{generate_wrapper, install_wrapper};
use crate::{wrong_cmd, FUJI_DIR};
use crate::os::OS;

pub fn install(op: &Op) -> Result<()> {
	let Op::Install {
		jdk,
		arch,
		operating_system,
		features,
		include_kotlin: _include_kotlin,
		dry_run,
		version,
	} = op else {
		wrong_cmd!(install);
	};
	// Temurin & Java SE both only need major version, except for 'latest' where we return the latest major from our endpoint
	let json: String = if (jdk == &JDK::Temurin || jdk == &JDK::JavaSE) && version != "latest" {
		format!("{{\"major\": \"{version}\", \"specific\":\"\", \"revision\": \"\"}}")
	} else {
		connect(
			format!(
				"https://raw.githubusercontent.com/EpicVon2468/fixmyjavainstall/refs/heads/master/listing/jvm/{}/{}.json",
				jdk,
				version
			)
		)?
	};
	let java_version: JavaVersion = serde_json::from_str(&json).expect("JSON failed to parse!");
	// FUJI_DIR/jvm/{version}
	let output_dir: &str = &format!("{FUJI_DIR}{MAIN_SEPARATOR}jvm{MAIN_SEPARATOR}{}", java_version.major);
	if !dry_run {
		if exists(output_dir)? {
			remove_dir_all(output_dir).expect(&io_expect(output_dir, "remove directory"));
		};
		create_dir_all(output_dir).expect(&io_expect(output_dir, "create directory"));
	};
	let download_jdk: DownloadJdkFn = match jdk {
		JDK::Auto => todo!(),
		JDK::JBR => download_jbr,
		JDK::JavaSE => download_java_se,
		JDK::Temurin => download_temurin,
		JDK::Liberica => download_liberica,
	};
	download_jdk(arch, java_version, features, operating_system, output_dir, dry_run)?;
	println!();
	// https://stackoverflow.com/questions/1997718/difference-between-java-exe-and-javaw-exe
	let is_win: bool = operating_system == &OS::Windows;
	let mut executable_suffixes: Vec<&str> = vec![""];
	if is_win {
		executable_suffixes.push("w");
	};
	for suffix in executable_suffixes {
		let suffix: &str = if is_win {
			&format!("{suffix}.exe")
		} else {
			suffix
		};
		// $JAVA_HOME/bin/java(w)(.exe)
		let java_executable: String = format!("{output_dir}{MAIN_SEPARATOR}bin{MAIN_SEPARATOR}java{suffix}");
		let script: String = generate_wrapper(output_dir, features, is_win, suffix);
		println!("Writing script to {java_executable}...");
		println!("'''\n{script}\n'''");
		println!();
		if *dry_run {
			continue;
		};
		// move $JAVA_HOME/bin/java(w)(.exe) to a 'backup' file so that programs which try to run $JAVA_HOME/bin/java(w)(.exe) literally can't skip the run script
		rename(&java_executable, format!("{java_executable}.bak"))?;
		let script_file = install_wrapper(script, output_dir, suffix);
		// link $JAVA_HOME/bin/java(w)(.exe) to $JAVA_HOME/bin/fuji_jvm_wrapper
		symlink_link(&script_file, java_executable)?;
	};
	if *dry_run {
		return Ok(());
	};
	// make FUJI_DIR/jvm/latest point to output_dir
	symlink_link(
		output_dir,
		format!("{FUJI_DIR}{MAIN_SEPARATOR}jvm{MAIN_SEPARATOR}latest")
	)?;
	// link all of $JAVA_HOME/bin
	cmd_link(
		&Cmd::Link {
			paths: vec![output_dir.to_string()],
			link_dir: if cfg!(windows) { "" } else { "/usr/bin" }.into(),
			use_update_alternatives: false
		}
	)
}