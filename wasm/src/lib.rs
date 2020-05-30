#[macro_use]
extern crate serde_derive;
extern crate wasm_bindgen;
mod course;
mod flubber;
mod point;
mod spline;
mod vector;

use course::Course;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn generate_course() -> JsValue {
    let course = Course::new();
    return JsValue::from_serde(&course).unwrap();
}

#[wasm_bindgen]
pub fn interpolate(_t: f64) {
    // TODO
}
