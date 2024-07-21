use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Player {
    #[serde(rename = "Name")]
    name: String,

    #[serde(rename = "Position")]
    position: String,

    #[serde(rename = "DOB")]
    dob: String,

    #[serde(rename = "Nationality")]
    nationality: String,

    #[serde(rename = "Kit Number")]
    kit: u8,
}

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: Subcommand,
}

#[derive(Debug, Parser)]
pub enum Subcommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,

    #[arg(short, long, default_value = "output.json")]
    // the default_value'work is  "output.json".into()
    pub output: String,

    #[arg(short, long, default_value_t = ',')] // default_value_t's work is assign ',' to delimiter
    pub delimiter: char,

    #[arg(long, default_value_t = true)]
    pub header: bool,
}

fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if std::path::Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exists")
    }
}
