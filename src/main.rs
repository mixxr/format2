use std::collections::HashMap;
use std::io::{self};

use std::fs::File;
use std::io::BufReader;
use std::io::SeekFrom;
use std::io::prelude::*;
use std::time::{Instant};

mod definitions;
use clap::Parser;
use definitions::args::Args;

// #[derive(Debug, serde::Deserialize, serde::Serialize)]
// struct Certificate {
//     isin: String,
//     name: String,
//     tickers: String,
//     start_date: String,
//     end_date: String,
// }

// #[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
// struct Quote {
//     isin: String,
//     obs_dt: String,
//     ask: f32,
//     bid: f32,
//     currency: String,
// }

// fn insert_certificates(cs: Vec<Certificate>) -> Vec<String> {
//     let db = env.d1("DB")?;
//     let query = db.prepare("INSERT INTO certificate VALUES (?,?,?,?,?)");
//     let mut result = Vec<String>::new();
//     for c in cs {
//         let r = query.bind(&[c.isin.into(), c.name.into(), c.tickers.into(), c.start_date.into(), c.end_date.into()])?.run().await?;
//         result.push(r);
//     }
//     result
// }

// fn insert_quotes(cs: Vec<Quote>) -> Vec<String> {
//     let db = env.d1("DB")?;
//     let query = db.prepare("INSERT INTO quote VALUES (?,?,?,?,?)");
//     let mut result = Vec<String>::new();
//     for c in cs {
//         let r = query.bind(&[c.isin.into(), c.obs_dt.into(), c.ask.into(), c.bid.into(), c.currency.into()])?.run().await;
//         result.push(r);
//     }
//     result
// }

fn read_kv_file(path: &str) -> io::Result<std::collections::HashMap<String, String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut map = std::collections::HashMap::new();

    for (line_num, line) in reader.lines().enumerate() {
        let line = line?; // Handle I/O errors per line

        // Skip empty lines and comments
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        // Split into key and value
        if let Some((key, value)) = trimmed.split_once('=') {
            map.insert(key.trim().to_string(), value.trim().to_string());
        } else {
            log::error!("Warning: Invalid format at line {}: '{}'", line_num + 1, trimmed);
        }
    }

    Ok(map)
}

fn extract_value(file_path: &str,content_str: &str, pattern: &str) -> String {
    match pattern {
        p if p.starts_with("Path") => extract_value_from_path(file_path, pattern.split('.').nth(1).unwrap_or("0").parse::<usize>().unwrap()),
        // XPath is in format XPath.elementN.element2.element1, extract_value_from_xpath needs content_str and pattern starting from the first "."
        // p if p.starts_with("XPath") => extract_value_from_xpath(content_str, pattern.split('.').skip(1).collect()),
        _ => extract_value_from_regex(content_str, pattern),
    }
}

fn extract_value_from_regex(content_str: &str, pattern: &str) -> String {
    let rx = regex::Regex::new(&pattern).unwrap();
    println!("String contains 'ask': {}, 'bid': {}, pattern: {}", content_str.contains("ask"), content_str.contains("bid"), pattern);
    let Some(caps) = rx.captures(&content_str) else { return "".to_string()};
    log::debug!("Extracting from Regex: {}, Captures: {:?}, Caps len: {}", pattern, caps, caps.len());
    caps[1].to_string()
}

fn extract_value_from_path(filepath: &str, position: usize) -> String {
    // file_path: "data/issuer/2026-01-01/IT000002"
    // position: 0 => ISIN, 1 => dt
    
    let parts: Vec<&str> = filepath.split(std::path::MAIN_SEPARATOR).collect();
    log::debug!("Extracting from Path: {}, position: {}, parts: {:?}", filepath, position, parts);
    if position >= parts.len() {
        return "".to_string();
    }
    let mut value = parts[parts.len() - position - 1].to_string();
    // if position is 0 (ISIN) then remove file extension if any
    if position == 0 {
        if let Some(dot_pos) = value.find('.') {
            value = value[..dot_pos].to_string();
        }
    }
    value
}


fn read_file_content(path: &str, max_len: usize, seek_first: usize) -> String {
    let f = File::open(path).expect("Can't find file!");
    let mut reader = BufReader::new(f);

    // TODO: seek to skip first bytes (as per parameter, default is 0)
    reader.seek(SeekFrom::Start(seek_first as u64)).unwrap();

    // Read first n bytes
    let mut buffer = vec![0; max_len];
    let bytes_read = reader.read(&mut buffer).unwrap();

    // Trim unused bytes and convert to string (if UTF-8)
    let contents = String::from_utf8_lossy(&buffer[..bytes_read]);
    let cstr = contents.to_string();
    log::debug!("Read {} bytes from file {} ask: {}, bid: {}", bytes_read, path, cstr.contains(r#""ask""#), cstr.contains(r#""bid""#));
    cstr
}

// fn extract_certificates() -> Vec<Certificate> {
//     let mut cs: Vec<Certificate> = Vec::new();
//     for i in 1..10 {
//         let c = Certificate {
//             isin: String::from("IT000002"),
//             name: String::from("Cert 0002"),
//             tickers: String::from("t1000,t2000,t3000,"),
//             start_date: String::from("2026-01-01"),
//             end_date: String::from("2029-01-01"),
//         };
//         cs.push(c);
//     }
//     cs
// }



/*
regex2 --config <issuer>.rx.txt --input-dir output\<dt> --output-format [json|sql|csv] --output-dir <path>
- <path> contains 1 file with entries as defined in <issuer>.rx.txt, eg. <isin>,<dt>,<ask>,<bid>,...
*/
fn main() -> std::process::ExitCode {
    // get current milliseconds since epoch
    let start = Instant::now();
    let args = Args::parse();
    
    env_logger::init();

    println!("Configuration: {:?}, Log Level: {}", args, std::env::var("RUST_LOG").unwrap_or("ERROR".to_string()));

    // check if input dir does not exist then exit with error
    if !std::path::Path::new(&args.input_dir).exists() {
        log::error!("Input directory does not exist: {}", &args.input_dir);
        return std::process::ExitCode::FAILURE;
    }
    // check if ndjson and output-dir is a filepath with extention
    if args.output_format == "ndjson" {
        let path = std::path::Path::new(&args.output_dir);
        if path.is_dir() {
            log::error!("Output directory is a directory but ndjson format requires a file path: {}", &args.output_dir);
            return std::process::ExitCode::FAILURE;
        }
        if path.extension().is_none() {
            log::error!("Output file path does not have an extension: {}", &args.output_dir);
            return std::process::ExitCode::FAILURE;
        }
        // output_dir = dirpath/filename.ext, dirpath will be created anyway
        let dirpath = std::path::Path::new(&args.output_dir).parent().unwrap();
        let _ = std::fs::create_dir_all(&dirpath);
    }
    // check if output dir does not exist then create it
    if args.output_format != "ndjson" && !std::path::Path::new(&args.output_dir).exists() {
        std::fs::create_dir_all(&args.output_dir).unwrap();
    }
    // check if output dir has trailing separator and if not add it
    let mut output_dir = args.output_dir.to_string();
    if args.output_format != "ndjson" && !output_dir.ends_with(std::path::MAIN_SEPARATOR) {
        output_dir.push(std::path::MAIN_SEPARATOR);
    }

    log::info!("Job {}", args.input_dir);
    match read_kv_file(&args.config) {
        Ok(map) => {
            log::debug!("Fields to extract: {} ", map.len());
             let mut skip_first = 0; // default value
            let mut max_len = 5000; // default value
            // read skip and len parameters from config file, if not present use default values
            // config file is named <issuer>.cfg.txt and contains key-value pairs
            let extra_cfg_filepath = args.config.replace(".rx.txt", ".cfg.txt");
            match read_kv_file(&extra_cfg_filepath) {
                Ok(extra_map) => {
                    skip_first = extra_map.get("skip").and_then(|v| v.parse::<usize>().ok()).unwrap_or(skip_first);
                    max_len = extra_map.get("len").and_then(|v| v.parse::<usize>().ok()).unwrap_or(max_len);
                    log::debug!("Extra config: skip_first={}, max_len={}", skip_first, max_len);
                }
                Err(_e) => log::warn!("No extra config file {}: {skip_first}, {max_len}", extra_cfg_filepath),
            }
           
            // for each file in the input dir, extract the fields and print them
            for entry in std::fs::read_dir(&args.input_dir).unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();
                if path.is_file() {
                    log::info!("Processing {}", path.display());
                    let content_str = read_file_content(path.to_str().unwrap(), max_len, skip_first);
                    let mut fields: HashMap<String, String> = HashMap::new();
                    for (key, value) in &map {
                        log::debug!("Extracting {} => {}", key, value);
                        fields.insert(key.to_string(), extract_value(path.to_str().unwrap(), &content_str, value));
                    }
                    log::info!("Extracted fields: {:?}", fields);
                    // TODO: serialize fields in accordance with output format (json, sql, csv)
                    match args.output_format.to_lowercase().as_str() {
                        "csv" => {
                            // TODO: create 1 file?
                            // let output_filepath = &[output_dir.to_string(), fields.get("isin").unwrap().to_string(), ".csv".to_string()].concat();
                            // let mut wtr = csv::Writer::from_writer(File::create(output_filepath).unwrap());
                            // wtr.serialize(fields).unwrap();
                            // wtr.flush().unwrap();
                            todo!()
                        }
                        "ndjson" => {
                            let output_filepath = output_dir.to_string();
                            log::debug!("Writing json to {output_filepath}...");
                            // ndJSON is 1 file containing multiple JSON objects, each in a new line
                            let mut file = File::options().append(true).create(true).open(output_filepath).unwrap();
                            serde_json::to_writer(&mut file, &fields).unwrap();
                            // add a new line after each JSON object
                            file.write_all(b"\n").unwrap();

                        }
                        _ => {
                            // default is json => 1 file?
                            let output_filepath = &[output_dir.to_string(), fields.get("isin").unwrap().to_string(), ".json".to_string()].concat();
                            serde_json::to_writer(File::create(output_filepath).unwrap(), &fields).unwrap();
                        }   
                    }
                }
            }
            log::info!("Job completed in {:?}", start.elapsed());
            // let mut fields: HashMap<String, String> = HashMap::new();
            // let content_str = read_file_content(&args.input_dir, max_len);
            // for (key, value) in &map {
            //     log::info!("extracting {} => {}...", key, value);
            //     fields.insert(key.to_string(), extract_value(&args.input_dir, &content_str, value));
            // }
            // log::info!("Extracted fields: {:?}", fields);
        }
        Err(e) => log::error!("Error reading file {}: {}", &args.config, e),
    }
    std::process::ExitCode::SUCCESS
}
