use structopt::StructOpt;
use std::fs::File;
use std::io::{
	BufReader,
	BufRead
};
use anyhow::{Result};


#[derive(StructOpt)]
struct Cli {
	pattern: String,
	#[structopt(skip, parse(from_os_str))]
	path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::from_args();
	
	let reader: Box<dyn BufRead> = match File::open(&args.path) {
        Err(_error) => Box::new(BufReader::new(std::io::stdin())),
        Ok(file) => Box::new(BufReader::new(file))
    };

	grepclone::find_matches(reader, &args.pattern, &mut std::io::stdout())?;

	Ok(())
} 