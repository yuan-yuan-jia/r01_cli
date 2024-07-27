use csv::Reader;
use serde::{Deserialize, Serialize};
use std::fs;

use crate::cli::OutputFormat;

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

pub fn process_csv_2_json(
    filepath: &str,
    output: String,
    format: OutputFormat,
) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(filepath)?;
    let mut ret = Vec::with_capacity(128);
    let headers = reader.headers()?.clone();
    for result in reader.records() {
        let record = result?;
        //println!("{:?}", record)
        let json_value = headers
            .iter()
            .zip(record.iter())
            .collect::<serde_json::Value>();
        ret.push(json_value);
    }
    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
    };
    fs::write(output, content)?;
    Ok(())
}
