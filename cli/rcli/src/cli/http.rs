use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
pub enum HttpSubCommand {
    #[command(about = "Start a http server")]
    Serve(HttpServeOpts),
}

#[derive(Debug, Parser)]
pub struct HttpServeOpts {
    #[arg(short, long, default_value = "127.0.0.1")]
    pub dir: PathBuf,
    #[arg(short, long, default_value = "8000")]
    pub port: u16,
}
