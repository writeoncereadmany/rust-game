use crate::shapes::vec2d::Vec2d;

#[derive(Debug, PartialEq)]
pub struct Projection {
    pub min: f64,
    pub max: f64
}

pub fn project_line(start: &(f64, f64), end: &(f64, f64), axis: &(f64, f64)) -> Projection {
    let start_proj = start.dot(axis);
    let end_proj = end.dot(axis);
    Projection { min: start_proj.min(end_proj), max: start_proj.max(end_proj) }
}

pub fn intersects(a: &Projection, b: &Projection) -> bool {
    a.max > b.min && a.min < b.max
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