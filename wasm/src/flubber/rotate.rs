use crate::flubber::math::distance;
use crate::point::Point;

pub fn rotate(ring: &mut Vec<Point>, to_ring: &mut Vec<Point>) {
    let len = ring.len();
    let mut min = 1e9;
    let mut best_offset = 0;
    let mut sum_of_squares;

    let mut offset = 0;
    while offset < len {
        sum_of_squares = 0.;

        for (i, p) in to_ring.iter().enumerate() {
            let d = distance(ring[offset + i % len], *p);
            sum_of_squares += d * d;
        }

        if sum_of_squares < min {
            min = sum_of_squares;
            best_offset = offset;
        }

        if best_offset > 0 {
            ring.rotate_left(best_offset);
        }

        offset += 1;
    }
}
