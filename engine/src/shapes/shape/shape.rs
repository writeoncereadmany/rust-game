use crate::shapes::shape::collision::Collision;
use crate::shapes::shape::projection::{Projection, Projects};
use crate::shapes::shape::{bbox, bbox_circle, circle};
use crate::shapes::vec2d::Vec2d;

#[derive(Clone)]
pub enum Shape {
    Circle(circle::Circle),
    BBox(bbox::BBox),
}

impl Shape {

    pub fn bbox(left: f64, bottom: f64, width: f64, height: f64) -> Self {
        let right = left + width;
        let top = bottom + height;
        Shape::BBox(bbox::BBox { left, right, bottom, top})
    }

    pub fn circle(center: (f64, f64), radius: f64) -> Self {
        Shape::Circle(circle::Circle { center, radius })
    }

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

    pub fn collides(&self, other: &Shape, dv: &(f64, f64)) -> Option<Collision> {
        match (self, other) {
            (Shape::Circle(circle1), Shape::Circle(circle2)) => {
                circle::collides(circle1, circle2, dv)
            }
            (Shape::BBox(bbox1), Shape::BBox(bbox2)) => {
                bbox::collides(bbox1, bbox2, dv)
            }
            (Shape::Circle(circle), Shape::BBox(bbox)) => {
                bbox_circle::collides(bbox, circle, &dv.scale(&-1.0)).map(|col| col.invert())
            }
            (Shape::BBox(bbox), Shape::Circle(circle)) => {
                bbox_circle::collides(bbox, circle, dv)
            }
        }
    }
}

impl Projects for Shape {
    fn project(&self, axis: &(f64, f64)) -> Projection {
        match self {
            Shape::Circle(circle) => { circle.project(axis) }
            Shape::BBox(bbox) => { bbox.project(axis) }
        }
    }
}