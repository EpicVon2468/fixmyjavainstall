use std::io::Result;

use clap::{Subcommand, ValueEnum};

use serde::{Deserialize as Deserialise, Serialize as Serialise};

use crate::arch::Arch;
use crate::jvm::vendor::Vendor;

#[derive(Subcommand)]
pub enum Op {
	/// Installs a new JVM
	Install {
		/// The vendor for the requested JVM
		#[arg(short, long, default_value = "jbr")]
		vendor: Vendor,

		/// The architecture for the requested JVM.  Note that not every vendor may support every architecture, and some vendors may not offer certain features for all architectures.  Generally speaking, x64 (amd64) has the highest level of support overall
		#[arg(short, long, default_value="system")]
		arch: Arch,

		/// The features for the requested JVM.  Note that not every vendor may support every feature, and some vendors may only offer features for certain versions or with incompatibilities with other features
		#[arg(short, long)]
		features: Vec<Feature>,

		/// The requested JVM version.  An integer representing the major version (or 'latest' for the latest available version)
		// #[clap(default_value_t = String::from("latest"))]
		version: String,
	},
	/// Removes the currently installed JVM (only affects JVMs installed via fuji)
	Remove,
}

#[derive(ValueEnum, Clone, PartialEq)]
pub enum Feature {
	/// Minimal JVM (JRE or no-Javadoc JDK).  If you don't know what this means & aren't a developer, you probably want this
	MINIMAL,
	/// Dynamic Code Evolution Virtual Machine (enhanced runtime class redefinition) – https://ssw.jku.at/dcevm/
	///
	/// `-XX:+AllowEnhancedClassRedefinition`
	DCEVM,
	/// JDK Enhancement Proposal 519 (Compact Object Headers) – https://openjdk.org/jeps/519
	///
	/// `-XX:+UseCompactObjectHeaders`
	#[clap(name = "jep-519")]
	JEP519,
	/// Wayland support (requires Vulkan) – https://wiki.openjdk.org/spaces/wakefield/pages/77693134/Pure+Wayland+toolkit+prototype
	///
	/// `-Dawt.tookit.name=WLToolkit -Dsun.java2d.vulkan=true -Dsun.java2d.vulkan.accelsd=false`
	#[clap(name = "wltoolkit")]
	WLToolkit,
	/// OpenGL for AWT/Swing.  This has been bundled in OpenJDK for a long time, but isn't on by default
	///
	/// `-Dsun.java2d.opengl=true`
	#[clap(name = "opengl")]
	OpenGL,
	/// Vulkan for AWT/Swing.
	///
	/// `-Dsun.java2d.vulkan=true -Dsun.java2d.vulkan.accelsd=false`
	Vulkan,
	/// Java Chromium Embedded Framework – https://github.com/chromiumembedded/java-cef/
	JCEF,
	/// MUSL libc support – https://musl.libc.org/
	MUSL,
}

#[derive(Serialise, Deserialise)]
pub struct JavaVersion<'a> {
	pub major: &'a str,
	pub revision: &'a str,
}

pub fn download_jbr(
	arch: &Arch,
	version: &JavaVersion,
	features: &Vec<Feature>
) -> Result<()> {
	let mut url: String = String::with_capacity(100);
	url.push_str("https://cache-redirector.jetbrains.com/intellij-jbr/jbr");
	if !features.contains(&Feature::MINIMAL) {
		url.push_str("sdk");
	};
	if features.contains(&Feature::JCEF) {
		url.push_str("_jcef");
	};
	url.push('-');
	url.push_str(version.major);
	url.push_str("-linux-");
	if features.contains(&Feature::MUSL) {
		url.push_str("musl-");
	};
	url.push_str(arch.to_string().as_str());
	url.push('-');
	url.push_str(version.revision);
	url.push_str(".tar.gz");
	println!("Got URL: {url}");
	Ok(())
}