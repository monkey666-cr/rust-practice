use std::fs;

use serde::{Deserialize, Serialize};

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

pub fn read_csv(path: &str) -> Vec<Record> {
    let mut rdr = csv::Reader::from_path(path).unwrap();
    // for result in rdr.records() {
    //     let record = result.expect("a CSV record");
    //     println!("{:?}", record);
    // }
    let mut res = Vec::with_capacity(128);
    for result in rdr.deserialize() {
        let record: Record = result.unwrap();
        res.push(record);
    }

    res
}

pub fn write_json(path: &str, res: &Vec<Record>) {
    let json = serde_json::to_string_pretty(&res);
    fs::write(path, json.unwrap()).expect("写入json数据失败")
}
