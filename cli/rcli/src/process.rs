use std::fs;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::opts::OutputFormat;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[allow(dead_code)]
pub struct Record {
    latitude: f64,
    longitude: f64,
    population: Option<u64>,
    #[serde(rename = "CITY")]
    city: String,
    state: String,
}

pub fn read_csv(path: &str) -> Vec<Value> {
    let mut rdr = csv::Reader::from_path(path).unwrap();
    // for result in rdr.records() {
    //     let record = result.expect("a CSV record");
    //     println!("{:?}", record);
    // }
    let mut res = Vec::with_capacity(128);
    let headers = rdr.headers().unwrap().clone();
    for result in rdr.records() {
        // let record: Record = result.unwrap();
        let record = result.unwrap();
        let json_value = headers.iter().zip(record.iter()).collect::<Value>();
        res.push(json_value);
    }

    res
}

pub fn write_data(path: &str, format: OutputFormat, res: &Vec<Value>) {
    match format {
        OutputFormat::Json => {
            fs::write(path, serde_json::to_string_pretty(&res).unwrap()).expect("写入json数据失败");
        }
        OutputFormat::Yaml => {
            fs::write(path, serde_yaml::to_string(&res).unwrap()).expect("写入yaml数据失败");
        }
    }
}
