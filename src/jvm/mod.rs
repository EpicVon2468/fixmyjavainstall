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
use crate::cli::Software;
use crate::jvm::jvm::JVM;
use crate::jvm::major_version::{FujiValueEnumParser, MajorVersion};
#[cfg(feature = "multi-os")]
use crate::os::OS;
use crate::wrong_cmd;

#[derive(Subcommand)]
#[command(author)]
pub enum Op {
	// TODO: L&F?
	/// Installs a new JVM.
	#[command(author)]
	Install {
		/// The build/vendor for the requested JVM.
		#[arg(short, long, alias = "java-virtual-machine", default_value = "jbr")]
		jvm: JVM,

		/// The architecture for the requested JVM.
		///
		/// Note that not every JVM may support every architecture, and some JVMs may not offer certain features for all architectures.  Generally speaking, x64 (amd64) has the highest level of support overall.
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

		// TODO: concat
		/// The features for the requested JVM.
		///
		/// Note that not every JVM may support every feature, and some JVMs may only offer features for certain versions or with incompatibilities with other features.
		#[arg(short, long)]
		features: Vec<Feature>,

		// TODO: Move into Feature?
		/// Whether to bundle Kotlin with the requested JVM.
		#[arg(short = 'k', long)]
		include_kotlin: bool,

		/// Show execution path without actually installing the JVM.
		#[arg(long)]
		dry_run: bool,

		/// The version for the requested JVM
		#[arg(value_parser = FujiValueEnumParser::default())]
		version: MajorVersion,
	},
	/// Removes the currently installed JVM (only affects JVMs installed via fuji).
	#[command(author, alias = "uninstall")]
	Remove,
	/// Installs a new JVM from a selection of presets.
	#[command(author, alias = "presets")]
	Preset {
		#[command(subcommand)]
		preset: Preset,
	},
}

#[derive(Subcommand)]
#[command(subcommand_value_name = "PRESET")]
pub enum Preset {
	/// All the recommended defaults + optimisations for your system – Java Runtime Environment edition.
	RecommendedJRE,
	/// All the recommended defaults + optimisations for your system – Java Development Kit edition.
	RecommendedJDK,
	/// (Almost) all the optimisations – Java Runtime Environment edition; For the performance-wary user.
	FastJRE,
	/// (Almost) all the optimisations – Java Development Kit edition; For the performance-wary developer.
	FastJDK,
	/// Bleeding-edge & unstable, you say?
	LatestJRE,
	/// Bleeding-edge & unstable, you say?
	LatestJDK,
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

// FIXME: https://github.com/rust-lang/cargo/issues/4648 + https://github.com/clap-rs/clap/issues/6096
#[non_exhaustive]
#[derive(ValueEnum, Clone, PartialEq, Eq)]
pub enum Feature {
	/// Minimal JVM (JRE or no-Javadoc JDK).
	///
	/// If you don't know what this means & aren't a developer, you probably want this.
	Minimal,
	/// Dynamic Code Evolution Virtual Machine (enhanced runtime class redefinition) – <https://ssw.jku.at/dcevm/>.
	///
	/// Highly recommended for development, as it can allow for non-insignificant code changes without needing to restart the JVM.
	DCEVM,
	/// JDK Enhancement Proposal 519 (Compact Object Headers) – <https://openjdk.org/jeps/519>.
	///
	/// This feature can generally be considered stable, and is recommended for its strong performance benefits.
	///
	/// Additionally, some JVM vendors have backported this feature to previous versions, and [it may be enabled by default in future](https://openjdk.org/jeps/534).
	#[value(name = "jep-519", alias = "compact-object-headers")]
	JEP519,
	/// Wayland support (requires Vulkan) – <https://wiki.openjdk.org/spaces/wakefield/pages/77693134/Pure+Wayland+toolkit+prototype>.
	///
	/// See also:
	///
	/// - <https://github.com/openjdk/wakefield/>.
	/// - <https://openjdk.org/projects/wakefield/>.
	#[cfg(any(target_os = "linux", feature = "multi-os"))]
	#[value(aliases = vec!["wakefield", "wltoolkit", "wl"])]
	Wayland,
	/// OpenGL for AWT/Swing.
	///
	/// This has been bundled in OpenJDK for a long time, but isn't on by default.
	///
	/// macOS users, use Metal instead (Apple has deprecated OpenGL on macOS).
	#[value(name = "opengl", alias = "gl")]
	OpenGL,
	/// Metal support for AWT/Swing (macOS) – <https://developer.apple.com/metal/>.
	///
	/// If you're on macOS, use this instead of OpenGL (Apple has deprecated OpenGL on macOS).
	#[cfg(any(target_os = "macos", feature = "multi-os"))]
	Metal,
	/// Vulkan for AWT/Swing.
	///
	/// DEV NOTE: I'm not sure if this does anything when not used in conjunction with Wayland.
	#[value(alias = "vk")]
	Vulkan,
	/// Java Chromium Embedded Framework – <https://github.com/chromiumembedded/java-cef/>.
	///
	/// Webdev???  In my JVM???
	JCEF,
	/// Allows all Java modules to use the (soon to be) restricted native library access – <https://openjdk.org/jeps/472>.
	///
	/// See also:
	///
	/// - <https://inside.java/2024/12/09/quality-heads-up/>.
	/// - <https://docs.oracle.com/en/java/javase/25/core/restricted-methods.html>.
	#[value(alias = "allow-native")]
	Native,
	/// Allows use of the (soon to be) restricted sun.misc.Unsafe API access – <https://openjdk.org/jeps/471>.
	#[value(alias = "allow-unsafe")]
	Unsafe,
	/// Enables AWT font antialiasing.  This can improve readability and quality of text.
	FontFix,
	/// General fixes for NVIDIA GPUs on Linux.
	///
	/// Rendering may not work correctly or even at all without these.
	#[cfg(any(target_os = "linux", feature = "multi-os"))]
	NVIDIA,
	/// MUSL libc support – <https://musl.libc.org/>.
	///
	/// It is unlikely that a glibc JVM will work on MUSL.  Additionally, MUSL support is few and far between amongst JVM vendors.
	#[cfg(any(target_env = "musl", feature = "multi-os"))]
	MUSL,
}

#[derive(Serialise, Deserialise)]
pub struct JavaVersion {
	pub major: String,
	pub specific: String,
	pub revision: String,
}
