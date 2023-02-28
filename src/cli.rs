use std::path::PathBuf;

#[derive(Debug, clap::Parser)]
#[clap(about, version, author)]
pub struct Cli {
    #[clap(subcommand)]
    pub subcommand: Commands
}

#[derive(Debug, clap::Subcommand)]
#[clap(rename_all = "snake_case")]
pub enum Commands {
    RollBack {
        // input urls of pull request
        #[clap(long = "url-file")]
        url_file: PathBuf,
    
        // local repository dir
        #[clap(long = "local-repo")]
        local_repo: PathBuf,
    
        // output dir
        #[clap(long = "out-dir")]
        out_dir: PathBuf
    }
}