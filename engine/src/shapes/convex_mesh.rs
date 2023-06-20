use std::cmp::Ordering;

use super::bbox::BBox;
use super::push::Push;
use super::vec2d::Vec2d;

const PUSH_EPSILON: f64 = 0.001;

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

pub enum Mesh {
    Convex(ConvexMesh),
    AABB(BBox),
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
        ConvexMesh::new(
            vec![(left, bottom), (left+width, bottom), (left+width, bottom+height), (left, bottom+height)],
            vec![(-1.0, 0.0), (1.0, 0.0), (0.0, -1.0), (0.0, 1.0)]
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

impl Push<Mesh> for Mesh {
    fn push(&self, other: &Mesh, relative_translation: &(f64, f64)) -> Option<(f64, f64)> {
        match (self, other) {
            (Mesh::Convex(this), Mesh::Convex(other)) => this.push(other, relative_translation),
            (Mesh::AABB(this), Mesh::AABB(that)) => this.push(that, relative_translation),
            _ => None
        }
    }
}

impl Push<ConvexMesh> for ConvexMesh {

    fn push(&self, other: &ConvexMesh, relative_travel: &(f64, f64)) -> Option<(f64, f64)> {
        let mut earliest_push: Option<(f64, f64)> = None;

        for normal in normals_as_applied_to_other(self, other) {
            let my_max : f64 = self.points.iter().map(|point| normal.dot(point)).reduce(f64::max)?;
            let their_min : f64 = other.points.iter().map(|point| normal.dot(point)).reduce(f64::min)?;
            if my_max < their_min {
                return None;
            } else {
                let push_distance = my_max - their_min;
                if push_distance + normal.dot(relative_travel) <= PUSH_EPSILON
                {
                    let potential_push = normal.scale(push_distance);
                    earliest_push = earliest_push.map_or(Some(potential_push), |push| Some(earliest(push, potential_push, relative_travel)));
                }
            }
        }
        earliest_push
    }
}

fn normals_as_applied_to_other(first: &ConvexMesh, other: &ConvexMesh) -> Vec<(f64, f64)> {
    first.normals.iter()
        .map(|mine| mine.scale(1.0))
        .chain(other.normals.iter().map(|theirs| theirs.scale(-1.0)))
        .collect()
}

fn earliest(a: (f64, f64), b: (f64, f64), relative_travel: &(f64, f64)) -> (f64, f64) {
    // we want the push with the largest component iro relative_travel, as that's the edge
    // that would be hit first
    let proj_a = a.dot(relative_travel);
    let proj_b = b.dot(relative_travel);
    match proj_a.partial_cmp(&proj_b) {
        Some(Ordering::Greater) => b,
        _otherwise => a
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn pushes_against_motion(){
        let pushes_up_only = ConvexMesh::new(vec![(0.0, 0.0), (10.0, 0.0)], vec![(0.0, 1.0)]);
        let square = ConvexMesh::rect(-1.0, -1.0, 2.0, 2.0);

        assert_eq!(pushes_up_only.push(&square, &(0.0, -5.0)), Some((0.0, 1.0)));
    }

    #[test]
    fn does_not_push_with_motion(){
        let deep_rect = ConvexMesh::rect(0.0, -20.0, 10.0, 20.0);
        let square = ConvexMesh::rect(-1.0, -1.0, 2.0, 2.0);

        assert_eq!(deep_rect.push(&square, &(0.0, 5.0)), None);
    }

    #[test]
    fn does_not_push_more_than_movement() {
        let deep_rect = ConvexMesh::rect(0.0, -20.0, 10.0, 20.0);
        let square = ConvexMesh::rect(-1.0, -1.0, 2.0, 2.0);

        // was already 1 unit to other side of barrier, but has only moved 0.5 this frame: was already through
        // the object, let it continue
        assert_eq!(deep_rect.push(&square, &(0.0, -0.5)), None);
    }

    #[test]
    fn does_not_push_more_than_movement_in_normal_component() {
        let deep_rect = ConvexMesh::rect(-100.0, -20.0, 100.0, 0.0);
        let square = ConvexMesh::rect(-1.0, -1.0, 2.0, 2.0);

        // horizontal speed should not matter here, only vertical
        assert_eq!(deep_rect.push(&square, &(20.0, -0.5)), None);
    }

    #[test]
    fn pushes_against_first_impacted_edge_from_side(){
        let pushes_up_and_left = ConvexMesh::new(vec![(0.0, -10.0), (0.0, 0.0), (10.0, 0.0)], vec![(0.0, 1.0), (-1.0, 0.0)]);
        let square = ConvexMesh::rect(-1.0, -1.0, 2.0, 2.0);

        // intersects nore shallowly into top than left, but hits left first,
        // therefore push should be all left
        assert_eq!(pushes_up_and_left.push(&square, &(10.0, -2.0)), Some((-1.0, 0.0)));
    }

    #[test]
    fn pushes_against_first_impacted_edge_from_top(){
        let pushes_up_and_left = ConvexMesh::new(vec![(0.0, -10.0), (0.0, 0.0), (10.0, 0.0)], vec![(0.0, 1.0), (-1.0, 0.0)]);
        let square = ConvexMesh::rect(-1.0, -1.0, 2.0, 2.0);

        // intersects evenly deeply into left and top sides, but with larger vertical component to approach,
        // therefore push should be all up
        assert_eq!(pushes_up_and_left.push(&square, &(2.0, -10.0)), Some((0.0, 1.0)));
    }
}