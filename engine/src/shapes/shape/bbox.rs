use crate::shapes::shape::collision::Collision;
use crate::shapes::shape::projection;
use crate::shapes::vec2d::{UNIT_X, UNIT_Y, Vec2d};

use super::projection::{Projection, Projects, pushes};

pub struct BBox {
    pub left: f64,
    pub right: f64,
    pub bottom: f64,
    pub top: f64
}

impl Projects for BBox {
    fn project(&self, axis@(x, y): &(f64, f64)) -> Projection {
        let &BBox { left: l, right: r, bottom: b, top: t} = self;
        let (first_x, second_x) = if x < &0.0 { (r, l) } else { (l, r) };
        let (first_y, second_y) = if y < &0.0 { (t, b) } else { (b, t) };
        Projection { min: (first_x, first_y).dot(axis), max: (second_x, second_y).dot(axis) }
    }
}

pub fn intersects(bbox1: &BBox, bbox2: &BBox) -> bool {
    let intersects_horizontally = projection::intersects(&bbox1.project(&UNIT_X), &bbox2.project(&UNIT_X));
    let intersects_vertically = projection::intersects(&bbox1.project(&UNIT_Y), &bbox2.project(&UNIT_Y));
    intersects_horizontally && intersects_vertically
}

pub fn collides(
    bbox1: &BBox,
    bbox2: &BBox,
    dv: &(f64, f64)
) -> Option<Collision> {
    match (axis_push(bbox1, bbox2, dv, &UNIT_X), axis_push(bbox1, bbox2, dv, &UNIT_Y))
    {
        (Some(x_pushes), Some(y_pushes)) => None,
        (_, _) => None
    }
}

fn axis_push(bbox1: &BBox, bbox2: &BBox, dv: &(f64, f64), axis: &(f64, f64)
) -> Option<Vec<(f64, f64)>> {
    let proj_1 = bbox1.project_moving(dv, axis);
    let proj_2 = bbox2.project(axis);
    let pushes = pushes(&proj_1, &proj_2)?;
    Some(pushes
        .iter()
        .map(|push| axis.scale(push))
        .collect())
}


#[cfg(test)]
mod tests {
    use super::*;

// projection tests

    #[test]
    fn project_x_axis() {
        let bbox = BBox { left: 3.0, right: 5.0, top: 4.0, bottom: 2.0 };
        assert_eq!(bbox.project(&(1.0, 0.0)), Projection { min: 3.0, max: 5.0 });
    }

    #[test]
    fn project_y_axis() {
        let bbox = BBox { left: 3.0, right: 5.0, top: 4.0, bottom: 2.0 };
        assert_eq!(bbox.project(&(0.0, 1.0)), Projection { min: 2.0, max: 4.0 });
    }

    #[test]
    fn project_diagonal_axis() {
        let bbox = BBox { left: 3.0, right: 6.0, top: 8.0, bottom: 4.0 };
        assert_eq!(bbox.project(&(0.6, 0.8)), Projection { min: 5.0, max: 10.0 });
    }

    // intersection tests

    #[test]
    fn enclosing_intersects() {
        let bbox1 = BBox { left: 1.0, right: 7.0, bottom: 3.0, top: 9.0 };
        let bbox2 = BBox { left: 3.0, right: 4.0, bottom: 4.0, top: 7.0 };

        assert_eq!(intersects(&bbox1, &bbox2), true);
        assert_eq!(intersects(&bbox2, &bbox1), true);
    }

    #[test]
    fn overlapping_intersects() {
        let bbox1 = BBox { left: 1.0, right: 5.0, bottom: 3.0, top: 6.0 };
        let bbox2 = BBox { left: 3.0, right: 8.0, bottom: 4.0, top: 7.0 };

        assert_eq!(intersects(&bbox1, &bbox2), true);
        assert_eq!(intersects(&bbox2, &bbox1), true);
    }

    #[test]
    fn horizontally_separated() {
        let bbox1 = BBox { left: 1.0, right: 3.0, bottom: 2.0, top: 5.0 };
        let bbox2 = BBox { left: 4.0, right: 6.0, bottom: 3.0, top: 4.0 };
        assert_eq!(intersects(&bbox1, &bbox2), false);
        assert_eq!(intersects(&bbox2, &bbox1), false);
    }

    #[test]
    fn vertically_separated() {
        let bbox1 = BBox { left: 1.0, right: 4.0, bottom: 4.0, top: 5.0 };
        let bbox2 = BBox { left: 3.0, right: 6.0, bottom: 2.0, top: 3.0 };
        assert_eq!(intersects(&bbox1, &bbox2), false);
        assert_eq!(intersects(&bbox2, &bbox1), false);
    }

    // #[test]
    // fn horizontal_collision_from_left() {
    //     let bbox1 = BBox { left: 1.0, right: 4.0, bottom: 0.0, top: 3.0 };
    //     let bbox2 = BBox { left: 6.0, right: 8.0, bottom: 0.0, top: 3.0 };
    //     assert_that!(
    //         collides(&bbox1, &bbox2, &(4.0, 0.0)),
    //         some(eq_collision(0.5, (-2.0, 0.0))));
    // }
}