// rcli csv -i input.csv -o output.json --header -d ','
use clap::Parser;
use rcli::process_csv_2_json;
use rcli::{Opts, Subcommand};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        Subcommand::Csv(opts) => {
            process_csv_2_json(&opts.input, &opts.output)?;
        }
    }

    Ok(())
}
