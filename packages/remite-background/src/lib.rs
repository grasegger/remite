use wasm_bindgen::prelude::*;
use web_sys::console;
use browser_storage_sync as settings;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue>{
    console_error_panic_hook::set_once();
    console::log_1(&"Welcome to the background js.".into());

    let settings = settings::get().await.unwrap();

    console::log_1(&settings.into());
    Ok(())
}
