use wasm_bindgen::prelude::*;
use web_sys::console;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
console_error_panic_hook::set_once();

#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue>{
    console::log_1(&"Welcome to the background js.".into());
    Ok(())
}
