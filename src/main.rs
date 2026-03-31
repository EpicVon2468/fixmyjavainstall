mod commands;
mod cli;
mod cmd_link;
mod cmd_manage;
mod macros;
mod jvm;
mod arch;
mod kotlin;

use std::env::set_var;

use clap::Parser;

use crate::cli::{Arguments, Cmd};
use crate::cmd_link::cmd_link;
use crate::cmd_manage::cmd_manage;

// I'll think about it.
#[cfg(not(windows))]
fn main() {
	// Doing this when trying to run the binary didn't work
	unsafe {
		set_var("RUST_BACKTRACE", "1");
	};
	let arguments: Arguments = Arguments::parse();
	if let Some(command) = &arguments.command {
		match command {
			Cmd::Link { .. } => {
				cmd_link(command).unwrap();
			},
			Cmd::Manage { .. } => {
				cmd_manage(command).unwrap();
			},
		};
	};
}