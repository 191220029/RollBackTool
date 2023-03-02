use std::{path::PathBuf, fs::{create_dir, create_dir_all, File}, io::Write};

use anyhow::{Ok, Context};

pub fn cache_writer(dir: &PathBuf, pr_id: u32, text: &String) -> anyhow::Result<()> {
    create_dir_all(dir)?;

    let mut fptr = File::create(dir.join(pr_id.to_string() + ".cache"))
        .with_context(|| format!("Fail to create cache file {:?}", dir.join(pr_id.to_string()).join(".cache")))?;

    fptr.write_all(text.as_bytes())?;
    
    Ok(())
}