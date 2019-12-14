use std::path::Path;
use std::io::prelude::*;

enum OperationMode {
    toml2json,
    json2toml,
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<_> = std::env::args()
        .collect();

    let path = Path::new(args[0].as_str());
    let name = path.file_name();
    let source = match args.len() {
        2 => args[1].as_str(),
        _ => {
            eprintln!("Unsupported command line arguments!");
            std::process::exit(1);
        }
    };

    let mode = match name.unwrap().to_str() {
        Some("toml2json") => OperationMode::toml2json,
        Some("json2toml") => OperationMode::json2toml,
        _ => {
            eprintln!("Unsupported operating mode!");
            std::process::exit(1);
        }
    };

    let file = std::fs::File::open(source)?;
    let reader = std::io::BufReader::new(file);
    // let mut lines: Vec<String> = Vec::new();

    let mut all_lines = String::new();
    for line in reader.lines() {
        let line = line?;
        all_lines = format!("{}{}\n", all_lines, line);
    }

    let result = match mode {
        OperationMode::toml2json => toml2json(&all_lines),
        OperationMode::json2toml => json2toml(&all_lines),
    };

    println!("{}", result);

    return Ok(());
}

fn toml2json(toml: &str) -> String {
    let object: toml::value::Value = toml::from_str(toml)
        .expect("Failed to parse input as valid TOML!");
    let json = serde_json::to_string_pretty(&object)
        .expect("Failed to convert object to JSON!");

    json
}

fn json2toml(json: &str) -> String {
    let object: serde_json::value::Value = serde_json::from_str(json)
        .expect("Failed to parse input as valid JSON!");
    let toml = toml::to_string_pretty(&object)
        .expect("Failed to convert object to TOML!");

    toml
}

