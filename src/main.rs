use env_logger;
use log::{info, warn};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{stdout, BufReader, BufWriter, Write};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}
fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    info!("starting up");
    info!("warn");

    let args = Cli::from_args();
    let content = std::fs::read_to_string(&args.path).expect("could not read file");
    let reader = BufReader::new(File::open(&args.path)?);

    let out = stdout();
    let mut out = BufWriter::new(out.lock());

    for line in reader.lines() {
        let content = line?;
        if content.contains(&args.pattern) {
            // println! {"{}", content};
            writeln!(out, "{}", content)?;
        }
    }
    Ok(())
}
