extern crate cfg_if;
use cfg_if::cfg_if;

mod utils;

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

use std;
extern crate csv;
use csv::{Position, Reader};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
cfg_if! {
    if #[cfg(feature = "wee_alloc")] {
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

impl CsvFile {
    pub fn new(file_path: String) -> CsvFile {
        let new_csv = CsvFile {
            csv_file: file_path.to_owned(),
        };

        new_csv
    }

    pub fn get(&self, skip: u64, limit: usize) -> std::vec::Vec<csv::StringRecord> {
        // return a slice of the csv file
        let mut csv_reader = Reader::from_path(self.csv_file.to_owned()).unwrap();
        let item_count = csv_reader.headers().iter().count();
        let start = Position::new().set_line(skip).to_owned();
        csv_reader.seek(start.clone());
        let mut rows = vec![csv::StringRecord::default()];
        for index in 1..limit {
            if let Some(row) = csv_reader.records().next() {
                rows.push(row.unwrap())
            }
        }
        return rows;
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
struct CsvFile {
    csv_file: String,
}
