pub mod product;
mod utils;

use wasm_bindgen::prelude::*;
use web_sys::console;
use crate::product::*;
use crate::utils::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {

    set_panic_hook();

    console::log_1(&JsValue::from_str("Hello world!"));

    Ok(())

}

#[wasm_bindgen]
extern {
    pub fn log(s: &str);
}

#[wasm_bindgen]
pub fn product_csv_to_generate(data: &str) {

    let mut m = Products::new();
    let mut r = csv::ReaderBuilder::new().delimiter(b',').from_reader(data.as_bytes());
    let mut id: usize = 0;

    for record in r.records() {
        let record = record.unwrap();
        m.add_product(
            Product {
                id: id,
                name: record[0].to_string(),
                cost: record[1].parse().unwrap(),
            }
        );
        id = id + 1;
    }

}
