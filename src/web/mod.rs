use reqwest;
use http::{HeaderMap, HeaderValue, header::{COOKIE,USER_AGENT}};
use anyhow;

pub async fn request_html(url: &String) -> anyhow::Result<()> {
    let resp = reqwest::get(url).await?;

    println!("Response: {:?} {}", resp.version(), resp.status());
    println!("Headers: {:#?}\n", resp.headers());
    println!("Body:{:#?}",resp.text().await?);
    Ok(())
}