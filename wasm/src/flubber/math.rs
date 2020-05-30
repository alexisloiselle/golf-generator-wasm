use crate::point::Point;

pub fn distance(a: Point, b: Point) -> f64 {
    return ((a.x - b.x) * (a.x - b.x) + (a.y - b.y) * (a.y - b.y)).sqrt();
}

pub fn point_along(a: Point, b: Point, pct: f64) -> Point {
    return Point {
        x: a.x + (b.x - a.x) * pct,
        y: a.y + (b.y - a.y) * pct,
    };
}

pub fn same_point(a: Point, b: Point) -> bool {
    return distance(a, b) < 1e-9;
}

pub fn interpolate_points(
    a: &mut Vec<Point>,
    b: &mut Vec<Point>,
) -> Box<dyn Fn(f64) -> Vec<Point>> {
    let interpolators: Vec<Box<dyn Fn(f64) -> Point>> = a
        .iter()
        .enumerate()
        .map(|(i, point)| interpolate_point(*point, b[i]))
        .collect();

    return Box::new(move |t| interpolators.iter().map(|inter| inter(t)).collect());
}

pub fn interpolate_point(a: Point, b: Point) -> Box<dyn Fn(f64) -> Point> {
    return Box::new(move |t| Point {
        x: a.x + t * (b.x - a.x),
        y: a.y + t * (b.y - a.y),
    });
}

pub fn polygon_centroid(polygon: Vec<Point>) -> Point {
    if non_zero_area(polygon.clone()) {
        if polygon.len() == 1 {
            Point::new(polygon[0].x, polygon[0].y)
        } else {
            let mut sum_x = 0.;
            let mut sum_y = 0.;
            let mut total_length = 0.;
            for ps in polygon.windows(2) {
                let segment_len = distance(ps[0], ps[1]);
                let (x1, y1, x2, y2) = (ps[0].x, ps[0].y, ps[1].x, ps[1].y);
                total_length = total_length + segment_len;
                sum_x = sum_x + segment_len * ((x1 + x2) / (2.));
                sum_y = sum_y + segment_len * ((y1 + y2) / (2.));
            }
            Point::new(sum_x / total_length, sum_y / total_length)
        }
    } else {
        Point::new(
            (polygon[0].x + polygon[polygon.len() - 1].x) / 2.,
            (polygon[0].y + polygon[polygon.len() - 1].y) / 2.,
        )
    }
}

fn non_zero_area(polygon: Vec<Point>) -> bool {
    let mut i = 0;
    while i < polygon.len() {
        let a = polygon[i];
        let b = polygon[i + 1];
        let c = polygon[i + 2];

        if a.x * (b.x - c.x) + b.x * (c.x - a.x) + c.x * (a.x - b.x) > 1e-9 {
            return true;
        }

        i += 2
    }

    return false;
}

pub fn polygon_area(polygon: Vec<Point>) -> f64 {
    if polygon.is_empty() || polygon.len() == 1 {
        return 0.;
    }
    let mut tmp = 0.;
    for ps in polygon.windows(2) {
        tmp = tmp + (ps[0].x * ps[1].y - ps[1].x * ps[0].y);
    }
    tmp / (2.)
}

pub fn polygon_length(polygon: &Vec<Point>) -> f64 {
    polygon
        .windows(2)
        .fold(0., |total_length, p| total_length + distance(p[0], p[1]))
}
