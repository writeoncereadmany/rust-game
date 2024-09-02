use crate::shapes::vec2d::Vec2d;

#[derive(Debug, PartialEq)]
pub struct Projection {
    pub min: f64,
    pub max: f64
}

pub trait Projects {
    fn project(&self, axis: &(f64, f64)) -> Projection;

    fn project_moving(&self, dv: &(f64, f64), axis: &(f64, f64)) -> Projection {
        let delta = dv.dot(axis);
        let Projection { min, max } = self.project(axis);
        Projection { min: min + delta.min(0.0), max: max + delta.max(0.0) }
    }
}

impl Projects for (f64, f64) {
    fn project(&self, axis: &(f64, f64)) -> Projection {
        Projection { min: self.dot(axis), max: self.dot(axis) }
    }
}

pub fn intersects_on_axis<A: Projects, B: Projects>(a: &A, b: &B, axis: &(f64, f64)) -> bool {
    intersects(&a.project(axis), &b.project(axis))
}

pub fn intersects_on_axis_moving<A: Projects, B: Projects>(
    moving: &A, stationary: &B, dv: &(f64, f64), axis: &(f64, f64)) -> bool
{
    intersects(&moving.project_moving(dv, axis), &stationary.project(axis))
}

pub fn intersects(a: &Projection, b: &Projection) -> bool {
    a.max > b.min && a.min < b.max
}

// determines pushes b would apply to a such that a is no longer intersecting with b
// a:    |--------------|
// b:            |---------|
// push left is b.min - a.max (always negative), push right is b.max - a.min (always positive)
// if b.min - a.max is positive or b.max - a.min is negative, then the shapes don't intersect
pub fn pushes(a: &Projection, b: &Projection) -> Option<(f64, f64)> {
    if intersects(a, b) {
        Some((b.min - a.max, b.max - a.min))
    } else {
        None
    }
}

mod tests {
    use super::*;

    #[test]
    fn projection_to_left_does_not_intersect() {
        let a = Projection { min: 1.0, max: 2.0 };
        let b = Projection { min: 3.0, max: 4.0 };
        assert_eq!(intersects(&a, &b), false);
    }

    #[test]
    fn projection_to_right_does_not_intersect() {
        let a = Projection { min: 1.0, max: 2.0 };
        let b = Projection { min: 3.0, max: 4.0 };
        assert_eq!(intersects(&b, &a), false);
    }

    #[test]
    fn projection_enclosing_intersects() {
        let a = Projection { min: 1.0, max: 4.0 };
        let b = Projection { min: 2.0, max: 3.0 };
        assert_eq!(intersects(&a, &b), true);
    }

    #[test]
    fn projection_enclosed_intersects() {
        let a = Projection { min: 1.0, max: 4.0 };
        let b = Projection { min: 2.0, max: 3.0 };
        assert_eq!(intersects(&b, &a), true);
    }

    #[test]
    fn projection_overlaps_max_intersects() {
        let a = Projection { min: 1.0, max: 3.0 };
        let b = Projection { min: 2.0, max: 4.0 };
        assert_eq!(intersects(&a, &b), true);
    }

    #[test]
    fn projection_overlaps_min_intersects() {
        let a = Projection { min: 1.0, max: 3.0 };
        let b = Projection { min: 2.0, max: 4.0 };
        assert_eq!(intersects(&b, &a), true);
    }
}