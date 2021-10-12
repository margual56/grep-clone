use structopt::StructOpt;
use std::fs::File;
use std::io::{
	BufReader,
	BufRead
};
use anyhow::{Result};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream};


#[derive(StructOpt)]
#[structopt(about = "Just a simple grep clone programmed in Rust. \ngrepclone takes either an input file (`-i` option) or reads the stdin if no file is passed as an argument.")]
struct Cli {
	/// Pattern(s) to search for. Required parameter.
	patterns: Vec<String>,

	/// Input file (Optional)
	#[structopt(short = "i", long = "input", parse(from_os_str))]
    input: Option<std::path::PathBuf>,

	/// Wether to print with colored output or not
	#[structopt(short = "c", long = "color", parse(from_flag))]
	color: bool,
}

fn main() -> Result<()> {
    let args = Cli::from_args();
	
	// Detect wether a file has been passed as an argument
	// or if it has to be read from stdin
	let reader: Box<dyn BufRead> = match args.input {
        None => Box::new(BufReader::new(std::io::stdin())),
        Some(path) => Box::new(BufReader::new(File::open(path)?))
    };

	// If the color flag has been passed, then create a special stdout with colors
	if args.color {
		let stdout = StandardStream::stdout(ColorChoice::Always);

		let mut color = ColorSpec::new();
		color.set_fg(Some(Color::Red));

		grepclone::find_matches_color_word(reader, args.patterns, stdout, &color)?;
	}else{
		grepclone::find_matches(reader, args.patterns, std::io::stdout())?;
	}

	Ok(())
} 