use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::io;
use std::process;

#[derive(Debug, Deserialize)]
struct Record {
    search_term: String,
    citation_key: String,
    quote: String,
    category1: String,
    category2: String,
    category3: String,
    category4: String,
    category5: String,
}

fn frequency(mut data: HashMap<String, i32>) -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[0];
    let file = File::open(file_path)?;
    let mut rdr = csv::ReaderBuilder::new().flexible(true).from_reader(file);
    for result in rdr.records() {
        let record = result?;

        //Field 1 and 2 and not codes of interest
        for field in record.iter().skip(1).skip(2) {
            let count = data.entry(field.to_string()).or_insert(0);
            *count += 1;
        }

    }

    for theme in data {
        println!("{:?}", theme);
    }
    Ok(())
}

fn main() {
    let mut data = HashMap::new();
    
    if let Err(err) = frequency(data) {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
