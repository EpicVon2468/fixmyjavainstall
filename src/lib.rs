#![warn(clippy::pedantic, clippy::nursery)]
#![allow(
	// Why???  Bad clippy!
	clippy::tabs_in_doc_comments,
	// Consistency & uniformity looks better.  Bad clippy!
	clippy::unnecessary_semicolon,
	// I'll get to writing doc comments when I get to them.
	clippy::missing_errors_doc,
	clippy::missing_panics_doc,
	// 'JetBrains' and 'AdoptOpenJDK' are not identifiers I'm referencing.  Bad clippy!
	clippy::doc_markdown,
)]
//! # Fix Ur Java Install – A JVM & Kotlin management utility.
//!
//! Developer note: Expect regular breaking changes; Do not depend on `libfuji` as a stable API!
pub mod arch;
pub mod cli;
pub mod cmd_link;
pub mod cmd_man;
pub mod cmd_manage;
pub mod commands;
pub mod jvm;
pub mod kotlin;
pub mod macros;
pub mod os;
#[cfg(windows)]
pub mod win_link;

use std::env::args_os;
use std::ffi::OsString;

use anyhow::Result;

use clap::Parser;

use crate::cli::{Arguments, Cmd};
#[cfg(any(not(windows), feature = "multi-os"))]
use crate::cmd_link::cmd_link;
use crate::cmd_man::cmd_man;
use crate::cmd_manage::cmd_manage;

/// The installation directory for fuji-managed programs.
///
/// # Platform-Specific Behaviour
///
/// * UNIX-likes: `/opt/fuji`
/// * Windows: `\Program Files\fuji`
pub const FUJI_DIR: &str = if cfg!(windows) {
	"\\Program Files\\fuji"
} else {
	"/opt/fuji"
};

/// Wrapper for [`entrypoint`], which takes in additional arguments for a shorthand / alias.
///
/// # Arguments
///
/// * `extras`: Additional arguments to append in-between `fuji` and the rest of the user's args.
///
/// # Errors
///
/// Error type: Dynamic (see [`anyhow::Error`]).
///
/// Error value(s):
///
/// * Always: Propagated up from [`entrypoint`].
///
/// # Returns
///
/// Return type: [`Result<()>`]
///
/// Return value(s):
///
/// * Always: Propagated up from [`entrypoint`].
///
/// # Examples
///
/// Creating a shorthand / alias for `fuji foo bar baz`:
/// ```
/// use fuji::alias_entrypoint;
///
/// // Becomes 'fuji foo bar baz <user args here>'
/// alias_entrypoint(&["foo".into(), "bar".into(), "baz".into()]).unwrap();
/// ```
///
/// Creating a shorthand / alias for `fuji manage jvm preset`:
/// ```
/// use fuji::alias_entrypoint;
///
/// // Becomes 'fuji manage jvm preset <user args here>'
/// alias_entrypoint(&["manage".into(), "jvm".into(), "preset".into()]).unwrap();
/// ```
pub fn alias_entrypoint(extras: &[OsString]) -> Result<()> {
	let mut args: Vec<OsString> = vec!["fuji".into()];
	args.extend_from_slice(extras);
	args.extend_from_slice(&args_os().skip(1).collect::<Vec<OsString>>());
	entrypoint(Arguments::parse_from(args))
}

/// A `main`-like function, which takes in [`Arguments`] and executes the operation(s) specified in them.
///
/// # Arguments
///
/// * `args`: The [`Arguments`] to execute using.
///
/// # Errors
///
/// Error type: Dynamic (see [`anyhow::Error`]).
///
/// Error value(s):
///
/// * If [`Arguments::command`][`field@Arguments::command`] is [`Some`]:
/// 	* Propagated up from the following functions (if they are called):
/// 		* [`cmd_link`][`cmd_link()`]
///			* [`cmd_manage`][`cmd_manage()`]
/// 		* [`cmd_man`][`cmd_man()`]
///
/// # Panics
///
/// * A `const` panic will occur if this function is called on Windows.  Windows support is not yet ready, so this is a countermeasure to prevent premature usage.
///
/// # Returns
///
/// Return type: [`Result<()>`]
///
/// Return value(s):
///
/// * If [`Arguments::command`][`field@Arguments::command`] is [`None`]: [`Ok`]
/// * If [`Arguments::command`][`field@Arguments::command`] is [`Some`]:
/// 	* Propagated up from the following functions (if they are called):
/// 		* [`cmd_link`][`cmd_link()`]
///			* [`cmd_manage`][`cmd_manage()`]
/// 		* [`cmd_man`][`cmd_man()`]
///
/// # Examples
///
/// ```
/// use clap::Parser;
///
/// use fuji::cli::Arguments;
/// use fuji::entrypoint;
///
/// entrypoint(Arguments::parse()).unwrap();
/// ```
pub fn entrypoint(args: Arguments) -> Result<()> {
	// dbg!(env!("CARGO_PKG_NAME"));
	// dbg!(env!("CARGO_PKG_VERSION"));
	const {
		// Ever heard of "Never judge a book by is cover" ?
		// It's about how you should judge based on the content of something, not what is on the outside
		// Windows saw that, and said "Okay, but what if we made file extensions matter for execution instead?"
		assert!(cfg!(not(windows)), "https://learn.microsoft.com/en-gb/windows/wsl/install/");
	};
	#[cfg(feature = "dev")] {
		use std::env::{set_var, var};

		if var("RUST_BACKTRACE").is_err() {
			unsafe {
				set_var("RUST_BACKTRACE", "1");
			};
		};
	};
	args.command.map_or_else(
		|| Ok(()),
		|command: Cmd| match command {
			#[cfg(any(not(windows), feature = "multi-os"))]
			Cmd::Link { .. } => cmd_link(command),
			Cmd::Manage { .. } => cmd_manage(command),
			Cmd::Manual { .. } => cmd_man(command),
		},
	)
}