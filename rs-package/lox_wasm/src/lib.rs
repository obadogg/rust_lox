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
    let now = js_sys::Date::now();
    lox_compiler::interpret(&code, Some(log_fn));
    web_sys::console::log_1(&format!("耗时:{}s", (js_sys::Date::now() - now) / 1000_f64).into());
}
