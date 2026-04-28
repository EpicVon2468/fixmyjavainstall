use anyhow::{Context as _, Result};

#[cfg(windows)]
pub fn set_java_home<S: AsRef<str>>(value: S) -> Result<()> {
	crate::win_link::set_java_home(value.as_ref())
		.context("Couldn't set JAVA_HOME environment variable!")
}

#[cfg(target_os = "linux")]
pub fn set_java_home<S: AsRef<str>>(_ignored: S) -> Result<()> {
	crate::unlock!(
		crate::env_util::append_or_create_env_file().context("Couldn't set JAVA_HOME!")?
	);
	Ok(())
}

#[cfg(target_os = "macos")]
pub fn set_java_home<S: AsRef<str>>(value: S) -> Result<()> {
	Ok(())
}
