use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console::log_1(&JsValue::from_str("World"));
    Ok(())
}

pub fn greet(name: &str) {
    console::debug_1(&JsValue::from_str(&format!("Hello, {}!", name)));
}
