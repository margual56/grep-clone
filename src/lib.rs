use std::io::{
	BufRead,
	Write as IoWrite,
};
use anyhow::{Context, Result};
use termcolor::{ColorSpec, WriteColor};


pub fn find_matches(content: Box<dyn BufRead>, patterns: Vec<String>, mut writer: impl IoWrite) -> Result<()>  {
    for line in content.lines() {
		let l = line.unwrap();
		for pattern in &patterns {
			if l.contains(pattern) {
				writeln!(writer, "{}", l).with_context(|| format!("Could not write to the buffer. Maybe it is full"))?;
			}
		}
    }

	Ok(())
}

pub fn find_matches_color_word(content: Box<dyn BufRead>, patterns: Vec<String>, mut writer: impl WriteColor, color: &ColorSpec) -> Result<()>  {
    for line in content.lines() {
		let l = line.unwrap();
		for pattern in &patterns {
			if l.contains(pattern) {
				let parts: Vec<&str> = l.split(pattern).collect();
				let last = parts[parts.len()-1];

				for part in parts {
					if part == last {continue}
					
					writer.reset()?; // Reset the color
					write!(writer, "{}", part).with_context(|| format!("Could not write to the buffer. Maybe it is full"))?;

					writer.set_color(color)?;
					write!(writer, "{}", pattern).with_context(|| format!("Could not write to the buffer. Maybe it is full"))?;
				}

				writer.reset()?; // Reset the color
				writeln!(writer, "{}", last)?;
			}
		}
	}

	Ok(())
}