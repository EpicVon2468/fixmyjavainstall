use std::fs::{File, OpenOptions, Permissions};
use std::io::Write;
use std::os::unix::fs::PermissionsExt;

use crate::commands::io_expect;
use crate::jvm::manage_jvm::Feature;

pub fn generate_wrapper(features: &Vec<Feature>) -> String {
	let mut result: String = String::with_capacity(500);
	result.push_str("#! /usr/bin/env sh\n\n");
	macro_rules! additional_jvm_args {
    	($comment:literal, $addition:literal) => {
			result.push_str(
				concat!("# ", $comment, '\n', "ADDITIONAL_JVM_ARGS=\"", $addition, " $ADDITIONAL_JVM_ARGS\"", '\n', '\n')
			);
		};
	}
	if features.contains(&Feature::DCEVM) {
		additional_jvm_args!(
			"Dynamic Code Evolution Virtual Machine (enhanced runtime class redefinition) – https://ssw.jku.at/dcevm/",
			"-XX:+AllowEnhancedClassRedefinition"
		);
	};
	if features.contains(&Feature::JEP519) {
		additional_jvm_args!(
			"JDK Enhancement Proposal 519 (Compact Object Headers) – https://openjdk.org/jeps/519",
			"-XX:+UseCompactObjectHeaders"
		);
	};
	let mut requires_vulkan: bool = false;
	if features.contains(&Feature::WLToolkit) {
		additional_jvm_args!(
			"Wayland support (requires Vulkan) – https://wiki.openjdk.org/spaces/wakefield/pages/77693134/Pure+Wayland+toolkit+prototype",
			"-Dawt.tookit.name=WLToolkit"
		);
		requires_vulkan = true;
	};
	if features.contains(&Feature::OpenGL) {
		if requires_vulkan {
			panic!("Vulkan required for WLToolkit, but OpenGL was also explicitly requested.  Resolve incompatible args and try again.");
		};
		additional_jvm_args!(
			"OpenGL for AWT/Swing.  This has been bundled in OpenJDK for a long time, but isn't on by default",
			"-Dsun.java2d.opengl=true"
		);
	};
	if requires_vulkan || features.contains(&Feature::Vulkan) {
		additional_jvm_args!(
			"Vulkan for AWT/Swing",
			"-Dsun.java2d.vulkan=true -Dsun.java2d.vulkan.accelsd=false"
		);
	};
	result.push_str("\
# shellcheck disable=SC2155
export JAVA_HOME=\"$(realpath \"..\")\"\n\n\
	");
	result.push_str("exec \"$PWD/java\" \"$ADDITIONAL_JVM_ARGS\" \"$@\"");
	result
}

pub fn install_wrapper(script: String, output_dir: &String) -> String {
	let script_file: String = format!("{output_dir}/bin/fuji_jvm_wrapper");
	let mut result: File = OpenOptions::new()
		.write(true)
		.create_new(true)
		.open(&script_file)
		.expect(io_expect(&script_file, "create").as_str());
	result
		.write_all(script.as_bytes())
		.expect(io_expect(&script_file, "write").as_str());
	// rwxr-xr-x
	result
		.set_permissions(Permissions::from_mode(0o755))
		.expect(io_expect(&script_file, "set permissions for").as_str());
	script_file
}