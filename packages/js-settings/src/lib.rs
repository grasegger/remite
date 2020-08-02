use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace= ["browser", "storage", "sync"], js_name = get)]
    async fn getBrowserSettings() -> JsValue;
    #[wasm_bindgen(js_namespace= ["browser", "storage", "sync"], js_name = set)]
    async fn setBrowserSettings(values: &JsValue) -> JsValue;
}

pub async fn get() -> Result<JsValue, JsValue> {
    let result = getBrowserSettings().await;
    Ok(result)
}

pub async fn set(values: &JsValue) -> Result<JsValue, JsValue> {
    let result = setBrowserSettings(values).await;
    Ok(result)
}
