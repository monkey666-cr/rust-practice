use std::fs;

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use clap::Parser;
use rcli::{
    get_content, get_reader, process_decode, process_encode, process_genpass, process_http_serve,
    process_text_key_generate, process_text_sign, process_text_verify, read_csv, write_data,
};
use rcli::{Base64SubCommand, HttpSubCommand, Opts, SubCommand, TextSubCommand};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

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
            let res = process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.numbers,
                opts.symbols,
            );
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
        SubCommand::Text(subcommand) => match subcommand {
            TextSubCommand::Sign(opts) => {
                let mut input_reader = get_reader(&opts.input);
                let key = get_content(&opts.key).unwrap();

                let res = process_text_sign(&mut input_reader, &key, opts.format);
                let encoded = URL_SAFE_NO_PAD.encode(res.unwrap());
                println!("签名结果: {:?}", encoded);
            }
            TextSubCommand::Verify(opts) => {
                let mut input_reader = get_reader(&opts.input);
                let key = get_content(&opts.key).unwrap();
                let decoded = URL_SAFE_NO_PAD.decode(&opts.signature).unwrap();
                let res =
                    process_text_verify(&mut input_reader, &key, &decoded, opts.format).unwrap();
                if res {
                    println!("验证成功")
                } else {
                    print!("验证失败")
                }
            }
            TextSubCommand::Generate(opts) => {
                let key = process_text_key_generate(opts.format).unwrap();
                for (k, v) in key {
                    fs::write(opts.output_path.join(k), v).unwrap();
                }
                println!("{:?}", opts);
            }
        },
        SubCommand::Http(subcommand) => match subcommand {
            HttpSubCommand::Serve(opts) => {
                process_http_serve(opts.dir, opts.port).await.unwrap();
            }
        },
    }
}
