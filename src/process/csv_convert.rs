use crate::OutputFormat;
use anyhow::Result;
use csv::Reader;
use serde_json::Value;
use std::fs;

pub fn process_csv(input: &str, output: &str, format: OutputFormat) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let headers = reader.headers()?.clone();
    let mut ret = Vec::with_capacity(128);
    for result in reader.records() {
        let record = result?;
        let json_value = headers.iter().zip(record.iter()).collect::<Value>();
        ret.push(json_value);
    }

    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
    };
    fs::write(output, content)?;

    Ok(())
}
