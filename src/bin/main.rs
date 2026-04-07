use anyhow::Result;

use clap::Parser;

use fuji::cli::Arguments;
use fuji::entrypoint;

/// `fuji`
pub fn main() -> Result<()> {
	entrypoint(Arguments::parse())
}