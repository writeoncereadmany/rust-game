pub trait Vec2d<A> {
    fn dot(self, other: A) -> f64;

    fn normalize(self) -> A;

    fn scale(self, other: f64) -> A;

    fn sq_len(self) -> f64;
}

impl Vec2d<(f64, f64)> for (f64, f64) {

    fn dot(self, other: (f64, f64)) -> f64 {
        let (ax, ay) = self;
        let (bx, by) = other;
        (ax * bx) + (ay * by)
    }

    fn normalize(self) -> (f64, f64) {
        let (x, y) = self;
        let length = (x*x + y*y).sqrt();
        (x / length, y / length)
    }

    fn scale(self, scale: f64) -> (f64, f64) {
        let (x, y) = self;
        (x * scale, y * scale)
    }

    fn sq_len(self) -> f64 {
        let (x, y) = self;
        x*x + y*y
    }
}