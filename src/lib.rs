use std::io::{
	BufRead,
	Write as IoWrite,
};
use anyhow::{Context, Result};

pub fn find_matches(content: Box<dyn BufRead>, pattern: &str, mut writer: impl IoWrite) -> Result<()>  {
    for line in content.lines() {
		let l = line.unwrap();
		if l.to_lowercase().contains(pattern) {
			writeln!(writer, "{}", l).with_context(|| format!("Could not write to the buffer. Maybe it is full"))?;
		}
    }

	Ok(())
}