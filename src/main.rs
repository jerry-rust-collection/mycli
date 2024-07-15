use anyhow::Result;
use clap::Parser;
use csv::Reader;
use std::{fs, path::Path};
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[command(name = "mycli", version, author, about, long_about = None)]
struct Opts {
    #[command(subcommand)]
    cmd: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
}

#[derive(Parser, Debug)]
struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    input: String,

    #[arg(short, long, default_value = "output.json")]
    output: String,

    #[arg(short, long, default_value_t = ',')]
    delimiter: char,

    #[arg(long, default_value_t = true)]
    header: bool,
}

fn verify_input_file(filename: &str) -> Result<String, String> {
    if !Path::new(filename).exists() {
        return Err(format!("File not found: {}", filename));
    }

    // println!("File found: {}", filename);
    Ok(filename.into())
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Play {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}
fn process_csv(input: &str, output: &str) -> Result<()> {
    println!("Processing CSV file: {}", input);
    println!("Output file: {}", output);

    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    for result in reader.deserialize() {
        let record: Play = result?;
        // println!("{:?}", record);
        ret.push(record);
    }
    let json = serde_json::to_string_pretty(&ret)?;
    println!("{}", json);
    fs::write(output, json)?;
    Ok(())
}

fn main() {
    let args: Opts = Opts::parse();
    match args.cmd {
        SubCommand::Csv(args) => {
            if let Err(err) = process_csv(&args.input, &args.output) {
                eprintln!("Error: {}", err);
            }
        }
    }
}
