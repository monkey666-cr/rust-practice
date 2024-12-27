use clap::Parser;
use rcli::CmdExector;
use rcli::Opts;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let opts = Opts::parse();

    opts.cmd.execute().await.unwrap();
}
