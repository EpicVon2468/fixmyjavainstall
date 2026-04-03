#![cfg(windows)]
use std::io::Result;
use std::path::Path;

// https://stackoverflow.com/questions/79701236/what-is-the-recommended-way-to-append-a-path-to-windows-path-environment-vari
// https://stackoverflow.com/questions/8358265/how-can-i-update-the-path-variable-permanently-from-the-windows-command-line
// https://learn.microsoft.com/en-gb/windows/win32/procthread/environment-variables
// Completely untested.  I don't even have the syntax highlighting to see if this works since I'm not on Windows.
pub fn win_link<P: AsRef<Path>>(bin_dir: P) -> Result<()> {
	let key = windows_registry::CURRENT_USER
		.options()
		.read()
		.write()
		.open("Environment")
		.unwrap();
	let prev_path = key.get_string("PATH").unwrap();
	key.set_string("PATH", format!("{prev_path};{}", bin_dir.as_ref().display()));
	Ok(())
}