use clap::Parser;
use rcli::{
    Base64SubCommand, HttpSubCommand, Opts, SubCommand, TextFormat, TextSubCommand, process_csv,
    process_decode, process_encode, process_genpass, process_http_serve, process_sign,
    process_text_generator, process_verify,
};
use std::fs;
use zxcvbn::zxcvbn;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, &output, opts.format)?
        }
        SubCommand::GenPass(opts) => {
            let password = process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.numbers,
                opts.symbols,
            )?;
            println!("{:?}", password);

            let estimate = zxcvbn(&password, &[]);
            eprintln!("密码强度: {}", estimate.score());
        }
        SubCommand::Base64(sub_cmd) => match sub_cmd {
            Base64SubCommand::Encode(opts) => {
                let encoded = process_encode(&opts.input, opts.format)?;
                println!("{}", encoded);
            }
            Base64SubCommand::Decode(opts) => {
                let decoded = process_decode(&opts.input, opts.format)?;
                let decoded = String::from_utf8(decoded)?;
                println!("{}", decoded);
            }
        },
        SubCommand::Text(sub_cmd) => match sub_cmd {
            TextSubCommand::Sign(opts) => {
                let signed = process_sign(&opts.input, &opts.key, opts.format)?;

                println!("{}", signed);
            }
            TextSubCommand::Verify(opts) => {
                process_verify(&opts.input, &opts.key, opts.format, &opts.sign)?
            }
            TextSubCommand::Generate(opts) => {
                let key = process_text_generator(opts.format)?;
                match opts.format {
                    TextFormat::Blake3 => {
                        let name = opts.output.join("blake3.txt");
                        fs::write(name, &key[0])?;
                    }
                    TextFormat::Ed25519 => {
                        let name = &opts.output;
                        fs::write(name.join("sk.pem"), &key[0])?;
                        fs::write(name.join("pk.pem"), &key[1])?;
                    }
                }
            }
        },
        SubCommand::Http(sub_cmd) => match sub_cmd {
            HttpSubCommand::Serve(opts) => process_http_serve(opts.dir, opts.port).await?,
        },
    }

    Ok(())
}
