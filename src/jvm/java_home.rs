use anyhow::{Context as _, Result};

pub fn set_java_home<S: AsRef<str>>(value: S) -> Result<()> {
	set_java_home_(value.as_ref())
}

#[cfg(windows)]
fn set_java_home_(value: &str) -> Result<()> {
	crate::win_link::set_java_home(value.as_ref())
		.context("Couldn't set JAVA_HOME environment variable!")
}

#[cfg(target_os = "linux")]
fn set_java_home_(_ignored: &str) -> Result<()> {
	crate::unlock!(
		crate::env_util::append_or_create_env_file().context("Couldn't set JAVA_HOME!")?
	);
	Ok(())
}

#[cfg(target_os = "macos")]
fn set_java_home_(_ignored: &str) -> Result<()> {
	Ok(())
}
