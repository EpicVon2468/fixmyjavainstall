#![allow(clippy::tabs_in_doc_comments)]
use std::env::args_os;
use std::ffi::OsString;

use clap::Parser;

use fuji::cli::Arguments;

pub fn main() {
	let mut new_args: Vec<OsString> = vec!["fuji".into(), "manage".into(), "jvm".into()];
	new_args.extend_from_slice(&args_os().skip(1).collect::<Vec<_>>());
	fuji::entrypoint(Arguments::parse_from(new_args));
}
