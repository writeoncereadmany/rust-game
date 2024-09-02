use crate::shapes::shape::{bbox, bbox_circle, circle};
use crate::shapes::vec2d::Vec2d;

enum Shape {
    Circle(circle::Circle),
    BBox(bbox::BBox),
}

impl Shape {
    pub fn translate(&self, dp: &(f64, f64)) -> Shape {
        match self {
            Shape::Circle(circle) => { Shape::Circle(circle::translate(circle, dp)) }
            Shape::BBox(bbox) => { Shape::BBox(bbox::translate(bbox, dp)) }
        }
    }

    pub fn intersects(&self, other: &Shape) -> bool {
        match (self, other) {
            (Shape::Circle(circle1), Shape::Circle(circle2)) => {
                circle::intersects(circle1, circle2)
            }
            (Shape::BBox(bbox1), Shape::BBox(bbox2)) => {
                bbox::intersects(bbox1, bbox2)
            }
            (Shape::Circle(circle), Shape::BBox(bbox)) => {
                bbox_circle::intersects(bbox, circle)
            }
            (Shape::BBox(bbox), Shape::Circle(circle)) => {
                bbox_circle::intersects(bbox, circle)
            }
        }
    }

    pub fn intersects_moving(&self, other: &Shape, dv: &(f64, f64)) -> bool {
        match (self, other) {
            (Shape::Circle(circle1), Shape::Circle(circle2)) => {
                circle::intersects_moving(circle1, circle2, dv)
            }
            (Shape::BBox(bbox1), Shape::BBox(bbox2)) => {
                bbox::intersects_moving(bbox1, bbox2, dv)
            }
            (Shape::Circle(circle), Shape::BBox(bbox)) => {
                bbox_circle::intersects_moving(bbox, circle, &dv.scale(&-1.0))
            }
            (Shape::BBox(bbox), Shape::Circle(circle)) => {
                bbox_circle::intersects_moving(bbox, circle, dv)
            }
        }
    }
}