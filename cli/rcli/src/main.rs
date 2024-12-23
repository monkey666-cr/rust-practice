use clap::Parser;
use rcli::{get_reader, process_decode, process_encode, process_genpass, read_csv, write_data};
use rcli::{Base64SubCommand, Opts, SubCommand};

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
        SubCommand::Base64(subcommand) => match subcommand {
            Base64SubCommand::Encode(opts) => {
                let mut reader = get_reader(&opts.input);
                let res = process_encode(&mut reader, opts.format);
                println!("编码结果: {}", res);
            }
            Base64SubCommand::Decode(opts) => {
                let mut reader = get_reader(&opts.input);
                let res = process_decode(&mut reader, opts.format);
                println!("解码结果: {}", res);
            }
        },
    }
}
