use crate::csvs::Player;
use csv::Reader;
use std::fs;

pub fn process_csv_2_json(filepath: &str, outputfilepath: &str) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(filepath)?;
    let mut ret = Vec::with_capacity(128);
    for result in reader.deserialize() {
        let record: Player = result?;
        //println!("{:?}", record)
        ret.push(record);
    }
    let json = serde_json::to_string_pretty(&ret)?;
    Ok(fs::write(outputfilepath, json)?)
}
