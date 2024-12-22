use clap::Parser;
use rcli::{process_genpass, read_csv, write_data};
use rcli::{Opts, SubCommand};

fn main() {
    let opts = Opts::parse();

    match opts.cmd {
        SubCommand::Csv(csv_opts) => {
            let res = read_csv(&csv_opts.input);

            // 输出文件名称
            let output = if let Some(output) = csv_opts.output {
                output
            } else {
                format!("output.{}", csv_opts.format)
            };

            write_data(&output, csv_opts.format, &res);
        }
        SubCommand::GenPass(opts) => {
            let res = process_genpass(opts);
            println!("随机生成的密码为: {}", res);
        }
    }
}
