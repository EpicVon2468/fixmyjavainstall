use std::fs::{create_dir, remove_dir_all, remove_file, File};
use std::io::{copy, BufReader, Result, Write};
use std::path::{Path, PathBuf};

use clap::{Command, CommandFactory};
use clap_mangen::Man;

use flate2::read::GzEncoder;
use flate2::Compression;

use crate::cli::{Arguments, Cmd};
use crate::wrong_cmd;

pub fn cmd_man(cmd: Cmd) -> Result<()> {
	let Cmd::Manual {
		man_dir,
	} = cmd else {
		wrong_cmd!(cmd_man);
	};
	let dir: &Path = &man_dir.canonicalize().expect("canonicalise").join("man8");
	if dir.exists() {
		remove_dir_all(dir).expect("remove_dir_all");
	};
	if !dir.exists() {
		create_dir(dir).expect("create_dir");
	};
	dump_manual(Arguments::command(), dir)
}

fn dump_manual<P: AsRef<Path>>(cmd: Command, out_dir: P) -> Result<()> {
	fn generate(parent: Command, out_dir: &Path) -> Result<()> {
		for child in parent.get_subcommands().filter(|s| !s.is_hide_set()).cloned() {
			generate(child, out_dir)?;
		};

		let man: Man = Man::new(parent)
			.section("8")
			.date("2026-04-06");
		man.generate_to(out_dir).expect("generate_to");

		let man_file: PathBuf = out_dir.join(man.get_filename());
		let mut input: BufReader<File> = BufReader::new(
			File::open(&man_file).expect("open man_file")
		);
		let mut output: GzEncoder<File> = GzEncoder::new(
			File::create_new(man_file.with_added_extension("gz")).expect("open man_file.gz"),
			Compression::default(),
		);
		copy(&mut input, &mut output).expect("copy");
		output.flush().expect("flush");
		remove_file(man_file).expect("remove_file");

		Ok(())
	}

	let mut cmd: Command = cmd.disable_help_subcommand(true);
	cmd.build();
	generate(cmd, out_dir.as_ref())
}