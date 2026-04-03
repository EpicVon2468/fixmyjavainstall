#![cfg(windows)]
use std::io::Result;
use std::path::Path;

// https://stackoverflow.com/questions/79701236/what-is-the-recommended-way-to-append-a-path-to-windows-path-environment-vari
// Completely untested.  I don't even have the syntax highlighting to see if this works since I'm not on Windows.
pub fn win_link<P: AsRef<Path>>(bin_dir: P) -> Result<()> {
	let bin_dir: &Path = bin_dir.as_ref();
	println!("Updating PATH with bin_dir {}", bin_dir.display());
	// "To programmatically add or modify system environment variables, add them to the HKEY_LOCAL_MACHINE\System\CurrentControlSet\Control\Session Manager\Environment registry key"
	// https://learn.microsoft.com/en-gb/windows/win32/procthread/environment-variables
	let key = windows_registry::LOCAL_MACHINE
		.options()
		.read()
		.write()
		.open("Environment")
		.unwrap_or_else(|_| {
			windows_registry::CURRENT_USER
				.options()
				.read()
				.write()
				.open("Environment")
				.expect("Couldn't get Environment for HKEY_LOCAL_MACHINE or HKEY_CURRENT_USER")
		});
	let prev_path = key.get_string("PATH").expect("Couldn't get PATH");
	// technically I'm supposed to broadcast a message about this, but uh... no.
	key.set_string("PATH", format!("{prev_path};{}", bin_dir.display())).expect("Couldn't set PATH");
	Ok(())
}