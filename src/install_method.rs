use std::env::var;

use anyhow::Result;

use clap::ValueEnum;

use crate::value_enum_extensions;

// TODO: use environment variables to set args for commands (i.e FUJI_UPDATE_ALTERNATIVES_ARGS) -- https://crates.io/crates/shell-words/
#[non_exhaustive]
#[derive_const(ValueEnum, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum InstallMethod {
	/// PATH environment variable modification.
	Path,
	/// Symbolic linking.
	#[value(hide = cfg!(all(not(unix), not(feature = "multi-os"))), alias = "symbolic-link")]
	Symlink,
	/// <https://man7.org/linux/man-pages/man1/update-alternatives.1.html>.
	#[value(hide = cfg!(all(not(target_os = "linux"), not(feature = "multi-os"))))]
	UpdateAlternatives,
}

impl InstallMethod {
	/// Returns the name of a program required by this [`InstallMethod`], if applicable.
	#[must_use]
	pub const fn program_name(&self) -> Option<&str> {
		match *self {
			Self::UpdateAlternatives => Some("update-alternatives"),
			_ => None,
		}
	}
}

value_enum_extensions!(
	InstallMethod,
	cfg_select! {
		target_os = "linux" => Self::Symlink,
		target_os = "macos" => Self::Path,
		windows => Self::Path,
		_ => panic!("Unsupported host!"),
	},
	match *self {
		Self::Path => "path",
		Self::Symlink => "symlink",
		Self::UpdateAlternatives => "update-alternatives",
	},
);

// noinspection RsReplaceMatchExpr
#[allow(
	clippy::unnecessary_wraps,
	clippy::option_if_let_else,
	clippy::manual_unwrap_or_default,
	unused
)]
fn get_args(method: &InstallMethod) -> Result<Vec<String>> {
	let raw_args: Option<String> = match var(format!("FUJI__{method}__ARGS")).map(Some) {
		Ok(value) => value,
		Err(_) => cfg_select! {
			feature = "interactive" => dialoguer::Editor::new().edit("Enter args for <command name here>:")?,
			_ => None,
		},
	};
	let args: Option<Result<Vec<String>, _>> = raw_args.as_deref().map(shell_words::split);
	Ok(todo!())
}
