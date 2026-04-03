mod commands;
mod cli;
mod cmd_link;
mod cmd_manage;
mod macros;
mod jvm;
mod arch;
mod kotlin;
mod os;
#[cfg(windows)]
mod win_link;

use std::env::set_var;

use clap::Parser;

use crate::cli::{Arguments, Cmd};
#[cfg(any(not(windows), feature = "multi_os"))]
use crate::cmd_link::cmd_link;
use crate::cmd_manage::cmd_manage;

pub const FUJI_DIR: &str = if cfg!(windows) { "\\Program Files\\fuji" } else { "/opt/fuji" };

fn main() {
	if cfg!(windows) {
		// Ever heard of "Never judge a book by is cover" ?
		// It's about how you should judge based on the content of something, not what is on the outside
		// Windows saw that, and said "Okay, but what if we made file extensions matter for execution instead?"
		panic!("https://learn.microsoft.com/en-gb/windows/wsl/install");
	};
	#[allow(unreachable_code)]
	// Doing this when trying to run the binary didn't work
	unsafe {
		set_var("RUST_BACKTRACE", "1");
	};
	let arguments: Arguments = Arguments::parse();
	if let Some(command) = &arguments.command {
		match command {
			#[cfg(any(not(windows), feature = "multi_os"))]
			Cmd::Link { .. } => cmd_link(command).unwrap(),
			Cmd::Manage { .. } => cmd_manage(command).unwrap(),
		};
	};
}