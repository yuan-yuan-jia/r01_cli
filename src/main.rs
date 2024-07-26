// rcli csv -i input.csv -o output.json --header -d ','
use clap::Parser;
use rcli::process_csv_2_json;
use rcli::{Opts, Subcommand};

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
    }

    Ok(())
}
