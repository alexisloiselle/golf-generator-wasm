use crate::flubber::math::{polygon_area, same_point};
use crate::flubber::add::bisect;
use crate::point::Point;

pub fn normalize_ring(ring: Vec<Point>, max_segment_length: f64) -> Vec<Point> {
    let mut points: Vec<Point>;
    let area: f64;

    points = ring.clone();

    if points.len() > 1 && same_point(points[0], points[points.len() - 1]) {
        points.pop();
    }

    area = polygon_area(points.clone());

    if area > 0. {
        points.reverse()
    }

    bisect(&mut points, max_segment_length);

    return points
}
