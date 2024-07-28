// rcli csv -i input.csv -o output.json --header -d ','
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use clap::Parser;
use rcli::process_csv_2_json;
use rcli::process_genpass;
use rcli::{get_content, get_reader};
use rcli::{process_decode, process_encode, Base64SubCommand, Opts, Subcommand};
use rcli::{process_text_key_generate, process_text_sign, process_text_verify};
use std::fs;

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        Subcommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };

            process_csv_2_json(&opts.input, output, opts.format)?;
        }
        Subcommand::GenPass(opts) => {
            process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
            )?;
        }
        Subcommand::Base64(subcommd) => match subcommd {
            Base64SubCommand::Encode(encode_opts) => {
                process_encode(&encode_opts.input, encode_opts.format)?;
            }
            Base64SubCommand::Decode(decode_opts) => {
                process_decode(&decode_opts.input, decode_opts.format)?;
            }
        },
        Subcommand::Text(cmd) => match cmd {
            rcli::TextSubCommand::Sign(opts) => {
                let mut reader = get_reader(&opts.input)?;
                let key = get_content(&opts.key)?;
                let sig = process_text_sign(&mut reader, &key, opts.format)?;

                //base64 output
                let encoded = URL_SAFE_NO_PAD.encode(sig);
                println!("{}", encoded);
            }
            rcli::TextSubCommand::Verify(opts) => {
                let mut reader = get_reader(&opts.input)?;
                let key = get_content(&opts.key)?;
                let decoded = URL_SAFE_NO_PAD.decode(&opts.sig)?;
                let verified = process_text_verify(&mut reader, &key, &decoded, opts.format)?;

                if verified {
                    println!("✓ Signature verified");
                } else {
                    println!("⚠ Signature not verified");
                }
            }
            rcli::TextSubCommand::Generate(opts) => {
                let key = process_text_key_generate(opts.format)?;
                for (k, v) in key {
                    fs::write(opts.output_path.join(k), v)?;
                }
            }
        },
    }

    Ok(())
}
