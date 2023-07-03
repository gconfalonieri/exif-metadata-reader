extern crate wasm_bindgen;
extern crate wee_alloc;

use exif::{Reader};
use std::{collections::HashMap, io::Cursor};
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn metadata_reader(vector: Vec<u8>) ->  String {
    let mut metadata_vector: HashMap<String, String> = HashMap::new();
    let exifreader = Reader::new();
    let exif = exifreader
                .read_from_container(&mut Cursor::new(vector.clone()))
                .expect("Error reading EXIF data");
    for f in exif.fields() {
        metadata_vector.insert(
            f.tag.to_string(),
            f.display_value().with_unit(&exif).to_string()
        );
    }
    return (serde_json::to_string(&metadata_vector).unwrap()).to_string();
}
