use std::io::Result;

use clap::{Subcommand, ValueEnum};

use serde::{Deserialize as Deserialise, Serialize as Serialise};

use crate::arch::Arch;
use crate::cli::Software;
use crate::jvm::install::install;
use crate::jvm::jdk::JDK;
use crate::jvm::major_version::{MajorVersion, MajorVersionParser};
#[cfg(feature = "multi_os")]
use crate::os::OS;
use crate::wrong_cmd;

#[derive(Subcommand)]
#[clap(author = "Mavity The Madity")]
pub enum Op {
	// TODO: L&F?
	/// Installs a new JVM
	#[clap(author = "Mavity The Madity")]
	Install {
		/// The JDK for the requested JVM
		#[arg(short, long = "java-dev-kit", default_value = "jbr")]
		jdk: JDK,

		/// The architecture for the requested JVM.  Note that not every JDK may support every architecture, and some JDKs may not offer certain features for all architectures.  Generally speaking, x64 (amd64) has the highest level of support overall
		#[arg(
			short,
			long,
			default_value = crate::arch::SYSTEM,
		)]
		arch: Arch,

		/// The OS for the requested JVM
		#[cfg(feature = "multi_os")]
		#[arg(
			short,
			long,
			alias = "os",
			default_value = crate::os::SYSTEM,
		)]
		operating_system: OS,

		/// The features for the requested JVM.  Note that not every JDK may support every feature, and some JDKs may only offer features for certain versions or with incompatibilities with other features
		#[arg(short, long)]
		features: Vec<Feature>,

		/// Whether to bundle Kotlin with the requested JVM
		#[arg(short = 'k', long)]
		include_kotlin: bool,

		/// Show execution path without actually installing the JVM
		#[arg(long)]
		dry_run: bool,

		/// The version for the requested JVM
		#[clap(value_parser = MajorVersionParser::new())]
		version: MajorVersion,
	},
	/// Removes the currently installed JVM (only affects JVMs installed via fuji)
	#[clap(author = "Mavity The Madity")]
	Remove,
}

pub fn manage_jvm(software: Software) -> Result<()> {
	let Software::JVM {
		op,
	} = software else {
		wrong_cmd!(manage_jvm);
	};
	match op {
		Op::Install { .. } => return install(op),
		Op::Remove => {},
	};
	Ok(())
}

#[derive(ValueEnum, Clone, PartialEq)]
pub enum Feature {
	/// Minimal JVM (JRE or no-Javadoc JDK).  If you don't know what this means & aren't a developer, you probably want this
	Minimal,
	/// Dynamic Code Evolution Virtual Machine (enhanced runtime class redefinition) – <https://ssw.jku.at/dcevm/>
	///
	/// `-XX:+AllowEnhancedClassRedefinition`
	DCEVM,
	/// JDK Enhancement Proposal 519 (Compact Object Headers) – <https://openjdk.org/jeps/519>
	///
	/// `-XX:+UseCompactObjectHeaders`
	#[clap(name = "jep-519", alias = "compact-object-headers")]
	JEP519,
	/// Wayland support (requires Vulkan) – <https://wiki.openjdk.org/spaces/wakefield/pages/77693134/Pure+Wayland+toolkit+prototype>
	///
	/// `-Dawt.tookit.name=WLToolkit -Dsun.java2d.vulkan=true -Dsun.java2d.vulkan.accelsd=false`
	#[cfg(any(target_os = "linux", feature = "multi_os"))]
	#[clap(name = "wltoolkit", aliases = vec!["wakefield", "wayland", "wl"])]
	WLToolkit,
	/// OpenGL for AWT/Swing.  This has been bundled in OpenJDK for a long time, but isn't on by default
	///
	/// `-Dsun.java2d.opengl=true`
	#[clap(name = "opengl", alias = "gl")]
	OpenGL,
	/// Metal support for AWT/Swing (macOS).  If you're on macOS, use this instead of OpenGL (Apple has deprecated OpenGL on macOS)
	///
	/// `-Dsun.java2d.metal=true`
	#[cfg(any(target_os = "macos", feature = "multi_os"))]
	Metal,
	/// Vulkan for AWT/Swing
	///
	/// `-Dsun.java2d.vulkan=true -Dsun.java2d.vulkan.accelsd=false`
	#[clap(alias = "vk")]
	Vulkan,
	/// Java Chromium Embedded Framework – <https://github.com/chromiumembedded/java-cef/>
	JCEF,
	/// Allows all Java modules to use the (soon to be) restricted native library access
	///
	/// `--enable-native-access=ALL-UNNAMED`
	AllowNative,
	/// Allows use of the (soon to be) restricted sun.misc.Unsafe API access
	///
	/// `--sun-misc-unsafe-memory-access=allow`
	AllowUnsafe,
	/// Enables AWT font antialiasing.  This can improve readability and quality of text
	///
	/// `-Dawt.useSystemAAFontSettings=on`
	FontAntiAliasing,
	/// General fixes for NVIDIA GPUs on Linux
	///
	/// `__GL_THREADED_OPTIMIZATIONS=0`
	#[cfg(any(target_os = "linux", feature = "multi_os"))]
	NVIDIAFixes,
	/// MUSL libc support – <https://musl.libc.org/>
	#[cfg(any(unix, feature = "multi_os"))]
	MUSL,
}

#[derive(Serialise, Deserialise)]
pub struct JavaVersion<'a> {
	pub major: &'a str,
	pub specific: &'a str,
	pub revision: &'a str,
}