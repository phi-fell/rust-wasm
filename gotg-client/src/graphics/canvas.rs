use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use std::f64;

pub struct Canvas {
    pub element: web_sys::HtmlCanvasElement,
    pub context: web_sys::CanvasRenderingContext2d,
    pub width: u32,
    pub height: u32,
}

impl Canvas {
    pub fn new(document: &web_sys::Document) -> Result<Canvas, JsValue> {
        let element = document
            .create_element("canvas")?
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .or(Err(JsValue::from_str("Created element is not a canvas!")))?;
        let context = element
            .get_context("2d")?
            .ok_or(JsValue::from_str(
                "Could not retrieve 2d context from canvas!",
            ))?
            .dyn_into::<web_sys::CanvasRenderingContext2d>()?;
        let width = element.width();
        let height = element.height();
        Ok(Canvas {
            element,
            context,
            width,
            height,
        })
    }
    pub fn set_pos(&self) {
        self.set_style("position:absolute;left:0;top:0;width:100%;height:100%;");
    }
    /*fn get_style(&self) -> String {
        self.element.style().css_text()
    }*/
    fn set_style(&self, style: &str) {
        self.element.style().set_css_text(style);
    }
    /*fn add_style(&self, style: &str) {
        self.set_style(&(self.get_style() + style));
    }*/
    pub fn resize(&self) {
        let width = self.element.client_width();
        let height = self.element.client_height();
        if width < 0 {
            web_sys::console::log_1(&"Canvas has negative client width!".into());
        } else if height < 0 {
            web_sys::console::log_1(&"Canvas has negative client height!".into());
        } else {
            let width: u32 = width as u32;
            let height: u32 = height as u32;
            self.element.set_width(width);
            self.element.set_height(height);
        }
    }
    pub fn draw(&self) {
        self.context.begin_path();

        // Draw the outer circle.
        self.context
            .arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
            .unwrap();

        // Draw the mouth.
        self.context.move_to(110.0, 75.0);
        self.context
            .arc(75.0, 75.0, 35.0, 0.0, f64::consts::PI)
            .unwrap();

        // Draw the left eye.
        self.context.move_to(65.0, 65.0);
        self.context
            .arc(60.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
            .unwrap();

        // Draw the right eye.
        self.context.move_to(95.0, 65.0);
        self.context
            .arc(90.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
            .unwrap();

        self.context.stroke();
    }
}
