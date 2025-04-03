use wasm_bindgen::prelude::*;
use wson_rs::{dumps, loads, validate};

#[wasm_bindgen]
pub fn wson_validate(input: &str) -> bool {
    validate(input)
}

#[wasm_bindgen]
pub fn wson_parse(input: &str) -> String {
    match loads(input) {
        Ok(data) => dumps(&data).unwrap_or_else(|_| "serialize error".to_string()),
        Err(_) => "parse error".to_string(),
    }
}