use crate::point::Point;

pub struct Spline {
    pub points: Vec<Point>,
}

impl Spline {
    pub fn new() -> Spline {
        Spline { points: vec![] }
    }

    pub fn get_spline_point(&self, t: f64, b_looped: bool) -> Point {
        let p0: usize;
        let p1: usize;
        let p2: usize;
        let p3: usize;

        if !b_looped {
            p1 = t.floor() as usize + 1;
            p2 = p1 + 1;
            p3 = p2 + 1;
            p0 = p1 - 1;
        } else {
            p1 = t.floor() as usize;
            p2 = (p1 + 1) % self.points.len();
            p3 = (p2 + 1) % self.points.len();
            p0 = if p1 >= 1 {
                p1 - 1
            } else {
                self.points.len() - 1
            };
        }

        let next_t = t - t.floor();

        let tt = next_t * next_t;
        let ttt = tt * next_t;

        let q1 = -ttt + 2.0 * tt - next_t;
        let q2 = 3.0 * ttt - 5.0 * tt + 2.0;
        let q3 = -3.0 * ttt + 4.0 * tt + next_t;
        let q4 = ttt - tt;

        let tx = 0.5
            * (self.points[p0].x * q1
                + self.points[p1].x * q2
                + self.points[p2].x * q3
                + self.points[p3].x * q4);
        let ty = 0.5
            * (self.points[p0].y * q1
                + self.points[p1].y * q2
                + self.points[p2].y * q3
                + self.points[p3].y * q4);

        return Point::new(tx.floor(), ty.floor());
    }
}
