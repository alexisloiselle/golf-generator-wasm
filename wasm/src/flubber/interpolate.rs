use crate::flubber::add::add_points;
use crate::flubber::math::interpolate_points;
use crate::flubber::normalize::normalize_ring;
use crate::flubber::rotate::rotate;
use crate::point::Point;

pub fn interpolate(
    from_shape: Vec<Point>,
    to_shape: Vec<Point>,
    max_segment_length: Option<f64>,
) -> Box<dyn Fn(f64) -> Vec<Point>> {
    let mut from_ring = normalize_ring(from_shape, max_segment_length.unwrap_or(10.));
    let mut to_ring = normalize_ring(to_shape, max_segment_length.unwrap_or(10.));
    return interpolate_ring(&mut from_ring, &mut to_ring);
}

pub fn interpolate_ring(
    from_ring: &mut Vec<Point>,
    to_ring: &mut Vec<Point>,
) -> Box<dyn Fn(f64) -> Vec<Point>> {
    let diff = from_ring.len() as i32 - to_ring.len() as i32;

    add_points(from_ring, if diff < 0 { (diff * -1) as usize } else { 0 });
    add_points(to_ring, if diff > 0 { diff as usize } else { 0 });

    rotate(from_ring, to_ring);

    return interpolate_points(from_ring, to_ring);
}
