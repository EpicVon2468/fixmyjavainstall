use std::io::Result;

use crate::commands::connect;
use crate::jvm::jdk_jbr::download_jbr;
use crate::jvm::manage_jvm::{JavaVersion, Op};
use crate::jvm::wrapper::generate_wrapper;
use crate::wrong_cmd;

pub fn install(op: &Op) -> Result<()> {
	let Op::Install {
		jdk,
		arch,
		features,
		version,
	} = op else {
		wrong_cmd!(install);
	};
	let script: String = generate_wrapper(features);
	println!("'''\n{script}\n'''");
	if true {
		let json: String = connect(
			format!(
				"https://raw.githubusercontent.com/EpicVon2468/fixmyjavainstall/refs/heads/master/listing/jvm/{}/{}.json",
				jdk,
				version
			)
		)?;
		let java_version: JavaVersion = serde_json::from_str(json.as_str()).expect("JSON failed to parse!");
		download_jbr(arch, &java_version, features)?;
	};
	Ok(())
}