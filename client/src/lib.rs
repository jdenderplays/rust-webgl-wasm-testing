use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen(start)]
pub fn greet() {
    alert(&format!("Hello, {}!", "Bob"));
}
