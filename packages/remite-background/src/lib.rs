use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue>{
    console::log_1(&"Welcome to the background js.".into());
    Ok(())
}
