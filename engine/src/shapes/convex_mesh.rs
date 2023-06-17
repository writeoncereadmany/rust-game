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
            vec![]
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

    fn push(&self, other: &ConvexMesh, relative_travel: &(f64, f64)) -> Option<(f64, f64)> {
        self.normals.iter()
            .map(|mine| mine.scale(1.0))
            .chain(other.normals.iter()
                .map(|theirs| theirs.scale(-1.0)))
            .filter(|normal| normal.dot(relative_travel) < 0.0)
            .map(|normal| {
                let my_max : f64 = self.points.iter().map(|point| normal.dot(point)).reduce(f64::max)?;
                let their_min : f64 = other.points.iter().map(|point| normal.dot(point)).reduce(f64::min)?;
                if my_max < their_min {
                    None
                } else {
                    Some(normal.scale(my_max - their_min))
                }
            })
            .flat_map(Option::into_iter)
            .filter(|push| {
                let normalized_push = push.normalize();
                normalized_push.dot(push) + normalized_push.dot(relative_travel) <= PUSH_EPSILON
            })
            .min_by(|a, b| earliest(a, b, relative_travel))
    }
}

fn earliest(a: &(f64, f64), b: &(f64, f64), relative_travel: &(f64, f64)) -> Ordering {
    // we want the push with the largest component iro relative_travel, as that's the edge
    // that would be hit first
    let proj_a = a.dot(relative_travel);
    let proj_b = b.dot(relative_travel);
    match proj_b.partial_cmp(&proj_a) {
        None => Ordering::Equal,
        Some(ord) => ord
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
        let pushes_up_only = ConvexMesh::new(vec![(0.0, 0.0), (10.0, 0.0)], vec![(0.0, 1.0)]);
        let square = ConvexMesh::rect(-1.0, -1.0, 2.0, 2.0);

        assert_eq!(pushes_up_only.push(&square, &(0.0, 5.0)), None);
    }

    #[test]
    fn does_not_push_more_than_movement(){
        let pushes_up_only = ConvexMesh::new(vec![(0.0, 0.0), (10.0, 0.0)], vec![(0.0, 1.0)]);
        let square = ConvexMesh::rect(-1.0, -1.0, 2.0, 2.0);

        // was already 1 unit to other side of barrier, but has only moved 0.5 this frame: was already through
        // the object, let it continue
        assert_eq!(pushes_up_only.push(&square, &(0.0, -0.5)), None);
    }


    #[test]
    fn does_not_push_more_than_movement_in_normal_component(){
        let pushes_up_only = ConvexMesh::new(vec![(0.0, 0.0), (10.0, 0.0)], vec![(0.0, 1.0)]);
        let square = ConvexMesh::rect(-1.0, -1.0, 2.0, 2.0);

        // horizontal speed should not matter here, only vertical
        assert_eq!(pushes_up_only.push(&square, &(20.0, -0.5)), None);
    }
}