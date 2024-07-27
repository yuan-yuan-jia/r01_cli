mod cli;
mod process;

pub use cli::{
    Base64DecodeOpts, Base64EncodeOpts, Base64Format, Base64SubCommand, CsvOpts, Opts, Subcommand,
};
pub use process::{process_csv_2_json, process_decode, process_encode, process_genpass};
