use structopt::StructOpt;
use std::fs::File;
use std::io::{
	BufReader,
};
use anyhow::{Context, Result};


#[derive(StructOpt)]
struct Cli {
	pattern: String,
	#[structopt(parse(from_os_str))]
	path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::from_args();
	let io_file = File::open(&args.path).with_context(|| format!("Could not read file {:?}", &args.path))?;
	let file = BufReader::new(io_file);

	grepclone::find_matches(file, &args.pattern, &mut std::io::stdout())?;

	Ok(())
}