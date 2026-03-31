use std::io::Result;

use crate::cli::{Cmd, Software};
use crate::commands::connect;
use crate::jvm::manage_jvm::{JavaVersion, Op};
use crate::jvm::jdk_jbr::download_jbr;
use crate::jvm::wrapper::generate_wrapper;
use crate::wrong_cmd;

pub fn cmd_manage(command: &Cmd) -> Result<()> {
	let Cmd::Manage {
		software: option,
	} = command else {
		wrong_cmd!(cmd_manage);
	};
	if let Some(software) = option {
		if let Software::JVM { op } = software {
			if let Op::Install {
				jdk: vendor,
				arch,
				features,
				version,
			} = op {
				let string = generate_wrapper(features);
				println!("{string}")
				// let json: String = connect(
				// 	format!(
				// 		"https://raw.githubusercontent.com/EpicVon2468/fixmyjavainstall/refs/heads/master/listing/jvm/{}/{}.json",
				// 		vendor,
				// 		version
				// 	)
				// )?;
				// let java_version: JavaVersion = serde_json::from_str(json.as_str()).expect("JSON failed to parse!");
				// download_jbr(arch, &java_version, features)?;
			};
		};
	};
	Ok(())
}