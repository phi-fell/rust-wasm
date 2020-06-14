use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn start() {
    let window = web_sys::window().expect("No global window exists!");
    let _ = window.alert_with_message(get_str());
    web_sys::console::log_1(&get_str().into());
}

fn get_str() -> &'static str {
    return "Deploy Test!";
}
