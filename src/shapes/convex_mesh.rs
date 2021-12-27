use std::cmp::Ordering;

use super::bbox::BBox;
use super::push::Push;
use super::vec2d::Vec2d;
use crate::graphics::sprite::{ Sprite, Sprited };

#[derive(Clone)]
pub struct ConvexMesh {
    bbox: BBox,
    points: Vec<(f64, f64)>,
    pub normals: Vec<(f64, f64)>
}

#[derive(Clone)]
pub struct Meshed<A>
where A: Clone {
    pub item: A,
    pub mesh: ConvexMesh
}

impl <'a, A> Sprited<'a> for Meshed<A> where A: Clone + Sprited<'a> {

    fn sprite(&self) -> &Sprite<'a> {
        self.item.sprite()
    }
}

impl ConvexMesh {
    pub fn new(points: Vec<(f64, f64)>, normals: Vec<(f64, f64)>) -> Self {
        let left = points.iter().map(|&(x, _y)| x).reduce(f64::min).unwrap();
        let right = points.iter().map(|&(x, _y)| x).reduce(f64::max).unwrap();
        let bottom = points.iter().map(|&(_x, y)| y).reduce(f64::min).unwrap();
        let top = points.iter().map(|&(_x, y)| y).reduce(f64::max).unwrap();
        ConvexMesh {
            bbox: BBox::from(left, bottom).to(right, top),
            points,
            normals
        }
    }

    pub fn translate(&self, dx: f64, dy: f64) -> ConvexMesh {
        ConvexMesh {
            bbox: self.bbox.translate(dx, dy),
            points: self.points.iter().map(|(x, y)| (x + dx, y + dy)).collect(),
            normals: self.normals.clone()
        }
    }

    pub fn bbox(&self) -> BBox {
        self.bbox
    }
}

impl Push<ConvexMesh> for ConvexMesh {
    fn push(&self, other: &ConvexMesh) -> Option<(f64, f64)> {
        self.normals.iter()
            .map(|x| x.scale(1.0))
            .chain(other.normals.iter()
                .map(|x| x.scale(-1.0)))
            .map(|normal| {
                let my_max : f64 = self.points.iter().map(|&point| normal.dot(point)).reduce(f64::max)?;
                let their_min : f64 = other.points.iter().map(|&point| normal.dot(point)).reduce(f64::min)?;
                if my_max < their_min {
                    None
                } else {
                    Some(normal.scale(my_max - their_min))
                }
        })
        .min_by(shorter)
        .unwrap_or(None)
    }
}

fn shorter(a: &Option<(f64, f64)>, b: &Option<(f64, f64)>) -> Ordering {
    match (a, b) {
        (None, None) => Ordering::Equal,
        (None, _) => Ordering::Less,
        (_, None) => Ordering::Greater,
        (Some((ax, ay)), Some((bx, by))) => {
            // we want the push with the shortest length, regardless of direction
            // length is /(a^2 + b^2) but we can skip the expensive square root as
            // we don't 
            let lensq_a = ax*ax + ay*ay;
            let lensq_b = bx*bx + by*by;
            match lensq_a.partial_cmp(&lensq_b) {
                None => Ordering::Equal,
                Some(ord) => ord
            }
        }
    }
}