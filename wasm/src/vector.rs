pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64) -> Vector {
        Vector { x: x, y: y }
    }

    pub fn normalize(&mut self) {
        let length = (self.x * self.x + self.y * self.y).sqrt();
        if length > 0. {
            self.x /= length;
            self.y /= length;
        }
    }
}
