use clap::Parser;
use cli::{Cli, Commands};
use web::request_html;

mod web;
mod roller;
mod cli;

#[tokio::main]
async fn main() -> Result<(),reqwest::Error> {
    let args = Cli::parse();

    match args.subcommand {
        Commands::RollBack { 
            url_file, 
            local_repo, 
            out_dir 
        } => {
            let urls = roller::url_reader::read_urls(&url_file);
            for url in urls {
                match request_html(&url).await {
                    anyhow::Result::Ok(_) =>
                        println!("Ok"),
                    err => 
                        println!("{:?}", err),
                }
            }
        }
    }

    

    Ok(())
}
