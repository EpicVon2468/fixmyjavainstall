use std::fs::{File, OpenOptions, Permissions};
use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::commands::io_failure;
use crate::jvm::Feature;

#[must_use]
pub fn generate_wrapper(
	java_home: &Path,
	features: &[Feature],
	is_win: bool,
	bin_suffix: &str,
) -> String {
	if is_win {
		generate_wrapper_win(java_home, features, bin_suffix)
	} else {
		generate_wrapper_unix(java_home, features, bin_suffix)
	}
}

// PowerShell is NOT an option. '-XX:+UseCompactObjectHeaders' is forcibly split into '-XX' and '+UseCompactObjectHeaders'.  With and without escapes & quotations & brackets.
// https://stackoverflow.com/questions/25122484/how-do-i-emulate-a-wrapper-script-on-windows
// https://superuser.com/questions/1500272/equivalent-of-export-command-in-windows
// https://stackoverflow.com/questions/12990480/shift-doesn-t-affect
// Oh dear...
// https://serverfault.com/questions/315077/is-there-a-windows-cmd-equivalent-of-unix-shells-exec
fn generate_wrapper_win(java_home: &Path, features: &[Feature], bin_suffix: &str) -> String {
	let mut result: String = String::with_capacity(500);

	result.push_str("@echo off\r\n\r\n");
	result.push_str("setlocal enableextensions\r\n\r\n");

	{
		use std::fmt::Write;

		let _ = write!(result, "set JAVA_HOME=\"{}\"\r\n\r\n", java_home.display());
	}

	result.push_str("if defined CLASSPATH (\r\n");
	result.push_str("\tset FUJI_CLASSPATH_ARG=\"-cp %CLASSPATH%;.\"\r\n");
	result.push_str(")\r\n\r\n");

	result.push_str("start /b /wait \"\" \"%JAVA_HOME%\\bin\\java");
	result.push_str(bin_suffix);
	result.push_str(".bak\" ");

	gen_features(&mut result, features, &|_, args: &str| format!("{args} "));

	result.push_str("\"$FUJI_CLASSPATH_ARG\" %*");

	result
}

fn generate_wrapper_unix(java_home: &Path, features: &[Feature], bin_suffix: &str) -> String {
	let mut result: String = String::with_capacity(500);

	result.push_str("#!/usr/bin/env sh\n\n");

	{
		use std::fmt::Write;

		let _ = writeln!(result, "JAVA_HOME=\"{}\"", java_home.display());
	}
	result.push_str("export JAVA_HOME\n\n");

	result.push_str("if [ -n \"$CLASSPATH\" ]; then\n");
	result.push_str("\tset -- -cp \"$CLASSPATH:.\" \"$@\"\n");
	result.push_str("fi\n\n");

	gen_features(&mut result, features, &|comment: &str, args: &str| {
		format!("# {comment}\nset -- {args} \"$@\"\n\n")
	});

	#[cfg(any(target_os = "linux", feature = "multi-os"))]
	if features.contains(&Feature::NVIDIAFixes) {
		result.push_str("# General fixes for NVIDIA GPUs on Linux\n");
		result.push_str("export __GL_THREADED_OPTIMIZATIONS=0\n\n");
	};

	result.push_str(r#"exec "$JAVA_HOME/bin/java"#);
	result.push_str(bin_suffix);
	result.push_str(r#".bak" "$@""#);
	result.push('\n');

	result
}

fn gen_features(
	result: &mut String,
	features: &[Feature],
	transform: &dyn Fn(&str, &str) -> String,
) {
	let mut fuji_jvm_arg = |comment: &str, args: &str| {
		result.push_str(&transform(comment, args));
	};

	if features.contains(&Feature::DCEVM) {
		fuji_jvm_arg(
			"Dynamic Code Evolution Virtual Machine (enhanced runtime class redefinition) – https://ssw.jku.at/dcevm/",
			"-XX:+AllowEnhancedClassRedefinition",
		);
	};
	if features.contains(&Feature::JEP519) {
		fuji_jvm_arg(
			"JDK Enhancement Proposal 519 (Compact Object Headers) – https://openjdk.org/jeps/519",
			"-XX:+UseCompactObjectHeaders",
		);
	};
	#[allow(unused_mut)]
	let mut requires_vulkan: bool = false;
	#[cfg(any(target_os = "linux", feature = "multi-os"))]
	if features.contains(&Feature::WLToolkit) {
		fuji_jvm_arg(
			"Wayland support (requires Vulkan) – https://wiki.openjdk.org/spaces/wakefield/pages/77693134/Pure+Wayland+toolkit+prototype",
			"-Dawt.tookit.name=WLToolkit",
		);
		requires_vulkan = true;
	};
	// https://docs.oracle.com/en/java/javase/25/troubleshoot/java-2d-properties.html
	if features.contains(&Feature::OpenGL) {
		assert!(
			!requires_vulkan,
			"Vulkan required for WLToolkit, but OpenGL was also explicitly requested.  Resolve incompatible args and try again.",
		);
		fuji_jvm_arg(
			"OpenGL for AWT/Swing.  This has been bundled in OpenJDK for a long time, but isn't on by default",
			"-Dsun.java2d.opengl=true",
		);
	};
	#[cfg(any(target_os = "macos", feature = "multi-os"))]
	if features.contains(&Feature::Metal) {
		assert!(
			!requires_vulkan,
			"Vulkan required for WLToolkit, but Metal was also explicitly requested.  Resolve incompatible args and try again.",
		);
		fuji_jvm_arg(
			"Metal support for AWT/Swing (macOS).  If you're on macOS, use this instead of OpenGL (Apple has deprecated OpenGL on macOS)",
			"-Dsun.java2d.metal=true",
		);
	};
	if requires_vulkan || features.contains(&Feature::Vulkan) {
		fuji_jvm_arg(
			"Vulkan for AWT/Swing",
			"-Dsun.java2d.vulkan=true -Dsun.java2d.vulkan.accelsd=false",
		);
	};
	if features.contains(&Feature::AllowNative) {
		fuji_jvm_arg(
			"Allows all Java modules to use the (soon to be) restricted native library access",
			"--enable-native-access=ALL-UNNAMED",
		);
	};
	if features.contains(&Feature::AllowUnsafe) {
		fuji_jvm_arg(
			"Allows use of the (soon to be) restricted sun.misc.Unsafe API access",
			"--sun-misc-unsafe-memory-access=allow",
		);
	};
	if features.contains(&Feature::FontAntiAliasing) {
		fuji_jvm_arg(
			"Enables AWT font antialiasing.  This can improve readability and quality of text",
			"-Dawt.useSystemAAFontSettings=on",
		);
	};
}

pub fn install_wrapper(
	script: &str,
	java_home: &Path,
	bin_suffix: &str,
	is_win: bool,
) -> Result<PathBuf> {
	let script_file: PathBuf = java_home.join("bin").join(
		// fuji_java_wrapper instead of fuji_jvm_wrapper so anything grepping through `ps aux` will still definitely find it
		format!(
			"fuji_java_wrapper{bin_suffix}{}",
			if is_win { ".bat" } else { "" }
		),
	);
	let mut result: File = OpenOptions::new()
		.write(true)
		.create_new(true)
		.open(&script_file)
		.with_context(|| io_failure(&script_file, "create"))?;
	result
		.write_all(script.as_bytes())
		.with_context(|| io_failure(&script_file, "write"))?;
	// rwxr-xr-x
	#[cfg(unix)] {
		use std::os::unix::fs::PermissionsExt;
		result
			.set_permissions(Permissions::from_mode(0o755))
			.with_context(|| io_failure(&script_file, "set permissions for"))?;
	};
	Ok(script_file)
}