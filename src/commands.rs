use std::cmp::min;
use std::env::var;
use std::fmt::Write;
use std::fs::{File, create_dir_all};
use std::io::copy;
use std::path::{Component, Components, Path, PathBuf};
use std::time::Duration;

use anyhow::{Context as _, Result};

use flate2::read::GzDecoder;

use indicatif::{MultiProgress, ProgressBar, ProgressState, ProgressStyle};

use tar::{Archive, Entries, Entry};

use ureq::http::Response;
use ureq::{Body, get};

use which::which;

use zip::ZipArchive;
use zip::read::ZipFile;

use crate::{lock, unlock};

/// Checks if the program `name` exists.  This is equivalent to `which(name).is_ok()`.
#[inline]
#[must_use]
pub fn has_program(name: &str) -> bool {
	which(name).is_ok()
}

/// Extracts `archive` into `dest`, stripping one or more components.
///
/// # Arguments
///
/// * `archive`: The path to a `.zip` or `.tar.gz` file containing the JVM.
/// * `dest`: The destination folder to extract into.
/// * `is_zip`: Whether `archive` is a `.zip` file.
/// * `is_mac`: Whether `archive` is a macOS JVM.
///
/// # Errors
///
/// Error type: Dynamic (see [`anyhow::Error`]).
///
/// Error value(s):
///
/// * Propagated up from the following functions (if they return [`Err`]):
/// 	* [`Path::canonicalise`][`Path::canonicalize`]
/// 	* [`File::open`]
/// * If `is_zip` is true:
/// 	* Propagated up from [`extract_jvm_zip`].
/// * If `is_zip` is false:
/// 	* Propagated up from [`extract_jvm_tar_gz`].
///
/// # Implementation Notes
///
/// * `dest` is [`canonicalised`][`Path::canonicalize`] before use.
/// * No checks are performed to determine if `dest` exists.
/// * If `is_zip` is true, no checks are performed to determine if `archive` ends with `.zip`, and vice versa.
///
/// # Platform-Specific Behaviour
///
/// * UNIX-likes: [`extract_jvm_tar_gz`] is used.
/// * Windows: [`extract_jvm_zip`] is used.
///
/// # Returns
///
/// Return type: [`Result<()>`]
///
/// Return value(s):
///
/// * Propagated up from the following functions (if they return [`Err`]):
/// 	* [`Path::canonicalise`][`Path::canonicalize`]
/// 	* [`File::open`]
/// * If `is_zip` is true:
/// 	* Propagated up from [`extract_jvm_zip`].
/// * If `is_zip` is false:
/// 	* Propagated up from [`extract_jvm_tar_gz`].
///
/// # Examples
///
/// Extracting a Linux JVM:
/// ```
/// use fuji::commands::extract_jvm;
///
/// assert_eq!(extract_jvm("java-25-linux.tar.gz", "./java-25-linux", false, false), Ok(()));
/// ```
///
/// Extracting a macOS JVM:
/// ```
/// use fuji::commands::extract_jvm;
///
/// assert_eq!(extract_jvm("java-25-osx.tar.gz", "./java-25-osx", false, true), Ok(()));
/// ```
///
/// Extracting a Windows JVM:
/// ```
/// use fuji::commands::extract_jvm;
///
/// assert_eq!(extract_jvm("java-25-win.zip", "./java-25-win", true, false), Ok(()));
/// ```
pub fn extract_jvm<S: AsRef<Path>, P: AsRef<Path>>(
	archive: S,
	dest: P,
	is_zip: bool,
	is_mac: bool,
) -> Result<()> {
	let dest: &Path = &dest
		.as_ref()
		.canonicalize()
		.context("Couldn't canonicalise destination path!")?;
	let input: File = File::open(archive.as_ref()).context("Couldn't open JVM archive!")?;
	lock!(input);
	let result: Result<()> = if is_zip {
		extract_jvm_zip(dest, &input, is_mac)
	} else {
		extract_jvm_tar_gz(dest, &input, is_mac)
	};
	println!("Done.\n");
	unlock!(input);
	result
}

pub fn extract_jvm_tar_gz(dest: &Path, input: &File, is_mac: bool) -> Result<()> {
	let multi: MultiProgress = MultiProgress::new();
	let max_len: u64 = input.metadata()?.len();
	let pb: ProgressBar = multi.add(progress_bar(max_len));
	let mut progress: u64 = 0;
	let mut archive: Archive<GzDecoder<&File>> = Archive::new(GzDecoder::new(input));
	#[expect(
		clippy::literal_string_with_formatting_args,
		reason = "False positive."
	)]
	let extract_pb: ProgressBar = multi.add(progress_bar_template(
		0,
		"[{elapsed_precise}] {spinner:.cyan} Writing {msg}...",
	));
	extract_pb.enable_steady_tick(Duration::from_millis(125));
	let entries: Entries<GzDecoder<&File>> = archive
		.entries()
		.context("Couldn't iterate through JVM archive!")?;
	for entry in entries {
		#[rustfmt::skip]
		let mut entry: Entry<GzDecoder<&File>> = entry.context("Couldn't get entry in JVM archive!")?;
		extract_jvm_entry(
			dest,
			entry
				.path()
				.context("Couldn't get path for entry in JVM archive!")?
				.to_path_buf()
				.as_path(),
			is_mac,
			|resolved: &Path| {
				#[rustfmt::skip]
				extract_pb.clone().with_message(resolved.display().to_string());
				entry.unpack(resolved)?;
				#[cfg(unix)]
				{
					use tar::Header;

					let header: &Header = entry.header();
					update_perms(resolved, header.mode().ok(), header.entry_type().is_dir())?;
				};
				Ok(())
			},
		)?;
		progress = min(progress + entry.size(), max_len);
		pb.set_position(progress);
	}
	extract_pb.finish_and_clear();
	pb.finish();
	Ok(())
}

pub fn extract_jvm_zip(dest: &Path, input: &File, is_mac: bool) -> Result<()> {
	let mut archive: ZipArchive<&File> =
		ZipArchive::new(input).context("Couldn't open JVM archive!")?;
	let multi: MultiProgress = MultiProgress::new();
	#[expect(
		clippy::cast_possible_truncation,
		clippy::as_conversions,
		reason = "A JVM `.zip` bigger than u64::MAX (16384 pebibytes) would be a zip bomb.  Bad clippy!"
	)]
	let max_len: u64 = archive.decompressed_size().unwrap() as u64;
	let pb: ProgressBar = multi.add(progress_bar(max_len));
	let mut progress: u64 = 0;
	let extract_pb: ProgressBar = multi.add(progress_bar_template(
		0,
		&format!("Writing {{msg}}… {TEMPLATE}"),
	));
	for index in 0..archive.len() {
		let mut entry: ZipFile<&File> = archive
			.by_index(index)
			.context("Couldn't get entry in JVM archive (ZIP)!")?;
		if entry.is_symlink() {
			println!("Absolutely not go fuck yourself");
			panic!("https://www.youtube.com/watch?v=yhDMpYkML2k");
		};
		let size: u64 = entry.size();
		extract_jvm_entry(
			dest,
			entry
				.enclosed_name()
				.context("Couldn't get path for entry in JVM archive (ZIP)!")?
				.as_path(),
			is_mac,
			|resolved: &Path| {
				#[cfg(unix)]
				let mode: Option<u32> = entry.unix_mode();

				if entry.is_dir() {
					create_dir_all(resolved).context("create_dir_all (zip)")?;

					#[cfg(unix)]
					update_perms(resolved, mode, true)?;
				} else {
					extract_pb.set_length(size);
					extract_pb.reset();
					// cloned progress bars still use the same internal state, so this call is only to appease the compiler, it serves no other purpose
					#[rustfmt::skip]
					extract_pb.clone().with_message(resolved.display().to_string());

					let out: File = File::create_new(resolved).context("File::create (zip)")?;
					lock!(out);
					{
						copy(&mut entry, &mut extract_pb.wrap_write(&out)).context("copy (zip)")?;

						#[cfg(unix)]
						update_perms(resolved, mode, false)?;
					};
					unlock!(out);
				};

				Ok(())
			},
		)?;
		progress = min(progress + size, max_len);
		pb.set_position(progress);
	}
	extract_pb.finish_and_clear();
	pb.finish();
	Ok(())
}

#[cfg(unix)]
pub fn update_perms(path: &Path, mode: Option<u32>, is_dir: bool) -> Result<()> {
	use std::fs::{Permissions, set_permissions};
	use std::os::unix::fs::PermissionsExt as _;

	let new_mode: u32 = if mode.is_some_and(is_executable) || is_dir {
		// rwxr-xr-x
		0o755
	} else {
		// rw-r--r--
		0o644
	};
	set_permissions(path, Permissions::from_mode(new_mode))
		.with_context(|| io_failure(path, "set permissions for"))
}

#[inline]
#[must_use]
#[cfg(unix)]
pub const fn is_executable(mode: u32) -> bool {
	(mode & 0o111) != 0
}

#[inline]
#[rustfmt::skip]
pub fn extract_jvm_entry<F>(dest: &Path, path: &Path, is_mac: bool, mut unpack: F) -> Result<()>
where F: FnMut(&Path) -> Result<()> {
	let mut components: Components = path.components();
	// https://stackoverflow.com/questions/845593/how-do-i-untar-a-subdirectory-into-the-current-directory
	// --strip-components 1
	components.next();
	assert!(
		!components.clone().any(|comp: Component| comp == Component::ParentDir),
		"Component::ParentDir found!"
	);
	// macOS .tar.gz is laid out differently.  it's a '.app'...
	if is_mac {
		// skip "Contents"
		components.next();
		// only allow paths under "Home"
		if components.next() != Some(Component::Normal("Home".as_ref())) {
			return Ok(());
		};
	};
	let resolved: PathBuf = dest.join(components.as_path());
	unpack(&resolved).context("Couldn't unpack entry from JVM archive!")
}

/// Downloads a resource from `url` to `dest`.
pub fn download<S: AsRef<str>, P: AsRef<Path>>(url: S, dest: P) -> Result<()> {
	let dest: &Path = dest.as_ref();

	let response: Response<Body> = get(url.as_ref())
		.call()
		.context("Couldn't download resource!")?;

	let len: u64 = response
		.headers()
		.get("Content-Length")
		.context("Couldn't get Content-Length header for response!")?
		.to_str()
		.context("Couldn't get string value of Content-Length header!")?
		.parse()
		.context("Couldn't parse integer from Content-Length header!")?;

	let pb: ProgressBar = progress_bar(len);
	let mut out: File =
		File::create(dest).context("Couldn't open destination file for download!")?;
	lock!(out);
	{
		#[rustfmt::skip]
		copy(
			&mut pb.wrap_read(&mut response.into_body().into_reader()),
			&mut out,
		).context("Couldn't download resource from URL!")?;
		pb.finish();
		println!("Done.\n");
	};
	unlock!(out);

	Ok(())
}

#[rustfmt::skip]
pub const TEMPLATE: &str = "[{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})";
pub const SPINNER_PAT: [&str; 13] = [
	"⠉⠙", "⠈⠹", " ⢹", " ⣸", "⢀⣰", "⣀⣠", "⣄⣀", "⣆⡀", "⣇ ", "⡏ ", "⠏⠁", "⠋⠉", "  ",
];
pub const PROGRESS_PAT: &str = "=>-";

#[must_use]
pub fn progress_bar_template(len: u64, message: &str) -> ProgressBar {
	let pb: ProgressBar = ProgressBar::new(len);
	pb.set_style(
		ProgressStyle::with_template(message)
			.unwrap()
			.with_key("eta", |state: &ProgressState, w: &mut dyn Write| {
				write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap();
			})
			.progress_chars(PROGRESS_PAT)
			.tick_strings(&SPINNER_PAT),
	);
	// pb.set_tab_width(4);
	pb
}

// https://github.com/console-rs/indicatif/blob/main/examples/download.rs
#[inline]
#[must_use]
pub fn progress_bar(len: u64) -> ProgressBar {
	progress_bar_template(len, TEMPLATE)
}

pub fn io_failure<P: AsRef<Path>, S: AsRef<str>>(dest: P, msg: S) -> String {
	format!(
		"Couldn't {} path '{}'!",
		msg.as_ref(),
		dest.as_ref().display()
	)
}

#[must_use]
pub fn is_wayland() -> bool {
	has_program("wayland-info")
		|| var("WAYLAND_DISPLAY").is_ok()
		|| var("XDG_SESSION_TYPE").is_ok_and(|var: String| var == "wayland")
}
