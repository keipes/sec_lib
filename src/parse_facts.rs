use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use serde::{Serialize, Deserialize};


pub const MSFT_FACTS: &str = "resources/test-data/CIK0000789019.json";
pub const FB_FACTS: &str = "resources/test-data/CIK0001326801.json";
pub const AMZN_FACTS: &str = "resources/test-data/CIK0001018724.json";
pub const AAPL_FACTS: &str = "resources/test-data/CIK0000320193.json";
pub const NFLX_FACTS: &str = "resources/test-data/CIK0001065280.json";
pub const GOOG_FACTS: &str = "resources/test-data/CIK0001652044.json";


/*
TODO: Can we enable strict parsing, so we know when an element is not successfully mapped to a data
 structure?
 */
pub fn open_file(file: &str) -> Filer {
    let file = File::open(Path::new(AMZN_FACTS)).unwrap();
    let mut buf_reader = BufReader::new(file);
    serde_json::from_reader(buf_reader).unwrap()
}

#[derive(Serialize, Deserialize)]
pub struct Filer {
    pub cik: u64,
    pub entityName: String,
    pub facts: DocumentEntityInformation
}

#[derive(Serialize, Deserialize)]
pub struct DocumentEntityInformation {
    pub dei: HashMap<String, FinancialElement>, // do you like this map structure? is it the only way to map fields with variable keys to a struct? what about the "other" annotation in serde
    #[serde(alias = "us-gaap")]
    pub gaap: HashMap<String, FinancialElement>
}

#[derive(Serialize, Deserialize)]
pub struct FinancialElement {
    pub label: String,
    pub description: String,
    pub units: HashMap<String, Vec<Fact>>
}

#[derive(Serialize, Deserialize)]
pub struct Fact {
    #[serde(default)]
    pub start: String, // TODO: make this a date?
    pub end: String, // TODO: make this a date?
    pub val: f64, // floating point? I don't think so
    pub accn: String, // accession number? (doc which fact is from)
    pub fy: u32,
    pub fp: String,
    pub form: String,
    pub filed: String // TODO: make this a date!
}
