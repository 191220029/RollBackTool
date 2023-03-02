use std::thread::sleep_ms;

use clap::Parser;
use cli::{Cli, Commands};
use roller::{url_reader::discover_htmls, pr_parser::parse_htmls};
use url_resolver::{request_html, cache::cache_writer, read_urls};

mod url_resolver;
mod roller;
mod cli;

#[tokio::main]
async fn main() -> Result<(),reqwest::Error> {
    let args = Cli::parse();

    match args.subcommand {
        Commands::RollBackLocal { 
            html_cache_dir, 
            local_repo, 
            out_dir 
        } => {
            let prs = match parse_htmls(match discover_htmls(&html_cache_dir){
                Ok(htmls) => htmls,
                Err(err) => {
                    eprintln!("Fail to discover htmls.\n{:?}", err);
                    return Ok(());
                }
            }) {
                Ok(prs) => prs,
                Err(err) => {
                    eprintln!("Fail to parse htmls.\n{:?}", err);
                    return Ok(());
                }
            };

                        
        },
        Commands::CachePR { 
            html_cache_dir, 
            pr_urls
        } => {
            match read_urls(&pr_urls) {
                Ok(urls) => {
                    for url in &urls {
                        sleep_ms(1000);
                        let mut id = 0u32; 
                        url.split('/').for_each(|s| {
                            let mut flag = true;
                            for c in s.chars() {
                                flag &= c.is_numeric();
                            }
                            if flag == true {
                                id = match s.parse::<u32>(){
                                    Ok(u) => u,
                                    Err(_) => return,
                                };
                            }
                        });
                        match request_html(url).await {
                            anyhow::Result::Ok(body) =>
                                match cache_writer(&html_cache_dir, id, &body){
                                    Ok(_) => (),
                                    Err(err) => eprintln!("Fail to cache urls.\n{}", err),
                                }
                            err => 
                                println!("{:?}", err),
                        };
                    }
                },
                Err(err) => eprintln!("Fail to read pr_urls file.\n{}", err),
            }
        },
    }

    Ok(())
}
