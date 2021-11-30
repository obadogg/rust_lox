extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, World!");
}

fn log_fn(s: String) {
    log(s.as_str())
}

#[wasm_bindgen]
pub fn interpret_lox(code: String) {
    lox_compiler::interpret(&code, Some(log_fn))
}
