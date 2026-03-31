use std::fs::{create_dir_all, exists, remove_dir_all, File, OpenOptions, Permissions};
use std::io::{Result, Write};
use std::os::unix::fs::PermissionsExt;

use crate::commands::{connect, io_expect};
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
	let json: String = connect(
		format!(
			"https://raw.githubusercontent.com/EpicVon2468/fixmyjavainstall/refs/heads/master/listing/jvm/{}/{}.json",
			jdk,
			version
		)
	)?;
	let java_version: JavaVersion = serde_json::from_str(json.as_str()).expect("JSON failed to parse!");
	let output_dir: &String = &format!("/opt/fuji/jvm/{}", java_version.major);
	if exists(output_dir)? {
		remove_dir_all(output_dir).expect(
			io_expect(output_dir, "remove directory").as_str()
		);
	};
	create_dir_all(output_dir).expect(
		io_expect(output_dir, "create directory").as_str()
	);
	download_jbr(arch, &java_version, features, output_dir)?;
	// return Ok(());
	let script_file: &String = &format!("{output_dir}/bin/fuji_jvm_wrapper");
	let mut result: File = OpenOptions::new()
		.write(true)
		.create_new(true)
		.open(script_file)
		.expect(io_expect(script_file, "create").as_str());
	result
		.write_all(script.as_bytes())
		.expect(io_expect(script_file, "write").as_str());
	// rwxr-xr-x
	result
		.set_permissions(Permissions::from_mode(0o755))
		.expect(io_expect(script_file, "set permissions for").as_str());
	Ok(())
}