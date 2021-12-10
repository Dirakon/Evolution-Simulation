use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(what_to_scream_at: &str) {
    alert(&format!("Fuck you, {}!", what_to_scream_at));
}
