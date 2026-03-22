mod commands;
mod cli;
mod cmd_install;

use std::env::set_var;

use clap::Parser;

use crate::cli::{Arguments, Cmd};
use crate::cmd_install::cmd_install;

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
			Cmd::Install { .. } => {
				cmd_install(command).unwrap();
			},
			Cmd::Foo { .. } => todo!(),
		};
	};
}