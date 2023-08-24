use super::vec2d::{ UNIT_X, UNIT_Y, Vec2d };

const AXES: [(f64, f64); 2] = [UNIT_X, UNIT_Y];

struct BBox {
    left: f64,
    right: f64,
    bottom: f64,
    top: f64
}

struct Circle {
    center: (f64, f64),
    radius: f64
}

struct Convex {
    points: Vec<(f64, f64)>
}

enum Shape {
    BBox(BBox),
    Circle(Circle),
    Convex(Convex)
}

impl Shape {
    fn bbox((left, bottom): (f64, f64), width: f64, height: f64) -> Shape {
        Shape::BBox(BBox { left, right: left + width, bottom, top: bottom + height })
    }

    fn circle(center: (f64, f64), radius: f64) -> Shape {
        Shape::Circle(Circle { center, radius })
    }

    fn convex(points: Vec<(f64, f64)>) -> Shape {
        Shape::Convex(Convex { points })
    }
}

#[derive(Debug, PartialEq)]
struct Projection { min: f64, max: f64}

impl Projection {
    fn overlaps(&self, other: &Projection) -> bool {
        !(self.min > other.max || self.max < other.min)
    }
}

trait Project {
    fn project(&self, axis: &(f64, f64)) -> Projection;
}

impl Project for BBox {
    fn project(&self, axis@(ax, ay): &(f64, f64)) -> Projection {
        let (min_x, max_x) = if ax > &0.0 { (self.left, self.right) } else { (self.right, self.left) };
        let (min_y, max_y) = if ay > &0.0 { (self.bottom, self.top) } else { (self.top, self.bottom) };
        Projection { min: axis.dot(&(min_x, min_y)), max: axis.dot(&(max_x, max_y)) }
    }
}

impl Project for Circle {
    fn project(&self, axis: &(f64, f64)) -> Projection {
        let projected_center = axis.dot(&self.center);
        Projection { min: projected_center - self.radius, max: projected_center + self.radius }
    }
}

impl Project for Convex {
    fn project(&self, axis: &(f64, f64)) -> Projection {
        let (mut min, mut max) = (f64::NAN, f64::NAN);
        for point in &self.points {
            let projection = axis.dot(&point);
            (min, max) = (min.min(projection), max.max(projection));
        }
        Projection { min, max }
    }
}

impl Project for Shape {
    fn project(&self, axis: &(f64, f64)) -> Projection {
        match self {
            Shape::BBox(bbox)     => bbox.project(axis),
            Shape::Circle(circle) => circle.project(axis),
            Shape::Convex(convex) => convex.project(axis)
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::shapes::vec2d::{ UNIT_X, UNIT_Y, Vec2d };

    #[test]
    fn test_bbox_projections()
    {
        let bbox = Shape::bbox((1.0, 2.0), 2.0, 3.0);
        assert_eq!(bbox.project(&UNIT_X), Projection { min: 1.0, max: 3.0 });
        assert_eq!(bbox.project(&UNIT_Y), Projection { min: 2.0, max: 5.0 });
        assert_eq!(bbox.project(&UNIT_X.scale(-1.0)), Projection { min: -3.0, max: -1.0 });
        assert_eq!(bbox.project(&UNIT_Y.scale(-1.0)), Projection { min: -5.0, max: -2.0 });
    }

    #[test]
    fn test_circle_projections()
    {
        let circle = Shape::circle((4.0, 3.0), 2.0);
        assert_eq!(circle.project(&UNIT_X), Projection { min: 2.0, max: 6.0 });
        assert_eq!(circle.project(&UNIT_Y), Projection { min: 1.0, max: 5.0 });
        assert_eq!(circle.project(&(0.8, 0.6)), Projection { min: 3.0, max: 7.0});
    }

    #[test]
    fn test_convex_projections()
    {
        let points = vec![(4.0, 3.0), (8.0, 6.0), (7.0, 2.0)];
        let convex = Shape::convex(points);
        assert_eq!(convex.project(&UNIT_X), Projection { min: 4.0, max: 8.0});
        assert_eq!(convex.project(&UNIT_Y), Projection { min: 2.0, max: 6.0});
        assert_eq!(convex.project(&(0.8, 0.6)), Projection { min: 5.0, max: 10.0 });
    }

    #[test]
    fn no_overlap_when_other_is_right()
    {
        assert_eq!(Projection { min: 5.0, max: 6.0}.overlaps(&Projection { min: 10.0, max: 12.0}), false);
    }

    #[test]
    fn no_overlap_when_other_is_left()
    {
        assert_eq!(Projection { min: 5.0, max: 6.0}.overlaps(&Projection { min: 1.0, max: 2.0}), false);
    }

    #[test]
    fn overlap_when_other_entirely_contained()
    {
        assert_eq!(Projection { min: 1.0, max: 6.0}.overlaps(&Projection { min: 3.0, max: 4.0}), true);
    }

    #[test]
    fn overlap_when_other_entirely_contains()
    {
        assert_eq!(Projection { min: 3.0, max: 4.0}.overlaps(&Projection { min: 1.0, max: 6.0}), true);
    }


    #[test]
    fn overlap_left_end()
    {
        assert_eq!(Projection { min: 2.0, max: 4.0}.overlaps(&Projection { min: 1.0, max: 3.0}), true);
    }

    #[test]
    fn overlap_right_end()
    {
        assert_eq!(Projection { min: 3.0, max: 6.0}.overlaps(&Projection { min: 1.0, max: 3.0}), true);
    }
}