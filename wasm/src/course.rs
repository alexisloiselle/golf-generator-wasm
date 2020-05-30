extern crate rand;

use crate::point::Point;
use crate::spline::Spline;
use crate::vector::Vector;
use rand::Rng;

const WIDTH: f64 = 780.;
const HEIGHT: f64 = 780.;

#[derive(Serialize, Clone)]
pub struct Course {
    pub fairway_outline: Vec<Point>,
    pub rough_outline: Vec<Point>,
    pub tee_center: Point,
    pub green_center: Point,
}

impl Course {
    pub fn new() -> Course {
        let base_path = _generate_base_path();

        let fairway = _generate_fairway(_get_anchors(base_path.clone(), 2));
        let rough = _generate_rough(base_path.clone(), _get_anchors(base_path.clone(), 5));

        return Course {
            fairway_outline: _outline_from_path(fairway),
            rough_outline: _outline_from_path(rough),
            tee_center: base_path[base_path.len() - 1],
            green_center: base_path[0],
        };
    }
}

fn _generate_base_path() -> Vec<Point> {
    const H_BOUND: f64 = 240.;
    let upper_bound: f64 = _get_random_float(120., 180.);
    let lower_bound: f64 = _get_random_float(120., 180.);

    let mut path = Vec::new();
    let first_point = Point::new(_get_random_float(H_BOUND, WIDTH - H_BOUND), upper_bound);
    path.push(first_point);

    let mut last_move: Point = Point::new(0., 0.);

    while path[path.len() - 1].y < HEIGHT - lower_bound {
        let mut moves: Vec<Point> = vec![
            Point::new(-20., 20.),
            Point::new(0., 10.),
            Point::new(20., 20.),
        ];
        let current_move: Point;

        if path[path.len() - 1].x < H_BOUND {
            moves[0] = Point::new(0., 10.);
        }
        if path[path.len() - 1].x > WIDTH - H_BOUND {
            moves[2] = Point::new(0., 10.);
        }

        if last_move.x == 0. {
            current_move = moves[_get_random_usize(0, 3)]
        } else if last_move.x < 0. {
            current_move = moves[_get_random_usize(0, 2)];
        } else {
            current_move = moves[_get_random_usize(1, 3)];
        }

        path.push(Point::new(
            path[path.len() - 1].x + current_move.x,
            path[path.len() - 1].y + current_move.y,
        ));
        last_move = current_move;
    }

    return path;
}

fn _get_anchors(path: Vec<Point>, count: usize) -> Vec<Point> {
    let interval = (path.len() as f64 / count as f64).floor() as usize;
    let mut anchors = vec![];

    let mut t = 0;
    while t < (count - 1) * interval + 1 {
        anchors.push(path[t]);
        t += interval;
    }

    anchors.push(*path.last().unwrap());

    return anchors;
}

fn _generate_rough(path: Vec<Point>, anchors: Vec<Point>) -> Spline {
    let mut rough = Spline::new();
    let mut i = 0;

    for (j, anchor) in anchors.iter().enumerate() {
        let width_min = _get_random_float(140., 180.);
        if j == 0 {
            rough.points.push(Point::new(anchor.x, anchor.y - 100.));
        }

        let mut left_most = Point::new(anchor.x - width_min, anchor.y);
        let mut right_most = Point::new(anchor.x + width_min, anchor.y);

        while !(path[i].x == anchor.x && path[i].y == anchor.y) {
            if path[i].x < left_most.x {
                left_most.x = path[i].x
            };

            if path[i].x > right_most.x {
                right_most.x = path[i].x
            };

            i += 1;
        }
        rough.points.insert(0, left_most);
        rough.points.push(right_most);
        if j == anchors.len() - 1 {
            rough.points.push(Point::new(anchor.x, anchor.y + 80.));
        }
    }

    return rough;
}

fn _generate_fairway(anchors: Vec<Point>) -> Spline {
    const WIDTH_MIN: f64 = 50.;
    const WIDTH_MAX: f64 = 70.;
    let mut fairway = Spline::new();

    for i in 0..anchors.len() {
        let width_left = _get_random_float(WIDTH_MIN, WIDTH_MAX);
        let width_right = _get_random_float(WIDTH_MIN, WIDTH_MAX);
        let mut next = i + 1;
        let prev = if i == 0 { i } else { i - 1 };

        if i == anchors.len() - 1 {
            next = i
        };

        let mut vect1 = Vector::new(
            anchors[next].x - anchors[i].x,
            anchors[next].y - anchors[i].y,
        );
        let mut vect2 = Vector::new(
            anchors[i].x - anchors[prev].x,
            anchors[i].y - anchors[prev].y,
        );
        vect1.normalize();
        vect2.normalize();

        let mut bisect = Vector::new(-1. * (vect1.y + vect2.y), vect1.x + vect2.x);
        bisect.normalize();

        fairway.points.insert(
            0,
            Point::new(
                (anchors[i].x - bisect.x * width_left).floor(),
                (anchors[i].y - bisect.y * width_left).floor(),
            ),
        );
        fairway.points.push(Point::new(
            (anchors[i].x + bisect.x * width_right).floor(),
            (anchors[i].y + bisect.y * width_right).floor(),
        ));
    }

    return fairway;
}

fn _outline_from_path(spline: Spline) -> Vec<Point> {
    let mut array = vec![];
    let mut t = 0.;
    while t < spline.points.len() as f64 {
        let pos = spline.get_spline_point(t, true);
        array.push(Point::new(pos.y, pos.x));
        t += 0.2;
    }
    return array;
}

fn _get_random_float(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(min, max);
}

fn _get_random_usize(min: usize, max: usize) -> usize {
    let mut rng = rand::thread_rng();
    return rng.gen_range(min, max);
}
