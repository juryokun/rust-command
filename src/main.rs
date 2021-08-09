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
    // let content = std::fs::read_to_string(&args.path).expect("could not read file");
    let reader = BufReader::new(File::open(&args.path)?);

    let out = stdout();
    let mut out = BufWriter::new(out.lock());

    find_matches(reader, &args.pattern, &mut out)
    // let out = stdout();
    // let mut out = BufWriter::new(out.lock());

    // for line in reader.lines() {
    //     let content = line?;
    //     if content.contains(&args.pattern) {
    //         // println! {"{}", content};
    //         writeln!(out, "{}", content)?;
    //     }
    // }
}

fn find_matches(
    content: BufReader<std::fs::File>,
    pattern: &str,
    mut writer: impl std::io::Write,
) -> std::io::Result<()> {
    for line in content.lines() {
        let line = line?;
        if line.contains(pattern) {
            writeln!(writer, "{}", line)?;
        }
    }
    Ok(())
}

#[test]
fn test_find_matches() {
    let file_name = "foo.txt";
    let mut f = File::create(file_name).unwrap();
    f.write_all(b"Hello, world!\nHello, rust");
    f.sync_data();
    let content = BufReader::new(File::open(file_name).unwrap());

    let mut out = Vec::new();

    find_matches(content, "world", &mut out);
    assert_eq!(out, b"Hello, world!\n");

    std::fs::remove_file(file_name);
}
