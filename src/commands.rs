use std::cmp::min;
use std::fmt::Write;
use std::fs::{create_dir_all, File, Permissions};
use std::io::copy;
use std::path::{Component, Components, Path, PathBuf};

use anyhow::{Context, Result};

use flate2::read::GzDecoder;

use indicatif::{MultiProgress, ProgressBar, ProgressState, ProgressStyle};

use tar::{Archive, Entry};

use ureq::http::Response;
use ureq::{get, Body};

use which::which;

use zip::read::ZipFile;
use zip::ZipArchive;

/// Checks if the program `name` exists.  This is equivalent to `which(name).is_ok()`.
pub fn has_program(name: &str) -> bool {
	which(name).is_ok()
}

/// Extracts `archive` into `dest`, stripping one component.
///
/// Implementation notes:
///
/// * `dest` is [`canonicalised`][`Path::canonicalize`] before use.
/// * No checks are performed to determine if `dest` exists – however, [`canonicalise`][`Path::canonicalize`] will panic if it does not.
/// * If `is_zip` is true, no checks are performed to determine if `archive` ends with `.zip`, and vice versa.
pub fn extract_jdk<S: AsRef<Path>, P: AsRef<Path>>(
	archive: S,
	dest: P,
	is_zip: bool,
	is_mac: bool,
) -> Result<()> {
	let dest: PathBuf = dest.as_ref().canonicalize().context("Couldn't canonicalise destination path!")?;
	let input: File = File::open(archive.as_ref()).context("Couldn't open JDK archive!")?;
	let result: Result<()> = if is_zip {
		_extract_jdk_zip(dest, input, is_mac)
	} else {
		_extract_jdk_tar_gz(dest, input, is_mac)
	};
	println!("Done.\n");
	result
}

fn _extract_jdk_tar_gz(dest: PathBuf, input: File, is_mac: bool) -> Result<()> {
	let max_len: u64 = input.metadata()?.len();
	let pb: ProgressBar = progress_bar(max_len);
	let mut progress: u64 = 0;
	let mut archive: Archive<GzDecoder<File>> = Archive::new(GzDecoder::new(input));
	archive.set_preserve_permissions(true);
	for entry in archive.entries().context("Couldn't iterate through JDK archive!")? {
		let mut entry: Entry<GzDecoder<File>> = entry.context("Couldn't get entry in JDK archive!")?;
		_extract_jdk(
			&dest,
			entry.path().context("Couldn't get path for entry in JDK archive!")?.to_path_buf(),
			is_mac,
			&mut |resolved: &Path| {
				// TODO: https://docs.rs/indicatif/latest/indicatif/struct.ProgressBar.html#method.enable_steady_tick ?
				// entry.header().mode()
				entry.unpack(resolved)?;
				Ok(())
			},
		)?;
		progress = min(progress + entry.size(), max_len);
		pb.set_position(progress);
	};
	pb.finish();
	Ok(())
}

fn _extract_jdk_zip(dest: PathBuf, input: File, is_mac: bool) -> Result<()> {
	let mut archive: ZipArchive<File> = ZipArchive::new(input).context("Couldn't open JDK archive (ZIP)!")?;
	let m: MultiProgress = MultiProgress::new();
	let max_len: u64 = archive.decompressed_size().unwrap() as u64;
	let pb: ProgressBar = m.add(progress_bar(max_len));
	let mut progress: u64 = 0;
	let extract_pb: ProgressBar = m.add(
		progress_bar_template(0, &format!("Writing {{msg}}… {TEMPLATE}"))
	);
	for index in 0..archive.len() {
		let mut entry: ZipFile<File> = archive.by_index(index).context("Couldn't get entry in JDK archive (ZIP)!")?;
		if entry.is_symlink() {
			println!("Absolutely not go fuck yourself");
			panic!("https://www.youtube.com/watch?v=yhDMpYkML2k");
		};
		let size: u64 = entry.size();
		_extract_jdk(
			&dest,
			entry.enclosed_name().context("Couldn't get path for entry in JDK archive (ZIP)!")?,
			is_mac,
			&mut |resolved: &Path| {
				if entry.is_dir() {
					create_dir_all(resolved).context("create_dir_all (zip)")?;
				} else {
					extract_pb.set_length(size);
					extract_pb.reset();
					// cloned progress bars still use the same internal state, so this call is only to appease the compiler, it serves no other purpose
					extract_pb.clone().with_message(resolved.display().to_string());
					let out: File = File::create(resolved).context("File::create (zip)")?;
					copy(&mut entry, &mut extract_pb.wrap_write(out)).context("copy (zip)")?;
				};
				#[cfg(unix)] {
					use std::fs::set_permissions;
					use std::os::unix::fs::PermissionsExt;
					// check if executable (need to figure out how to rewrite this so directory isn't a separate branch)
					let mode: u32 = if let Some(mode) = entry.unix_mode() && (mode & 0o111) != 0 {
						// rwxr-xr-x
						0o755
					} else if entry.is_dir() {
						// rwxr-xr-x
						0o755
					} else {
						// rw-r--r--
						0o644
					};
					set_permissions(resolved, Permissions::from_mode(mode)).context("set_permissions")?;
				};
				Ok(())
			},
		)?;
		progress = min(progress + size, max_len);
		pb.set_position(progress);
	};
	extract_pb.finish_and_clear();
	pb.finish();
	Ok(())
}

fn _extract_jdk<F>(dest: &Path, path: PathBuf, is_mac: bool, unpack: &mut F) -> Result<()>
where F: FnMut(&Path) -> Result<()> {
	let mut components: Components = path.components();
	// https://stackoverflow.com/questions/845593/how-do-i-untar-a-subdirectory-into-the-current-directory
	// --strip-components 1
	components.next();
	if components.clone().any(|c: Component| c == Component::ParentDir) {
		panic!("Component::ParentDir found!");
	};
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
	unpack(&resolved).context("Couldn't unpack entry from JDK archive!")
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
	let mut dest: File = File::create(dest).context("Couldn't open destination file for download!")?;
	copy(
		&mut pb.wrap_read(&mut response.into_body().into_reader()),
		&mut dest,
	).context("Couldn't download resource from URL!")?;
	pb.finish();
	println!("Done.\n");

	Ok(())
}

const TEMPLATE: &str = "[{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})";

pub fn progress_bar_template(len: u64, message: &str) -> ProgressBar {
	let pb: ProgressBar = ProgressBar::new(len);
	pb.set_style(
		ProgressStyle::with_template(message)
			.unwrap()
			.with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
			.progress_chars("=>-")
	);
	pb.set_tab_width(4);
	pb
}

// https://github.com/console-rs/indicatif/blob/main/examples/download.rs
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