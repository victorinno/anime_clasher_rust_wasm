mod player;
mod utils;
use js_sys::JsString;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use crate::player::player::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    let p = Player {
        name: "teste".to_string(),
    };
    let s_slice: &str = &format!("Hello, {}!", p.get_name());
    alert(s_slice);
}
