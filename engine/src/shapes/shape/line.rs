use crate::shapes::shape::projection::{Projection, Projects};
use crate::shapes::vec2d::Vec2d;

pub struct Line {
    start: (f64, f64),
    end: (f64, f64)
}

impl Line {
    pub fn new(start: (f64, f64), end: (f64, f64)) -> Self {
        Line { start, end }
    }
}

impl Projects for Line {
    fn project(&self, axis: &(f64, f64)) -> Projection {
        let start_proj = self.start.dot(axis);
        let end_proj = self.end.dot(axis);
        Projection { min: start_proj.min(end_proj), max: start_proj.max(end_proj) }
    }
}