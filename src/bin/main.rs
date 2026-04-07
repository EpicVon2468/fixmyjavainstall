#![allow(clippy::tabs_in_doc_comments)]
use clap::Parser;

use fuji::cli::Arguments;

// TODO: https://crates.io/crates/anyhow/
pub fn main() {
	fuji::entrypoint(Arguments::parse());
}