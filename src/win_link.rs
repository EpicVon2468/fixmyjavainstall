#![cfg(windows)]
use std::path::Path;

macro_rules! open_env {
	($holder:ident) => {
		windows_registry::$holder
			.options()
			.read()
			.write()
			.open("Environment")
	};
}

// https://stackoverflow.com/questions/79701236/what-is-the-recommended-way-to-append-a-path-to-windows-path-environment-vari
/// Appends `bin_dir` to the `PATH` environment variable.
///
/// Initially, this will attempt to modify the `PATH` of `HKEY_LOCAL_MACHINE`, however if that fails it will use `HKEY_CURRENT_USER` as a fallback.
pub fn win_link<P: AsRef<Path>>(bin_dir: P) -> std::io::Result<()> {
	let bin_dir: &str = bin_dir.as_ref().to_str().unwrap();
	println!("Updating PATH environment variable with bin_dir {bin_dir}");
	// "To programmatically add or modify system environment variables, add them to the HKEY_LOCAL_MACHINE\System\CurrentControlSet\Control\Session Manager\Environment registry key"
	// https://learn.microsoft.com/en-gb/windows/win32/procthread/environment-variables
	let environment = open_env!(LOCAL_MACHINE).unwrap_or_else(|_| {
		open_env!(CURRENT_USER)
			.expect("Couldn't get Environment for HKEY_LOCAL_MACHINE or HKEY_CURRENT_USER")
	});
	let prev_path: String = environment
		.get_string("PATH")
		.expect("Couldn't get PATH environment variable!");
	if prev_path.contains(bin_dir) {
		println!("PATH environment variable already contains directory, skipping!");
		return Ok(())
	};
	// technically I'm supposed to broadcast a message about this, but uh... no.
	environment
		.set_string("PATH", format!("{bin_dir};{prev_path}"))
		.expect("Couldn't set PATH environment variable!");
	println!("Updated PATH environment variable!  Restart your current shell or open a new one for the change to take effect!");
	Ok(())
}