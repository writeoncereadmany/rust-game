use std::cmp::{PartialOrd, Ordering};

#[derive(Clone)]
pub struct ConvexMesh {
    aabb_left: f64,
    aabb_right: f64,
    aabb_top: f64,
    aabb_bottom: f64,
    points: Vec<(f64, f64)>,
    normals: Vec<(f64, f64)>
}

impl ConvexMesh {
    pub fn new(points: Vec<(f64, f64)>, normals: Vec<(f64, f64)>) -> Self {
        ConvexMesh {
            aabb_left: points.iter().map(|&(x, _y)| x).reduce(f64::min).unwrap(),
            aabb_right: points.iter().map(|&(x, _y)| x).reduce(f64::max).unwrap(),
            aabb_top: points.iter().map(|&(_x, y)| y).reduce(f64::min).unwrap(),
            aabb_bottom: points.iter().map(|&(_x, y)| y).reduce(f64::max).unwrap(),
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

    pub fn aabbs_overlap(&self, other: &ConvexMesh) -> bool {
        self.aabb_left < other.aabb_right &&
        self.aabb_right > other.aabb_left &&
        self.aabb_top < other.aabb_bottom &&
        self.aabb_bottom > other.aabb_top
    }
}

pub trait VecMath<A> {
    fn dot(self, other: A) -> f64;

    fn normalize(self) -> A;

    fn scale(self, other: f64) -> A;

    fn sq_len(self) -> f64;
}

impl VecMath<(f64, f64)> for (f64, f64) {

    fn dot(self, other: (f64, f64)) -> f64 {
        let (ax, ay) = self;
        let (bx, by) = other;
        (ax * bx) + (ay * by)
    }

    fn normalize(self) -> (f64, f64) {
        let (x, y) = self;
        let length = (x*x + y*y).sqrt();
        (x / length, y / length)
    }

    fn scale(self, scale: f64) -> (f64, f64) {
        let (x, y) = self;
        (x * scale, y * scale)
    }

    fn sq_len(self) -> f64 {
        let (x, y) = self;
        x*x + y*y
    }
}

// returns the shortest vector that other needs to be moved by to no longer
// be overlapping self, or Option.None if they are already not overlapping
pub trait Push<A> {
    fn push(&self, other: &A) -> Option<(f64, f64)>;
}

impl Push<ConvexMesh> for ConvexMesh {
    fn push(&self, other: &ConvexMesh) -> Option<(f64, f64)> {
        self.normals.iter()
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