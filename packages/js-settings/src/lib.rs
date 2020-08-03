use wasm_bindgen::prelude::*;
use serde_json::json;
use std::collections::HashMap;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace= ["browser", "storage", "sync"], catch)]
    pub async fn get() -> Result<JsValue, JsValue>;
    #[wasm_bindgen(js_namespace= ["browser", "storage", "sync"], catch)]
    pub async fn set(values: &JsValue) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(js_namespace = ["browser", "storage", "sync"], catch)]
    pub async fn remove(key: &str) -> Result<JsValue, JsValue>;
}

pub async fn set_map(key: &str, value: HashMap<String, String>) -> Result<JsValue, JsValue> {
    let setting = JsValue::from_serde(&json!({ key: value })).unwrap();
    set(&setting).await
}

pub async fn set_string(key: &str, value: &str) -> Result<JsValue, JsValue> {
    let setting = JsValue::from_serde(&json!({key: value})).unwrap();
    set(&setting).await
}