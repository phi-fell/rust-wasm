use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
pub fn start_game() {
    app_main().expect("Application Crashed!");
}

fn app_main() -> Result<(), JsValue> {
    web_sys::console::log_1(&"Starting app...".into());
    let window = web_sys::window().expect("No global window exists!");
    let document = window.document().expect("No window document exists!");
    let body = document
        .create_element("body")?
        .dyn_into::<web_sys::HtmlElement>()?;
    document.set_body(Some(&body));

    let val = document.create_element("p")?;
    val.set_inner_html("Hello from Rust!");

    body.append_child(&val)?;

    Ok(())
}
