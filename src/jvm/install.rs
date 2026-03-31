use std::fs::{create_dir_all, exists, remove_dir_all};
use std::io::Result;

use crate::cli::Cmd;
use crate::cmd_link::{cmd_link, symlink_link};
use crate::commands::{connect, io_expect};
use crate::jvm::jdk::JDK;
use crate::jvm::jdk_java_se::download_java_se;
use crate::jvm::jdk_jbr::download_jbr;
use crate::jvm::jdk_temurin::download_temurin;
use crate::jvm::manage_jvm::{JavaVersion, Op};
use crate::jvm::wrapper::{generate_wrapper, install_wrapper};
use crate::wrong_cmd;

pub fn install(op: &Op) -> Result<()> {
	let Op::Install {
		jdk,
		arch,
		features,
		include_kotlin: _include_kotlin,
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
	let java_version: JavaVersion = serde_json::from_str(json.as_str()).expect("JSON failed to parse!");
	let output_dir: String = format!("/opt/fuji/jvm/{}", java_version.major);
	let script: String = generate_wrapper(&output_dir, features);
	println!("'''\n{script}\n'''");
	if exists(&output_dir)? {
		remove_dir_all(&output_dir).expect(&io_expect(&output_dir, "remove directory"));
	};
	create_dir_all(&output_dir).expect(&io_expect(&output_dir, "create directory"));
	match jdk {
		JDK::Auto => {},
		JDK::JBR => download_jbr(arch, &java_version, features, &output_dir)?,
		JDK::JavaSE => download_java_se(arch, &java_version, features, &output_dir)?,
		JDK::Temurin => download_temurin(arch, &java_version, features, &output_dir)?,
	};
	let script_file: String = install_wrapper(script, &output_dir);
	symlink_link(&output_dir, "/opt/fuji/jvm/latest")?;
	// link all of $JAVA_HOME/bin
	cmd_link(
		&Cmd::Link {
			paths: vec![output_dir],
			link_dir: String::from("/usr/bin"),
			use_update_alternatives: false
		}
	)?;
	// overwrite /usr/bin/java to point to the script instead
	symlink_link(script_file, "/usr/bin/java")
}