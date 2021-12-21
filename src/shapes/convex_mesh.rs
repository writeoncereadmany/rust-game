use std::cmp::Ordering;

use super::bbox::BBox;
use super::push::Push;
use super::vec2d::Vec2d;

#[derive(Clone)]
pub struct ConvexMesh {
    bbox: BBox,
    points: Vec<(f64, f64)>,
    normals: Vec<(f64, f64)>
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

    pub fn rect(left: f64, bottom: f64, width: f64, height: f64) -> Self {
        let right = left + width;
        let top = bottom + height;

        ConvexMesh::new( 
            vec![(left, bottom), (left, top), (right, top), (right, bottom)],
            vec![(-1.0, 0.0), (0.0, 1.0), (1.0, 0.0), (0.0, -1.0)]
        )
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn horizontally_disjoint_convex_mesh_rectangles_do_not_collide() {
        let left = ConvexMesh::rect(100.0, 100.0, 100.0, 100.0);
        let right = ConvexMesh::rect(300.0, 100.0, 100.0, 100.0);
        assert_eq!(left.push(&right), None);
    }

    #[test]
    fn pushes_convex_mesh_rect_right_with_slight_overlap() {
        let left = ConvexMesh::rect(100.0, 100.0, 100.0, 100.0);
        let right = ConvexMesh::rect(180.0, 100.0, 100.0, 100.0);
        assert_eq!(left.push(&right), Some((20.0, 0.0)));
    }
}