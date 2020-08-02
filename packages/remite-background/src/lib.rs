use wasm_bindgen::prelude::*;
use web_sys::console;
use settings;
use serde_json::json;

#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue>{
    let test_data = JsValue::from_serde(&json!({"abc": "def"})).unwrap();
    settings::set(&test_data).await.unwrap();

    let result = settings::get().await;
    console::log_1(&format!("{:?}", result).into());
    Ok(())
}
