// Group lints
#![warn(clippy::pedantic, clippy::nursery, clippy::suspicious)]
// Specific lints
#![warn(
	clippy::as_conversions,
	clippy::min_ident_chars,
	clippy::pattern_type_mismatch,
	clippy::use_self,
	clippy::unused_trait_names,
	clippy::create_dir,
	clippy::exit,
	clippy::float_cmp,
	clippy::float_cmp_const,
	clippy::while_float,
	clippy::integer_division,
	clippy::integer_division_remainder_used,
	clippy::unreadable_literal,
	clippy::unnecessary_literal_bound,
	clippy::missing_const_for_fn,
	clippy::needless_collect,
	clippy::needless_for_each,
	clippy::as_underscore,
	clippy::branches_sharing_code,
	clippy::infinite_loop,
	clippy::linkedlist,
	clippy::pub_use,
	clippy::wildcard_imports,
	clippy::uninlined_format_args,
	clippy::equatable_if_let,
	clippy::enum_glob_use
)]
#![deny(
	clippy::undocumented_unsafe_blocks,
	clippy::multiple_unsafe_ops_per_block,
	reason = "Unsafe bad.  Kill it with fire!"
)]
#![allow(clippy::tabs_in_doc_comments, reason = "Why???  Bad clippy!")]
#![allow(
	clippy::unnecessary_semicolon,
	reason = "Consistency & uniformity looks better!  Bad clippy!"
)]
#![allow(
	clippy::missing_errors_doc,
	clippy::missing_panics_doc,
	reason = "I'll get to writing doc comments when I get to them."
)]
#![allow(
	clippy::doc_markdown,
	reason = "'JetBrains' and 'AdoptOpenJDK' are not identifiers I'm referencing.  Bad clippy!"
)]
#![allow(clippy::default_trait_access, reason = "Shush")]
#![doc = include_str!("../README.md")]
pub mod arch;
pub mod cli;
pub mod cmd_link;
pub mod cmd_man;
pub mod cmd_manage;
pub mod commands;
pub mod fuji_value_enum;
pub mod install_method;
pub mod jvm;
pub mod kotlin;
pub mod macros;
pub mod os;
#[cfg(windows)]
pub mod win_link;

use std::env::args_os;
use std::ffi::OsString;
use std::fs::{File, exists, remove_file};
use std::io::{Write as _, stderr, stdout};
use std::process::{abort, id};

use anyhow::{Context as _, Result};

use clap::Parser as _;

use crate::cli::{FujiArgs, FujiCmd};
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
///
/// # Examples
///
/// ```
/// use std::path::Path;
///
/// use fuji::FUJI_DIR;
///
/// let resolved = Path::new(FUJI_DIR).join("foo").join("bar");
/// ```
pub const FUJI_DIR: &str = cfg_select! {
	unix => "/opt/fuji",
	windows => "\\Program Files\\fuji",
	_ => panic!("Unsupported host!"),
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
/// assert_eq!(alias_entrypoint(&["foo".into(), "bar".into(), "baz".into()]), Ok(()));
/// ```
///
/// Creating a shorthand / alias for `fuji manage jvm preset`:
/// ```
/// use fuji::alias_entrypoint;
///
/// // Becomes 'fuji manage jvm preset <user args here>'
/// assert_eq!(alias_entrypoint(&["manage".into(), "jvm".into(), "preset".into()]), Ok(()));
/// ```
pub fn alias_entrypoint(extras: &[OsString]) -> Result<()> {
	let mut args: Vec<OsString> = vec!["fuji".into()];
	args.extend_from_slice(extras);
	args.extend_from_slice(&args_os().skip(1).collect::<Vec<OsString>>());
	entrypoint(FujiArgs::parse_from(args))
}

/// A `main`-like function, which takes in [`FujiArgs`] and executes the operation(s) specified in them.
///
/// # Arguments
///
/// * `args`: The [`FujiArgs`] to execute using.
///
/// # Errors
///
/// Error type: Dynamic (see [`anyhow::Error`]).
///
/// Error value(s):
///
/// * If [`FujiArgs::command`][`field@FujiArgs::command`] is [`Some`]:
/// 	* Propagated up from the following functions (if they are called):
/// 		* [`cmd_link`][`cmd_link()`]
///			* [`cmd_manage`][`cmd_manage()`]
/// 		* [`cmd_man`][`cmd_man()`]
///
/// # Returns
///
/// Return type: [`Result<()>`]
///
/// Return value(s):
///
/// * If [`FujiArgs::command`][`field@FujiArgs::command`] is [`None`]: [`Ok`]
/// * If [`FujiArgs::command`][`field@FujiArgs::command`] is [`Some`]:
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
/// use fuji::cli::FujiArgs;
/// use fuji::entrypoint;
///
/// assert_eq!(entrypoint(FujiArgs::parse()), Ok(()));
/// ```
pub fn entrypoint(args: FujiArgs) -> Result<()> {
	#[cfg(feature = "dev")]
	// SAFETY:
	// Problem: Mutation of environ can be thread unsafe.
	// Excuses:
	// - I'm not doing multi-threading.
	// - I want a stack trace available in development immediately.
	// - This code can't exactly fail.
	unsafe {
		use std::env::{set_var, var};

		if var("RUST_BACKTRACE").is_err() {
			set_var("RUST_BACKTRACE", "1");
		};
	};
	#[cfg(unix)]
	// SAFETY:
	// Problem:
	// - A user may run as non-root by accident, or due to a lack of knowledge.
	// - To check this, getuid() from `libc` is needed.
	// - `libc` is unsafe.
	// Excuses:
	// - From `man 2 getuid`: "ERRORS These functions are always successful and never modify errno".
	unsafe {
		// SAFETY: The function declarations given below are in line with the header files of `libc`.
		unsafe extern "C" {

			fn getuid() -> u32;
		}

		if getuid() != 0 {
			eprintln!(
				"Fuji ran by non-root user!  If you are not using a permissions manager (i.e. `apparmor`), then this is likely a mistake!"
			);
		};
	};
	#[cfg(target_os = "linux")]
	let lock: File = claim_singleton_process()?;
	let result: Result<()> = args.command.map_or_else(
		|| Ok(()),
		|command: FujiCmd| match command {
			#[cfg(any(not(windows), feature = "multi-os"))]
			FujiCmd::Link { .. } => cmd_link(command),
			FujiCmd::Manage { .. } => cmd_manage(command),
			FujiCmd::Manual { .. } => cmd_man(command),
		},
	);
	#[cfg(target_os = "linux")]
	unclaim_singleton_process(lock)?;
	result
}

pub const LOCK: &str = "/var/lock/fixurjavainstall.lock";

fn claim_singleton_process() -> Result<File> {
	if exists(LOCK).is_ok_and(|exists: bool| exists) {
		eprintln!("Couldn't acquire lockfile {LOCK}!");
		// try to flush, but don't escape back upwards if it fails
		let _ = stdout().flush();
		let _ = stderr().flush();
		abort();
	};
	let mut file: File =
		File::create_new(LOCK).context(format!("Couldn't acquire lockfile {LOCK}!"))?;
	lock!(file);
	writeln!(file, "{}\n", id()).context(format!("Couldn't write to lockfile {LOCK}!"))?;
	Ok(file)
}

#[allow(clippy::needless_pass_by_value, reason = "Not using it anywhere else.")]
fn unclaim_singleton_process(file: File) -> Result<()> {
	unlock!(file);
	remove_file(LOCK).context(format!("Couldn't remove lockfile {LOCK}!"))?;
	Ok(())
}
