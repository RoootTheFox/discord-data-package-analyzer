use std::path::PathBuf;

use clap::Parser;

mod dc_structs;
mod errors;
mod meow;

#[derive(Parser, Debug)]
struct Args {
    #[arg(default_value = "package")]
    path: PathBuf,
}

fn main() {
    let args = Args::parse();

    println!("args {:#?}", args);

    if let Ok(()) = meow::analyze(args.path) {
        println!("done");
    }
}
