//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use wasm_bindgen::JsValue;

extern crate webcsv;
use webcsv::CsvFile;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
pub fn load_csv() {
    let mut csv_file = CsvFile::new("/Users/sam/webcsv/tests/AGNC.csv".to_string());
    csv_file.get(JsValue::from(0), JsValue::from(10));
}