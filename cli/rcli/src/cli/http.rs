use std::path::PathBuf;

use clap::Parser;
use enum_dispatch::enum_dispatch;

use crate::{process_http_serve, CmdExector};

#[derive(Parser, Debug)]
#[enum_dispatch(CmdExector)]
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

impl CmdExector for HttpServeOpts {
    async fn execute(&self) -> anyhow::Result<()> {
        process_http_serve(self.dir.clone(), self.port)
            .await
            .unwrap();
        Ok(())
    }
}
