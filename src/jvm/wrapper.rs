use std::fs::{File, OpenOptions, Permissions};
use std::io::Write;
use std::os::unix::fs::PermissionsExt;

use crate::commands::io_expect;
use crate::jvm::manage_jvm::Feature;

pub fn generate_wrapper(java_home: &str, features: &Vec<Feature>) -> String {
	let mut result: String = String::with_capacity(500);
	result.push_str("#! /usr/bin/env sh\n\n");
	macro_rules! fuji_jvm_arg {
    	($comment:literal, $args:literal) => {
			result.push_str(
				concat!("# ", $comment, '\n', self_arg!($args), '\n')
			);
		};
	}
	macro_rules! self_arg {
    	($args:literal) => {
			concat!("set -- ", $args, " \"$@\"\n")
		};
	}
	if features.contains(&Feature::DCEVM) {
		fuji_jvm_arg!(
			"Dynamic Code Evolution Virtual Machine (enhanced runtime class redefinition) – https://ssw.jku.at/dcevm/",
			"-XX:+AllowEnhancedClassRedefinition"
		);
	};
	if features.contains(&Feature::JEP519) {
		fuji_jvm_arg!(
			"JDK Enhancement Proposal 519 (Compact Object Headers) – https://openjdk.org/jeps/519",
			"-XX:+UseCompactObjectHeaders"
		);
	};
	let mut requires_vulkan: bool = false;
	#[cfg(any(target_os = "linux", feature = "multi_os"))]
	if features.contains(&Feature::WLToolkit) {
		fuji_jvm_arg!(
			"Wayland support (requires Vulkan) – https://wiki.openjdk.org/spaces/wakefield/pages/77693134/Pure+Wayland+toolkit+prototype",
			"-Dawt.tookit.name=WLToolkit"
		);
		requires_vulkan = true;
	};
	// https://docs.oracle.com/en/java/javase/25/troubleshoot/java-2d-properties.html
	if features.contains(&Feature::OpenGL) {
		if requires_vulkan {
			panic!("Vulkan required for WLToolkit, but OpenGL was also explicitly requested.  Resolve incompatible args and try again.");
		};
		fuji_jvm_arg!(
			"OpenGL for AWT/Swing.  This has been bundled in OpenJDK for a long time, but isn't on by default",
			"-Dsun.java2d.opengl=true"
		);
	};
	#[cfg(any(target_os = "macos", feature = "multi_os"))]
	if features.contains(&Feature::Metal) {
		if requires_vulkan {
			panic!("Vulkan required for WLToolkit, but Metal was also explicitly requested.  Resolve incompatible args and try again.");
		};
		fuji_jvm_arg!(
			"Metal support for AWT/Swing (macOS).  If you're on macOS, use this instead of OpenGL (Apple has deprecated OpenGL on macOS)",
			"-Dsun.java2d.metal=true"
		);
	};
	if requires_vulkan || features.contains(&Feature::Vulkan) {
		fuji_jvm_arg!(
			"Vulkan for AWT/Swing",
			"-Dsun.java2d.vulkan=true -Dsun.java2d.vulkan.accelsd=false"
		);
	};
	if features.contains(&Feature::AllowNative) {
		fuji_jvm_arg!(
			"Allows all Java modules to use the (soon to be) restricted native library access",
			"--enable-native-access=ALL-UNNAMED"
		);
	};
	if features.contains(&Feature::AllowUnsafe) {
		fuji_jvm_arg!(
			"Allows use of the (soon to be) restricted sun.misc.Unsafe API access",
			"--sun-misc-unsafe-memory-access=allow"
		);
	};
	if features.contains(&Feature::FontAntiAliasing) {
		fuji_jvm_arg!(
			"Enables AWT font antialiasing.  This can improve readability and quality of text",
			"-Dawt.useSystemAAFontSettings=on"
		);
	};
	#[cfg(any(target_os = "linux", feature = "multi_os"))]
	if features.contains(&Feature::NVIDIAFixes) {
		result.push_str("# General fixes for NVIDIA GPUs on Linux\n");
		result.push_str("export __GL_THREADED_OPTIMIZATIONS=0\n\n");
	};

	result.push_str("# shellcheck disable=SC2155\n");
	result.push_str(&format!("export JAVA_HOME=\"{java_home}\"\n\n"));

	result.push_str("if [ -n \"$CLASSPATH\" ]; then\n\t");
	result.push_str(self_arg!("-cp \"$CLASSPATH:.\""));
	result.push_str("fi\n\n");

	result.push_str("exec \"$JAVA_HOME/bin/java.bak\" \"$@\"");

	result
}

pub fn install_wrapper(script: String, output_dir: &str) -> String {
	let script_file: String = format!("{output_dir}/bin/fuji_jvm_wrapper");
	let mut result: File = OpenOptions::new()
		.write(true)
		.create_new(true)
		.open(&script_file)
		.expect(&io_expect(&script_file, "create"));
	result
		.write_all(script.as_bytes())
		.expect(&io_expect(&script_file, "write"));
	// rwxr-xr-x
	result
		.set_permissions(Permissions::from_mode(0o755))
		.expect(&io_expect(&script_file, "set permissions for"));
	script_file
}