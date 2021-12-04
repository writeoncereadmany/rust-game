use std::cmp::{PartialOrd, Ordering};

pub struct Rectangle {
    left: f64,
    right: f64,
    top: f64,
    bottom: f64
}

pub struct ConvexMesh {
    points: Vec<(f64, f64)>,
    normals: Vec<(f64, f64)>
}

impl ConvexMesh {
    fn new(points: Vec<(f64, f64)>) -> Self {
        let normals : Vec<(f64, f64)> = points.iter().map(|&v| v).collect();
        
        ConvexMesh {
            points,
            normals
        }
    }

    fn rect(left: f64, bottom: f64, width: f64, height: f64) -> Self {
        let right = left + width;
        let top = bottom + height;

        ConvexMesh::new(vec![(left, bottom), (left, top), (right, top), (right, bottom)])
    }
}

pub trait VecMath<A> {
    fn dot(self, other: A) -> f64;

    fn normalize(self) -> A;
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
}

// returns the shortest vector that other needs to be moved by to no longer
// be overlapping self, or Option.None if they are already not overlapping
pub trait Push<A> {
    fn push(&self, other: &A) -> Option<(f64, f64)>;
}

// Given two rectangles: 
//
// /----------------\
// | self           |
// |      /---------+--\
// |      | other   |  |
// \------+---------/  |
//        |            |
//        \------------/
//
// There are four possible pushes by which b can be pushed to no longer overlap a:
// 1) Down by 2 (0, -2) (compares a::bottom with b::top)
// 2) Up by 7 (0, 7) (compares a::top with b::bottom)
// 3) Left by 20 (0, -20) (compares a::left with b::right)
// 4) Right by 11 (0, 11) (compares a::right with b::left)
//
// Of those, the shortest is Down, so we return Option::Some((0, -2))
//
// Alternatively, if there is an axis along which there's separation, no push at all is required:
//
//  /-------\
//  | self  |   /-----------\
//  |       |   | other     |
//  \-------/   \-----------/
// 
// here, a::right < b::left, so there's already a separation - regardless of any comparisons on other vectors
// so we return Option::None
impl Push<Rectangle> for Rectangle {
    fn push(&self, other: &Rectangle) -> Option<(f64, f64)> {
        let pushes = vec!(
            axis_push(self.top, other.bottom, (0.0, 1.0)),
            axis_push(other.top, self.bottom, (0.0, -1.0)), // push self applies to other = -push other applies to self
            axis_push(self.right, other.left, (1.0, 0.0)),
            axis_push(other.right, self.left, (-1.0, 0.0)), // push self applies to other = -push other applies to self
        );
        pushes.iter().min_by(|a, b| shorter(a, b)).unwrap_or(&None).clone()
    }
}

impl Rectangle {
    pub fn new(left: f64, bottom: f64, width: f64, height: f64) -> Self {
        Rectangle {
            left,
            right: left + width,
            bottom,
            top: bottom + height
        }
    }
}

fn axis_push(baseline: f64, to_push: f64, axis: (f64, f64)) -> Option<(f64, f64)> {
    if baseline <= to_push {
        None
    } else {
        let distance = baseline - to_push;
        let (x, y) = axis;
        Some((distance*x, distance*y))
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
    fn horizontally_disjoint_rectangles_do_not_collide() {
        let left = Rectangle::new(100.0, 100.0, 100.0, 100.0);
        let right = Rectangle::new(300.0, 100.0, 100.0, 100.0);
        assert_eq!(left.push(&right), None);
    }

    #[test]
    fn vertically_disjoint_rectangles_do_not_collide() {
        let lower = Rectangle::new(100.0, 100.0, 100.0, 100.0);
        let higher = Rectangle::new(100.0, 300.0, 100.0, 100.0);
        assert_eq!(lower.push(&higher), None);
    }


    #[test]
    fn pushes_right_with_slight_overlap() {
        let left = Rectangle::new(100.0, 100.0, 100.0, 100.0);
        let right = Rectangle::new(180.0, 100.0, 100.0, 100.0);
        assert_eq!(left.push(&right), Some((20.0, 0.0)));
    }

    #[test]
    fn pushes_left_with_slight_overlap() {
        let left = Rectangle::new(100.0, 100.0, 100.0, 100.0);
        let right = Rectangle::new(180.0, 100.0, 100.0, 100.0);
        assert_eq!(right.push(&left), Some((-20.0, 0.0)));
    }

    #[test]
    fn pushes_up_with_slight_overlap() {
        let lower = Rectangle::new(100.0, 100.0, 100.0, 100.0);
        let higher = Rectangle::new(100.0, 180.0, 100.0, 100.0);
        assert_eq!(lower.push(&higher), Some((0.0, 20.0)));
    }


    #[test]
    fn pushes_down_with_slight_overlap() {
        let lower = Rectangle::new(100.0, 100.0, 100.0, 100.0);
        let higher = Rectangle::new(100.0, 180.0, 100.0, 100.0);
        assert_eq!(higher.push(&lower), Some((0.0, -20.0)));
    }
}