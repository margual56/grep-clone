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
	#[structopt(short = "i", long = "input", parse(from_os_str))]
    input: Option<std::path::PathBuf>,
}

fn main() -> Result<()> {
    let args = Cli::from_args();
	
	let reader: Box<dyn BufRead> = match args.input {
        None => Box::new(BufReader::new(std::io::stdin())),
        Some(path) => Box::new(BufReader::new(File::open(path)?))
    };

	grepclone::find_matches(reader, &args.pattern, &mut std::io::stdout())?;

	Ok(())
} 