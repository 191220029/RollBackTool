use std::{path::PathBuf, fs::{read_dir}};

use anyhow::bail;

pub fn discover_htmls(dir: &PathBuf) -> anyhow::Result<Vec<PathBuf>> {
    let mut htmls = vec![];

    for entry in read_dir(dir)? {
        let entry = entry?;

        let name = match entry.file_name().into_string() {
            Ok(s) => s,
            Err(e) => bail!("non-utf8 html file name: {:?}", e),
        };
        if !entry.file_type()?.is_file() {
            println!("{} is not a file, will be skipped.", name);
            continue;
        }

        htmls.push(PathBuf::from(dir.join(name)));
    }

    Ok(htmls)
}