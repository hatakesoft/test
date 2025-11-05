use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn draw_circle() {
    let window = window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();

    let ctx = canvas
        .get_context("2d").unwrap();
    
    ctx.begin_path();
    ctx.arc(150.0, 75.0, 50.0, 0.0, std::f64::consts::PI * 2.0).unwrap();
    ctx.set_fill_style(&JsValue::from_str("blue"));
    ctx.fill();
}
