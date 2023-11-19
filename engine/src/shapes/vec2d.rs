pub trait Vec2d<A> {
    fn dot(&self, other: &A) -> f64;

    fn unit(&self) -> A;

    fn scale(&self, other: &f64) -> A;

    fn len(&self) -> f64;

    fn sq_len(&self) -> f64;

    fn perpendicular(&self) -> A;

    fn plus(&self, other: &A) -> A;

    fn sub(&self, other: &A) -> A;
}

pub const ZERO: (f64, f64) = (0.0, 0.0);
pub const UNIT_X: (f64, f64) = (1.0, 0.0);
pub const UNIT_Y: (f64, f64) = (0.0, 1.0);

impl Vec2d<(f64, f64)> for (f64, f64) {

    fn dot(&self, other: &(f64, f64)) -> f64 {
        let (ax, ay) = self;
        let (bx, by) = other;
        (ax * bx) + (ay * by)
    }

    fn unit(&self) -> (f64, f64) {
        self.scale(&(1.0 / self.len()))
    }

    fn scale(&self, scale: &f64) -> (f64, f64) {
        let (x, y) = self;
        (x * scale, y * scale)
    }

    fn len(&self) -> f64 {
        let (x, y) = self;
        (x*x + y*y).sqrt()
    }

    fn sq_len(&self) -> f64 {
        let (x, y) = self;
        x*x + y*y
    }

    fn perpendicular(&self) -> (f64, f64) {
        let &(x, y) = self;
        (y, -x)
    }

    fn plus(&self, (ox, oy): &(f64, f64)) -> (f64, f64) {
        (self.0 + ox, self.1 + oy)
    }

    fn sub(&self, (ox, oy): &(f64, f64)) -> (f64, f64) {
        (self.0 - ox, self.1 - oy)
    }
}