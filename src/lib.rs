use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

mod graphics;

use graphics::canvas::Canvas;

#[wasm_bindgen]
pub fn start_game() {
    match app_main() {
        Ok(_) => web_sys::console::log_1(&"Application finished running.".into()),
        Err(error) => {
            web_sys::console::log_1(&"Application crashed!".into());
            web_sys::console::log_1(&error);
        }
    };
}

fn app_main() -> Result<(), JsValue> {
    web_sys::console::log_1(&"Starting app...".into());
    let window = web_sys::window().expect("No global window exists!");
    let document = window.document().expect("No window document exists!");
    let body = document
        .create_element("body")?
        .dyn_into::<web_sys::HtmlElement>()?;
    document.set_body(Some(&body));
    let canvas = Canvas::new(&document)?;
    canvas.set_pos();
    body.append_child(&canvas.element)?;
    canvas.resize();
    canvas.draw();

    Ok(())
}
