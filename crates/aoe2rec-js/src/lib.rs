mod utils;

use aoe2rec::Savegame;
use bytes::Bytes;

use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn parse_rec(buffer: js_sys::ArrayBuffer) -> JsValue {
    utils::set_panic_hook();

    let input_rec = Uint8Array::new(&buffer).to_vec();
    let rec = Savegame::from_bytes(Bytes::from(input_rec)).unwrap();
    serde_wasm_bindgen::to_value(&rec).unwrap()
}
