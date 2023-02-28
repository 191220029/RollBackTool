use std::{path::PathBuf, fs::File, io::BufReader};

pub fn read_urls(file: &PathBuf) -> anyhow::Result<Vec<String>> {
    let fptr = File::open(file)?;
    let reader = BufReader::new(fptr);
    let urls = serde_json::from_reader(reader)?;
    Ok(urls)
}