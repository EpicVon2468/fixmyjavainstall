#![cfg(feature = "dev")]

use std::fs::{create_dir, remove_dir_all};
use std::io::Result;
use std::path::Path;

use clap::{CommandFactory, Command};
use clap_mangen::{Man};

use crate::cli::{Arguments, Cmd};

pub fn cmd_man(_cmd: Cmd) -> Result<()> {
	let dir: &Path = &Path::new(env!("PWD")).join("man");
	if dir.exists() {
		remove_dir_all(dir)?;
	};
	if !dir.exists() {
		create_dir(dir)?;
	};
	dump_manual(Arguments::command(), dir)
}

fn dump_manual<P: AsRef<Path>>(cmd: Command, out_dir: P) -> Result<()> {
	fn generate(parent: Command, out_dir: &Path) -> Result<()> {
		for child in parent.get_subcommands().filter(|s| !s.is_hide_set()).cloned() {
			generate(child, out_dir)?;
		};
		Man::new(parent)
			.section("8")
			.date("2026-04-06")
			.generate_to(out_dir)?;
		Ok(())
	}

	let mut cmd: Command = cmd.disable_help_subcommand(true);
	cmd.build();
	generate(cmd, out_dir.as_ref())
}