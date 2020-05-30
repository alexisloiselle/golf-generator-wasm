use crate::flubber::math::{distance, point_along, polygon_length};
use crate::point::Point;

pub fn add_points(ring:&mut Vec<Point>, num_points: usize) {
    let desired_length = ring.len() + num_points;
    let step = polygon_length(ring) / num_points as f64;

    let mut i = 0;
    let mut cursor: f64 = 0.;
    let mut insert_at = step / 2.;

    while ring.len() < desired_length {
        let a = ring[i];
        let b = ring[(i + 1) % ring.len()];
        let segment = distance(a, b);

        if insert_at <= cursor as f64 + segment {
            ring.insert(
                i + 1,
                if segment > 0. {
                    point_along(a, b, (insert_at - cursor) / segment)
                } else {
                    a.clone()
                },
            );
            insert_at += step;
            continue;
        }

        cursor += segment;
        i += 1;
    }
}

pub fn bisect(ring: &mut Vec<Point>, max_segment_length: f64) {
    let mut i = 0;
    while i < ring.len() {
        let a = ring[i];
        let mut b = if i == ring.len() - 1 {
            ring[0]
        } else {
            ring[i + 1]
        };
        while distance(a, b) > max_segment_length {
            b = point_along(a, b, 0.5);
            ring.insert(i + 1, b)
        }
        i += 1
    }
}
