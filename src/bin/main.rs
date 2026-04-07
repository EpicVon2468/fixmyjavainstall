use clap::Parser;

use fuji::cli::Arguments;
use fuji::entrypoint;

/// `fuji`
pub fn main() {
	entrypoint(Arguments::parse());
}