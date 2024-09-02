use crate::shapes::shape::collision::Collision;
use crate::shapes::shape::projection;
use crate::shapes::vec2d::{UNIT_X, UNIT_Y, Vec2d};

use super::projection::{Projection, Projects, pushes};

pub struct BBox {
    pub left: f64,
    pub right: f64,
    pub bottom: f64,
    pub top: f64,
}

impl Projects for BBox {
    fn project(&self, axis @ (x, y): &(f64, f64)) -> Projection {
        let &BBox { left: l, right: r, bottom: b, top: t } = self;
        let (first_x, second_x) = if x < &0.0 { (r, l) } else { (l, r) };
        let (first_y, second_y) = if y < &0.0 { (t, b) } else { (b, t) };
        Projection { min: (first_x, first_y).dot(axis), max: (second_x, second_y).dot(axis) }
    }
}

impl BBox {
    pub fn translate(&self, (dx, dy): &(f64, f64)) -> BBox {
        BBox {
            left: self.left + dx,
            right: self.right + dx,
            bottom: self.bottom + dy,
            top: self.top + dy,
        }
    }
}

pub fn intersects(bbox1: &BBox, bbox2: &BBox) -> bool {
    intersects_moving(bbox1, bbox2, &(0.0, 0.0))
}

pub fn intersects_moving(bbox1: &BBox, bbox2: &BBox, dv: &(f64, f64)) -> bool {
    projection::intersects(&bbox1.project_moving(dv, &UNIT_X), &bbox2.project(&UNIT_X)) &&
        projection::intersects(&bbox1.project_moving(dv, &UNIT_Y), &bbox2.project(&UNIT_Y)) && {
        if dv.sq_len() > 0.0 {
            let normal_dv = dv.perpendicular().unit();
            projection::intersects(&bbox1.project_moving(dv, &normal_dv), &bbox2.project(&normal_dv))
        } else {
            true
        }
    }
}

pub fn collides(
    bbox1: &BBox,
    bbox2: &BBox,
    dv: &(f64, f64),
) -> Option<Collision> {
    if !intersects_moving(bbox1, bbox2, dv) {
        return None;
    }

    match (collision_on_axis(bbox1, bbox2, dv, &UNIT_X), collision_on_axis(bbox1, bbox2, dv, &UNIT_Y))
    {
        (Some(x_push), Some(y_push)) => Some(if x_push.dt > y_push.dt { x_push } else { y_push }),
        (Some(x_push), None) => Some(x_push),
        (None, Some(y_push)) => Some(y_push),
        (None, None) => None
    }
}

/*
 * Assuming that two boxes do pass through each other, at what point did they collide?
 * This returns the point during the motion where the two boxes first collide, including
 * both the fraction of motion required in order for them to collide and the vector required
 * to ensure they no longer collide. Note that if the boxes were already intersecting, then
 * no collision is reported: the boxes have not collided this frame.
 */
fn collision_on_axis(bbox1: &BBox, bbox2: &BBox, dv: &(f64, f64), axis: &(f64, f64),
) -> Option<Collision> {
    let proj_1 = bbox1.project_moving(dv, axis);
    let proj_2 = bbox2.project(axis);
    let proj_dv = -dv.dot(axis);
    let (left, right) = pushes(&proj_1, &proj_2)?;
    let (dt_left, dt_right) = (1.0 - left / proj_dv, 1.0 - right / proj_dv);

    if dt_left > 0.0 && dt_left <= 1.0 {
        Some(Collision { dt: dt_left, push: axis.scale(&left) })
    } else if dt_right > 0.0 && dt_right <= 1.0 {
        Some(Collision { dt: dt_right, push: axis.scale(&right) })
    } else {
        None
    }
}

pub fn corners_2(
    &BBox { left: l1, right: r1, top: t1, bottom: b1 }: &BBox,
    &BBox { left: l2, right: r2, top: t2, bottom: b2 }: &BBox) -> Vec<(f64, f64)>
{
    vec![(l1, t1), (r1, t1), (l1, b1), (r1, b1), (l2, t2), (r2, t2), (l2, b2), (r2, b2)]
}

#[cfg(test)]
mod tests {
    use googletest::assert_that;
    use googletest::matchers::{none, some};
    use crate::shapes::shape::collision::eq_collision;
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

    #[test]
    fn horizontal_collision_from_left() {
        let bbox1 = BBox { left: 1.0, right: 4.0, bottom: 0.0, top: 3.0 };
        let bbox2 = BBox { left: 6.0, right: 8.0, bottom: 0.0, top: 3.0 };
        assert_that!(
            collides(&bbox1, &bbox2, &(4.0, 0.0)),
            some(eq_collision(0.5, (-2.0, 0.0))));
    }

    #[test]
    fn horizontal_collision_from_left_when_completely_passes() {
        let bbox1 = BBox { left: 1.0, right: 4.0, bottom: 0.0, top: 3.0 };
        let bbox2 = BBox { left: 6.0, right: 8.0, bottom: 0.0, top: 3.0 };
        assert_that!(
            collides(&bbox1, &bbox2, &(10.0, 0.0)),
            some(eq_collision(0.2, (-8.0, 0.0))));
    }

    #[test]
    fn no_horizontal_collision_when_already_intersecting() {
        let bbox1 = BBox { left: 1.0, right: 4.0, bottom: 0.0, top: 3.0 };
        let bbox2 = BBox { left: 3.0, right: 7.0, bottom: 0.0, top: 3.0 };
        assert_that!(
            collides(&bbox1, &bbox2, &(10.0, 0.0)),
            none());
    }

    // intersects x-axis at dt 0.5, y-axis at 0.75. so not overlapping when intersecting x,
    // so collision is when intersecting y
    #[test]
    fn collision_reported_on_axis_which_intersected_latest() {
        let bbox1 = BBox { left: 1.0, right: 4.0, bottom: 0.0, top: 2.0 };
        let bbox2 = BBox { left: 6.0, right: 7.0, bottom: 5.0, top: 6.0 };
        assert_that!(
            collides(&bbox1, &bbox2, &(4.0, 4.0)),
            some(eq_collision(0.75, (0.0, -1.0))));
    }
}