use wasm_bindgen::prelude::*;
use web_sys::console;
use browser_storage_sync as settings;

#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue>{
//    settings::set_string("test", "value").await.unwrap();

    let result = settings::get().await?;
    console::log_1(&format!("{:?}", result).into());
    Ok(())
}
