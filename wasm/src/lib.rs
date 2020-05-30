#[macro_use]
extern crate serde_derive;
extern crate wasm_bindgen;
mod course;
mod flubber;
mod point;
mod spline;
mod vector;

use course::Course;
use point::Point;
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
struct CourseSvg {
    pub fairway_svg: String,
    pub rough_svg: String,
    pub green_center: Point,
    pub tee_center: Point,
}

#[wasm_bindgen]
pub fn generate_course() -> JsValue {
    let course = Course::new();
    return JsValue::from_serde(&course).unwrap();
}

#[wasm_bindgen]
pub fn generate_svg_course() -> JsValue {
    let course = Course::new();
    let course_svg = CourseSvg {
        fairway_svg: generate_svg(
            course
                .fairway_outline
                .iter()
                .map(|p| Point { x: p.y, y: p.x })
                .collect(),
        ),
        rough_svg: generate_svg(
            course
                .rough_outline
                .iter()
                .map(|p| Point { x: p.y, y: p.x })
                .collect(),
        ),
        green_center: course.green_center,
        tee_center: course.tee_center,
    };
    return JsValue::from_serde(&course_svg).unwrap();
}

fn generate_svg(from_vec: Vec<Point>) -> String {
    let mut path = format!("M {} {} ", from_vec[0].x, from_vec[0].y);
    let is_length_even = from_vec.len() % 2 == 0;

    // points
    let mut i = 0;
    while i < from_vec.len() - (if is_length_even { 3 } else { 2 }) {
        let p1 = from_vec[i + 2];
        let mid_point = Point::new((from_vec[i].x + p1.x) / 2., (from_vec[i].y + p1.y) / 2.);
        let p0 = Point::new(
            from_vec[i + 1].x + (from_vec[i + 1].x - mid_point.x),
            from_vec[i + 1].y + (from_vec[i + 1].y - mid_point.y),
        );
        path = format!("{} Q {} {} {} {} ", path, p0.x, p0.y, p1.x, p1.y);
        i += 2;
    }
    // last
    if is_length_even {
        let p1 = from_vec[0];
        let mid_point = Point::new(
            (from_vec[from_vec.len() - 2].x + p1.x) / 2.,
            (from_vec[from_vec.len() - 2].y + p1.y) / 2.,
        );
        let p0 = Point::new(
            from_vec[from_vec.len() - 1].x + (from_vec[from_vec.len() - 1].x - mid_point.x),
            from_vec[from_vec.len() - 1].y + (from_vec[from_vec.len() - 1].y - mid_point.y),
        );
        path = format!("{} Q {} {} {} {} ", path, p0.x, p0.y, p1.x, p1.y);
    }

    return path;
}

#[wasm_bindgen]
pub fn interpolate(_t: f64) {
    // TODO
}
