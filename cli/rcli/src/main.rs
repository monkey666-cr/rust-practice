use clap::Parser;
use rcli::{read_csv, write_json};
use rcli::{Opts, SubCommand};

fn main() {
    let opts = Opts::parse();

    match opts.cmd {
        SubCommand::Csv(csv_opts) => {
            let res = read_csv(&csv_opts.input);
            write_json(&csv_opts.output, &res);
        }
    }
}
