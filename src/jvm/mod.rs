//! The modules responsible for (un)installing & managing the Java Virtual Machine.
//!
//! The modules are laid out as related to the following groups:
//!
//! <details><summary>Java Virtual Machines:</summary>
//!
//! * [`jvm`] – The enumeration of supported JVM builds/vendors.
//! * [`jvm_generic`] – I don't even know at this point.
//! * [`jvm_java_se`] – The download handler for [`Java Platform, Standard Edition`][`jvm::JVM::JavaSE`].
//! * [`jvm_jbr`] – The download handler for [`JetBrains Runtime`][`jvm::JVM::JBR`].
//! * [`jvm_liberica`] – The download handler for [`Liberica`][`jvm::JVM::Liberica`].
//! * [`jvm_temurin`] – The download handler for [`Eclipse Temurin`][`jvm::JVM::Temurin`].
//! </details>
pub mod cmd_install;
pub mod cmd_preset;
#[cfg(target_os = "linux")]
pub mod desktop;
#[expect(clippy::module_inception)]
pub mod jvm;
pub mod jvm_generic;
pub mod jvm_java_se;
pub mod jvm_jbr;
pub mod jvm_liberica;
pub mod jvm_temurin;
pub mod major_version;
pub mod wrapper;

use anyhow::{Context as _, Result};

use clap::{Subcommand, ValueEnum};

use serde::{Deserialize as Deserialise, Serialize as Serialise};

use crate::arch::Arch;
use crate::cli::{Preset, Software};
use crate::jvm::jvm::JVM;
use crate::jvm::major_version::{MajorVersion, MajorVersionParser};
#[cfg(feature = "multi-os")]
use crate::os::OS;
use crate::wrong_cmd;

#[derive(Subcommand)]
#[clap(author = "Mavity The Madity")]
pub enum Op {
	// TODO: L&F?
	/// Installs a new JVM.
	#[clap(author = "Mavity The Madity")]
	Install {
		/// The build/vendor for the requested JVM.
		#[arg(short, long, alias = "java-virtual-machine", default_value = "jbr")]
		jvm: JVM,

		/// The architecture for the requested JVM.  Note that not every JVM may support every architecture, and some JVMs may not offer certain features for all architectures.  Generally speaking, x64 (amd64) has the highest level of support overall.
		#[arg(
			short,
			long,
			default_value = Arch::default(),
		)]
		arch: Arch,

		/// The OS for the requested JVM.
		#[cfg(feature = "multi-os")]
		#[arg(
			short,
			long,
			alias = "os",
			default_value = OS::default(),
		)]
		operating_system: OS,

		/// The features for the requested JVM.  Note that not every JVM may support every feature, and some JVMs may only offer features for certain versions or with incompatibilities with other features.
		#[arg(short, long)]
		features: Vec<Feature>,

		/// Whether to bundle Kotlin with the requested JVM.
		#[arg(short = 'k', long)]
		include_kotlin: bool,

		/// Show execution path without actually installing the JVM.
		#[arg(long)]
		dry_run: bool,

		/// The version for the requested JVM
		#[clap(value_parser = MajorVersionParser::default())]
		version: MajorVersion,
	},
	/// Removes the currently installed JVM (only affects JVMs installed via fuji).
	#[clap(author = "Mavity The Madity", alias = "uninstall")]
	Remove,
	/// Installs a new JVM from a selection of presets.
	#[clap(author = "Mavity The Madity", alias = "presets")]
	Preset {
		#[command(subcommand)]
		preset: Preset,
	},
}

pub fn manage_jvm(software: Software) -> Result<()> {
	let Software::JVM { op }: Software = software else {
		wrong_cmd!(manage_jvm);
	};
	match op {
		Op::Install { .. } => cmd_install::cmd_install(op).context("Couldn't install JVM!"),
		Op::Remove => todo!(),
		Op::Preset { .. } => cmd_preset::cmd_preset(op).context("Couldn't install JVM preset!"),
	}
}

#[derive(ValueEnum, Clone, PartialEq, Eq)]
pub enum Feature {
	/// Minimal JVM (JRE or no-Javadoc JDK).  If you don't know what this means & aren't a developer, you probably want this.
	Minimal,
	/// Dynamic Code Evolution Virtual Machine (enhanced runtime class redefinition) – <https://ssw.jku.at/dcevm/>.
	DCEVM,
	/// JDK Enhancement Proposal 519 (Compact Object Headers) – <https://openjdk.org/jeps/519>.
	#[clap(name = "jep-519", alias = "compact-object-headers")]
	JEP519,
	/// Wayland support (requires Vulkan) – <https://wiki.openjdk.org/spaces/wakefield/pages/77693134/Pure+Wayland+toolkit+prototype>.
	#[cfg(any(target_os = "linux", feature = "multi-os"))]
	#[clap(name = "wltoolkit", aliases = vec!["wakefield", "wayland", "wl"])]
	WLToolkit,
	/// OpenGL for AWT/Swing.  This has been bundled in OpenJDK for a long time, but isn't on by default.
	#[clap(name = "opengl", alias = "gl")]
	OpenGL,
	/// Metal support for AWT/Swing (macOS).  If you're on macOS, use this instead of OpenGL (Apple has deprecated OpenGL on macOS).
	#[cfg(any(target_os = "macos", feature = "multi-os"))]
	Metal,
	/// Vulkan for AWT/Swing.
	#[clap(alias = "vk")]
	Vulkan,
	/// Java Chromium Embedded Framework – <https://github.com/chromiumembedded/java-cef/>.
	JCEF,
	/// Allows all Java modules to use the (soon to be) restricted native library access.
	AllowNative,
	/// Allows use of the (soon to be) restricted sun.misc.Unsafe API access.
	AllowUnsafe,
	/// Enables AWT font antialiasing.  This can improve readability and quality of text.
	FontAntiAliasing,
	/// General fixes for NVIDIA GPUs on Linux.
	#[cfg(any(target_os = "linux", feature = "multi-os"))]
	NVIDIAFixes,
	/// MUSL libc support – <https://musl.libc.org/>.
	#[cfg(any(target_env = "musl", feature = "multi-os"))]
	MUSL,
}

#[derive(Serialise, Deserialise)]
pub struct JavaVersion {
	pub major: String,
	pub specific: String,
	pub revision: String,
}
