use structopt::StructOpt;
use std::fs::File;
use std::io::{
	BufReader,
	BufRead
};
use std::error::Error;

#[derive(StructOpt)]
struct Cli {
	pattern: String,
	#[structopt(
		short = "-f",
		long = "--file",
		parse(from_os_str))]
	path: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();
	let file = match File::open(&args.path) {
		Ok(file) => { BufReader::new(file) },
		Err(_error) => { Err("Can't read file") }
	};

	for line in file.lines() {
		let l = line.unwrap();
		if l.to_lowercase().contains(&args.pattern) {
			println!("{}", l);
		}
	}
}
