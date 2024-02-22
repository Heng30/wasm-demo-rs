mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn init_lib() {
    utils::set_panic_hook();
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, maps!");
}

#[wasm_bindgen]
pub fn say_hello(name: &str) -> js_sys::JsString {
    let msg = format!("Hello, {name}!");
    log(&msg);
    msg.into()
}
