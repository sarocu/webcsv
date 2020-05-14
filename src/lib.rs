extern crate cfg_if;
use cfg_if::cfg_if;

mod utils;

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

use std::io;

extern crate csv;
use csv::{Reader};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
cfg_if! {
    if #[cfg(feature = "wee_alloc")] {
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

impl CsvFile {
    pub fn new(
        file_path: String
    ) -> CsvFile {        
        let new_csv = CsvFile {
            csv_file: file_path.to_owned(),
        };
        
        new_csv
    }

    // pub fn get(
    //     skip: i32,
    //     limit: i32,
    // ) {
    //     // return a slice of the csv file
    // }

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