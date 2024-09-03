use crate::shapes::shape::collision::Collision;
use crate::shapes::shape::projection::{Projection, Projects};
use crate::shapes::shape::{bbox, bbox_circle, circle};
use crate::shapes::shape::shape::Shape::{ BBox, Circle };
use crate::shapes::vec2d::Vec2d;

#[derive(Clone, Debug)]
pub enum Shape {
    Circle(circle::Circle),
    BBox(bbox::BBox),
}

pub const BLOCK: Shape = BBox(bbox::BBox { left: 0.0, bottom: 0.0, right: 1.0, top: 1.0 });

impl Shape {

    pub fn bbox(left: f64, bottom: f64, width: f64, height: f64) -> Self {
        let right = left + width;
        let top = bottom + height;
        BBox(bbox::BBox { left, right, bottom, top})
    }

    pub fn circle(center: (f64, f64), radius: f64) -> Self {
        Circle(circle::Circle { center, radius })
    }

    pub fn translate(&self, dp: &(f64, f64)) -> Shape {
        match self {
            Circle(circle) => { Circle(circle::translate(circle, dp)) }
            BBox(bbox) => { BBox(bbox::translate(bbox, dp)) }
        }
    }

    pub fn intersects(&self, other: &Shape) -> bool {
        match (self, other) {
            (Circle(circle1), Circle(circle2)) => {
                circle::intersects(circle1, circle2)
            }
            (BBox(bbox1), BBox(bbox2)) => {
                bbox::intersects(bbox1, bbox2)
            }
            (Circle(circle), BBox(bbox)) => {
                bbox_circle::intersects(bbox, circle)
            }
            (BBox(bbox), Circle(circle)) => {
                bbox_circle::intersects(bbox, circle)
            }
        }
    }

    pub fn intersects_moving(&self, other: &Shape, dv: &(f64, f64)) -> bool {
        match (self, other) {
            (Circle(circle1), Circle(circle2)) => {
                circle::intersects_moving(circle1, circle2, dv)
            }
            (BBox(bbox1), BBox(bbox2)) => {
                bbox::intersects_moving(bbox1, bbox2, dv)
            }
            (Circle(circle), BBox(bbox)) => {
                bbox_circle::intersects_moving(bbox, circle, &dv.scale(&-1.0))
            }
            (BBox(bbox), Circle(circle)) => {
                bbox_circle::intersects_moving(bbox, circle, dv)
            }
        }
    }

    pub fn collides(&self, other: &Shape, dv: &(f64, f64)) -> Option<Collision> {
        match (self, other) {
            (Circle(circle1), Circle(circle2)) => {
                circle::collides(circle1, circle2, dv)
            }
            (BBox(bbox1), BBox(bbox2)) => {
                bbox::collides(bbox1, bbox2, dv)
            }
            (Circle(circle), BBox(bbox)) => {
                bbox_circle::collides(bbox, circle, &dv.scale(&-1.0)).map(|col| col.invert())
            }
            (BBox(bbox), Circle(circle)) => {
                bbox_circle::collides(bbox, circle, dv)
            }
        }
    }
}

impl Projects for Shape {
    fn project(&self, axis: &(f64, f64)) -> Projection {
        match self {
            Circle(circle) => { circle.project(axis) }
            BBox(bbox) => { bbox.project(axis) }
        }
    }
}