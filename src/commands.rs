use std::borrow::Cow;
use std::cmp::min;
use std::fmt::Write;
use std::fs::File;
use std::io::copy;
use std::path::{Component, Components, Path, PathBuf};

use anyhow::{Context, Result};

use flate2::read::GzDecoder;

use indicatif::{ProgressBar, ProgressState, ProgressStyle};

use tar::{Archive, Entry};

use ureq::http::Response;
use ureq::{get, Body};

use which::which;

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
	if is_zip {
		panic!("no.");
	};
	let dest: PathBuf = dest.as_ref().canonicalize().context("Couldn't canonicalise destination path!")?;
	let input: File = File::open(archive.as_ref()).context("Couldn't open JDK archive!")?;
	let max_len: u64 = input.metadata()?.len();
	let pb: ProgressBar = progress_bar(max_len);
	let mut progress: u64 = 0;
	let mut reader: Archive<GzDecoder<File>> = Archive::new(GzDecoder::new(input));
	for entry in reader.entries().context("Couldn't iterate through JDK archive!")? {
		let mut entry: Entry<GzDecoder<File>> = entry.context("Couldn't get entry in JDK archive!")?;
		let path: Cow<Path> = entry.path().context("Couldn't get path for entry in JDK archive!")?;
		let mut components: Components = path.components();
		// https://stackoverflow.com/questions/845593/how-do-i-untar-a-subdirectory-into-the-current-directory
		// --strip-components 1
		components.next();
		// macOS .tar.gz is laid out differently.  it's a '.app'...
		if is_mac {
			// skip "Contents"
			components.next();
			// only allow paths under "Home"
			if components.next() != Some(Component::Normal("Home".as_ref())) {
				continue;
			};
		};
		entry
			.unpack(dest.join(components.as_path()))
			.context("Couldn't unpack file from JDK archive!")?;
		progress = min(progress + entry.size(), max_len);
		pb.set_position(progress);
	};
	pb.finish();
	println!("Done.\n");
	Ok(())
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

// https://github.com/console-rs/indicatif/blob/main/examples/download.rs
pub fn progress_bar(len: u64) -> ProgressBar {
	let pb: ProgressBar = ProgressBar::new(len);
	pb.set_style(ProgressStyle::with_template(TEMPLATE)
		.unwrap()
		.with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
		.progress_chars("=>-"));
	pb
}

pub fn io_failure<P: AsRef<Path>, S: AsRef<str>>(dest: P, msg: S) -> String {
	format!(
		"Couldn't {} path '{}'!",
		msg.as_ref(),
		dest.as_ref().display()
	)
}