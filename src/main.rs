use schred_lib::*;
use clap::Parser;
use std::path::Path;

/// Secure erase files and directories
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to erase
    path: String,

    /// Print verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Delete the file(s) after erasure
    #[arg(short, long)]
    deallocate: bool,

    /// Recursive shredding of directories
    #[arg(short, long)]
    recursive: bool,

    /// Number of passes using random data
    #[arg(long, default_value_t = 2)]
    nrand: u8,

    /// Number of passes using zeros
    #[arg(long, default_value_t = 1)]
    nzero: u8,
}

fn main() {
    let args = Args::parse();

    let s = Shredder::new(ShredOptions {
        verbose: args.verbose,
        deallocate: args.deallocate,
        recursive: args.recursive,
        rand_passes: args.nrand,
        zero_passes: args.nzero,
        ..ShredOptions::default()
    });

    s.shred(Path::new(&args.path)).unwrap();
}
