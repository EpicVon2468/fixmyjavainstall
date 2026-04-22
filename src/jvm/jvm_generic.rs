use std::fs::{remove_dir_all, remove_file};
use std::path::Path;

use anyhow::{Context as _, Result};

use crate::arch::Arch;
use crate::commands::{download, extract_jvm};
use crate::exists;
use crate::jvm::JavaVersion;
use crate::jvm::feature::Feature;
use crate::os::OS;

pub type DownloadJVMFn = fn(args: DownloadJVMArgs) -> Result<()>;

pub struct DownloadJVMArgs<'a> {
	pub arch: Arch,
	pub version: JavaVersion,
	pub features: &'a [Feature],
	pub os: OS,
	pub java_home: &'a Path,
	pub dry_run: bool,
}

impl DownloadJVMArgs<'_> {
	#[must_use]
	pub fn is_win(&self) -> bool {
		self.os == OS::Windows
	}

	#[must_use]
	pub fn is_mac(&self) -> bool {
		self.os == OS::OSX
	}
}

#[expect(
	clippy::needless_pass_by_value,
	reason = "Literally nothing else is (or ever will be) using the args after I'm done with them.  Bad clippy!"
)]
pub fn jvm_download_impl<S: AsRef<str>>(url: S, args: DownloadJVMArgs) -> Result<()> {
	let url: &str = url.as_ref();
	let java_home: &Path = args.java_home;
	let is_win: bool = args.is_win();
	let archive: &Path = &java_home.with_added_extension(if is_win { "zip" } else { "tar.gz" });

	println!("Downloading JVM: {url}...");
	if args.dry_run {
		return Ok(());
	};
	// Might exist from a failed previous install
	if exists!(archive) {
		#[rustfmt::skip]
		if archive.is_dir() {
			remove_dir_all(archive)
		} else {
			remove_file(archive)
		}.context("Couldn't remove unexpected pre-existing JVM archive!")?;
	};
	download(url, archive).context("Couldn't download JVM archive!")?;

	println!("Extracting JVM...");
	extract_jvm(archive, java_home, is_win, args.is_mac()).context("Couldn't extract JVM!")?;

	println!("Removing JVM archive...");
	remove_file(archive).context("Couldn't delete JVM archive!")?;
	println!("Done.\n");

	Ok(())
}
