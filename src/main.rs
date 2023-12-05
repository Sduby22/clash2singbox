mod conversion;
mod converters;
mod cli;

use clap::Parser;


fn main() {
    let cli = cli::Args::parse();
    cli.run();
}