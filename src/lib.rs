mod opts;
mod process;

pub use opts::{CsvOpts, Opts, Subcommand};
pub use process::process_csv_2_json;
pub use process::process_genpass;
