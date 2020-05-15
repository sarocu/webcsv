extern crate cfg_if;
use cfg_if::cfg_if;

mod utils;

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

use wasm_bindgen_futures::JsFuture;
use web_sys::Response;

use std;
use std::collections::HashMap;
use std::io::SeekFrom;

extern crate csv;
use csv::{Position, Reader};

extern crate serde;

#[macro_use]
extern crate serde_json;

use serde::{Deserialize, Serialize};
use serde_json::Value;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
cfg_if! {
    if #[cfg(feature = "wee_alloc")] {
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
impl CsvFile {
    pub fn new(file_path: String) -> CsvFile {
        let new_csv = CsvFile {
            csv_file: file_path.to_owned(),
        };

        new_csv
    }

    pub async fn get(self, skip: JsValue, limit: JsValue) -> Rows {
        let skip_index: usize = match skip.as_string() {
            None => i32::from(0) as usize,
            Some(skip_option) => skip_option.parse().unwrap(),
        };

        let limit_rows: usize = match limit.as_string() {
            None => i32::from(9999999) as usize, // all rows
            Some(limit_option) => limit_option.parse().unwrap(),
        };
        // return a slice of the csv file
        let csv_string = fetch_file(self.csv_file.to_owned()).await.unwrap();
        let mut csv_reader = Reader::from_reader(csv_string.as_bytes());
        let headers = csv_reader.headers().unwrap().clone();
        // let mut pos = Position::new();
        let mut iter = csv_reader.into_records().skip(skip_index);
        let mut rows = Vec::new();

        // we don't use index but this keeps us within the limit of rows requested
        loop {
            if let Some(row_data) = iter.next() {
                let string_record: Row = row_data.unwrap().deserialize(Some(&headers)).unwrap();
                rows.push(string_record)
            }

            if rows.iter().count() == limit_rows {
                break;
            }
        }

        // let slices = rows.iter().map(|&r| r.as_slice()).collect::<Vec<_>>();
        return Rows {
            headers: serde_json::to_value(headers.as_slice()).unwrap(),
            rows: rows,
        };
    }

    // pub fn count(&self) -> Result<usize, ()> {
    //     // return the total rows
    //     Ok(self.length)
    // }

    // pub fn headers(&self) -> Result<csv::StringRecord, ()> {
    //     // return an array of headers
    //     Ok(self.header_row)
    // }
}

#[wasm_bindgen]
pub struct CsvFile {
    csv_file: String,
}

#[wasm_bindgen]
pub struct Rows {
    headers: Value,
    rows: Vec<Row>,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Row {
    values: HashMap<String, String>,
}

#[wasm_bindgen]
pub async fn fetch_file(file_path: String) -> Result<String, String> {
    let window = web_sys::window().unwrap();
    let file_response = JsFuture::from(window.fetch_with_str(&file_path)).await;
    let file_string: String = match file_response.unwrap().as_string() {
        Some(file_option) => file_option,
        None => String::from(""),
    };
    Ok(file_string)
}
