use std::fs::{create_dir, remove_dir_all, File};
use std::io::{Result, Write};
use std::path::Path;

use clap::{Arg, Command, CommandFactory};
use clap_mangen::Man;
use clap_mangen::roff::{roman, Roff};

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

		let man: Man = Man::new(parent.clone())
			.section("8")
			.date("2026-04-06");

		let mut output: GzEncoder<File> = GzEncoder::new(
			File::create_new(out_dir.join(man.get_filename()).with_added_extension("gz")).expect("create man_file.gz"),
			Compression::default(),
		);
		man.render_title(&mut output)?;
		man.render_name_section(&mut output)?;
		man.render_synopsis_section(&mut output)?;
		man.render_description_section(&mut output)?;
		if parent.get_arguments().any(|i: &Arg| !i.is_hide_set()) {
			man.render_options_section(&mut output)?;
		};
		if parent.get_subcommands().any(|i| !i.is_hide_set()) {
			let mut roff = Roff::default();
			roff.control(
				"SH",
				[parent.get_subcommand_help_heading().unwrap_or("SUBCOMMANDS")],
			);
			let mut sorted_subcommands: Vec<&Command> = parent.get_subcommands().filter(|s| !s.is_hide_set()).collect();
			sorted_subcommands.sort_by_key(|c| (c.get_display_order(), c.get_name()));
			for sub in sorted_subcommands {
				roff.control("TP", []);
				let result: String = sub.get_display_name().map(str::to_string).unwrap_or_else(|| {
					format!(
						"{}-{}",
						parent.get_display_name().unwrap_or_else(|| parent.get_name()),
						sub.get_name()
					)
				});
				let name: String = format!("{result}(8)");
				roff.text([roman(name)]);
				if let Some(about) = sub.get_about().or_else(|| sub.get_long_about()) {
					for line in about.to_string().lines() {
						roff.text([roman(line)]);
					};
				};
			};
			roff.to_writer(&mut output)?;
		};
		if parent.get_after_long_help().is_some() || parent.get_after_help().is_some() {
			man.render_extra_section(&mut output)?;
		};
		if parent.get_version().or_else(|| parent.get_long_version()).is_some() {
			man.render_version_section(&mut output)?;
		};
		if parent.get_author().is_some() {
			man.render_authors_section(&mut output)?;
		};
		output.flush().expect("flush");

		Ok(())
	}

	let mut cmd: Command = cmd.disable_help_subcommand(true);
	cmd.build();
	generate(cmd, out_dir.as_ref())
}