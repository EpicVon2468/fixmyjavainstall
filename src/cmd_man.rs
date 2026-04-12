use std::fs::{File, create_dir_all, remove_dir_all};
use std::io::Write;
use std::iter::{Cloned, Filter};
use std::path::Path;

use anyhow::{Context, Result};

use clap::{Arg, Command, CommandFactory};
use clap_mangen::Man;
use clap_mangen::roff::{Roff, roman};

use flate2::Compression;
use flate2::read::GzEncoder;

use crate::cli::{Arguments, Cmd};
use crate::wrong_cmd;

pub fn cmd_man(cmd: Cmd) -> Result<()> {
	let Cmd::Manual {
		man_dir,
	}: Cmd = cmd else {
		wrong_cmd!(cmd_man);
	};
	let dir: &Path = &man_dir.join("man8");
	if dir.exists() {
		remove_dir_all(dir).context("remove_dir_all")?;
	};
	if !dir.exists() {
		create_dir_all(dir).context("create_dir_all")?;
	};
	dump_manual(Arguments::command(), dir)
}

/// Based off [`clap_mangen::generate_to`]
fn dump_manual<P: AsRef<Path>>(cmd: Command, out_dir: P) -> Result<()> {
	fn generate(parent: Command, out_dir: &Path) -> Result<()> {
		let children: Cloned<Filter<_, _>> = parent
			.get_subcommands()
			.filter(|c: &&Command| !c.is_hide_set())
			.cloned();
		for child in children {
			generate(child, out_dir)?;
		};

		let man: Man = Man::new(parent.clone()).section("8").date("2026-04-07");

		let mut output: GzEncoder<File> = GzEncoder::new(
			File::create_new(out_dir.join(man.get_filename()).with_added_extension("gz"))
				.context("create man_file.gz")?,
			Compression::default(),
		);
		render0(&parent, &man, &mut output)?;
		render_subcommands(&parent, &mut output)?;
		render1(&parent, &man, &mut output)?;
		output.flush().context("flush")?;

		Ok(())
	}

	let mut cmd: Command = cmd.disable_help_subcommand(true);
	cmd.build();
	generate(cmd, out_dir.as_ref())
}

fn render0(cmd: &Command, man: &Man, mut output: &mut GzEncoder<File>) -> Result<()> {
	man.render_title(&mut output).context("title")?;
	man.render_name_section(&mut output).context("name")?;
	man.render_synopsis_section(&mut output).context("synopsis")?;
	man.render_description_section(&mut output).context("description")?;
	if cmd.get_arguments().any(|a: &Arg| !a.is_hide_set()) {
		man.render_options_section(&mut output).context("options")?;
	};
	Ok(())
}

/// Slight modification of [`Man::render_subcommands_section`] to fix display names
///
/// TODO: PR `clap_mangen` with minimal fix?
fn render_subcommands(cmd: &Command, mut output: &mut GzEncoder<File>) -> Result<()> {
	if cmd.get_subcommands().any(|c: &Command| !c.is_hide_set()) {
		let mut roff: Roff = Roff::default();
		roff.control(
			"SH",
			[cmd.get_subcommand_help_heading().unwrap_or("SUBCOMMANDS")],
		);
		let mut sorted_subcommands: Vec<&Command> =
			cmd.get_subcommands().filter(|s| !s.is_hide_set()).collect();
		sorted_subcommands.sort_by_key(|c| (c.get_display_order(), c.get_name()));
		for sub in sorted_subcommands {
			roff.control("TP", []);
			// the built-in implementation of this part is broken
			// fuji-manage will try to resolve fuji-jvm as though it were still called fuji-manage-jvm
			let name: String = sub
				.get_display_name()
				.map(str::to_string)
				.unwrap_or_else(|| {
					format!(
						"{}-{}",
						cmd.get_display_name().unwrap_or_else(|| cmd.get_name()),
						sub.get_name(),
					)
				}) + "(8)";
			roff.text([roman(name)]);
			if let Some(about) = sub.get_about().or_else(|| sub.get_long_about()) {
				for line in about.to_string().lines() {
					roff.text([roman(line)]);
				};
			};
		};
		roff.to_writer(&mut output).context("subcommands")?;
	};
	Ok(())
}

fn render1(cmd: &Command, man: &Man, mut output: &mut GzEncoder<File>) -> Result<()> {
	if cmd.get_after_long_help().is_some() || cmd.get_after_help().is_some() {
		man.render_extra_section(&mut output).context("extra")?;
	};
	if has_version(cmd) {
		man.render_version_section(&mut output).context("version")?;
	};
	if cmd.get_author().is_some() {
		man.render_authors_section(&mut output).context("authors")?;
	};
	Ok(())
}

fn has_version(cmd: &Command) -> bool {
	cmd.get_version()
		.or_else(|| cmd.get_long_version())
		.is_some()
}