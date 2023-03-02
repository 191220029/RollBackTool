use std::{fs::File, io::BufReader, path::PathBuf};

use reqwest;
use anyhow;

pub mod cache;

pub async fn request_html(url: &String) -> anyhow::Result<String> {
    let resp = reqwest::get(url).await?;

    // println!("Response: {:?} {}", resp.version(), resp.status());
    // println!("Headers: {:#?}\n", resp.headers());
    // println!("Body:{:#?}",resp.text().await?);
    
    println!("successfully get {}", url);

    Ok(resp.text().await?)
}


pub fn read_urls(file: &PathBuf) -> anyhow::Result<Vec<String>> {
    let fptr = File::open(file)?;
    let reader = BufReader::new(fptr);
    let urls = serde_json::from_reader(reader)?;
    Ok(urls)
}