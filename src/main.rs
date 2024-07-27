// rcli csv -i input.csv -o output.json --header -d ','
use clap::Parser;
use rcli::process_csv_2_json;
use rcli::process_genpass;
use rcli::{process_decode, process_encode};
use rcli::{Base64SubCommand, Opts, Subcommand};

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
    }

    Ok(())
}
