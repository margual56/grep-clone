use structopt::StructOpt;
use std::fs::File;
use std::io::{
	BufReader,
	BufRead,
	Write as IoWrite,
};
use anyhow::{Context, Result};


#[derive(StructOpt)]
struct Cli {
	pattern: String,
	#[structopt(
		short = "-f",
		long = "--file",
		parse(from_os_str))]
	path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::from_args();
	let io_file = File::open(&args.path).with_context(|| format!("Could not read file {:?}", &args.path))?;
	let file = BufReader::new(io_file);

	find_matches(file, &args.pattern, &mut std::io::stdout())?;

	Ok(())
}

fn find_matches(content: BufReader<File>, pattern: &str, mut writer: impl IoWrite) -> Result<()>  {
    for line in content.lines() {
		let l = line.unwrap();
		if l.to_lowercase().contains(pattern) {
			writeln!(writer, "{}", l).with_context(|| format!("Could not write to the buffer. Maybe it is full"))?;
		}
    }

	Ok(())
}