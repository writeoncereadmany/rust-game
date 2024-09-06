use super::projection::{collision_on_axis, intersects_on_axis, intersects_on_axis_moving, Projects};
use crate::shapes::shape::bbox::{corners, corners_2, translate, BBox};
use crate::shapes::shape::circle;
use crate::shapes::shape::circle::Circle;
use crate::shapes::shape::collision::Collision;
use crate::shapes::vec2d::{Vec2d, UNIT_X, UNIT_Y};

pub fn intersects(bbox: &BBox, circle: &Circle) -> bool {
    intersects_on_axis(bbox, circle, &UNIT_X) &&
        intersects_on_axis(bbox, circle, &UNIT_Y) && {
        let closest_corner = nearest_corner(&circle.center, &corners(bbox));
        let separation = circle.center.sub(&closest_corner);
        if separation.sq_len() == 0.0 {
            true
        } else {
            let closest_corner_axis = separation.unit();
            intersects_on_axis(bbox, circle, &closest_corner_axis)
        }
    }
}

pub fn intersects_moving(bbox: &BBox, circle: &Circle, dv: &(f64, f64)) -> bool {
    if dv.sq_len() == 0.0 {
        return intersects(bbox, circle);
    }

    intersects_on_axis_moving(bbox, circle, dv, &UNIT_X) &&
        intersects_on_axis_moving(bbox, circle, dv, &UNIT_Y) && {
        let normal_to_travel = dv.unit().perpendicular();
        // no movement normal to the direction of movement, so we can just use intersects,
        // instead of intersects moving.
        intersects_on_axis(bbox, circle, &normal_to_travel)
    } && {
        let corners = corners_2(bbox, &translate(bbox, dv));
        let nearest_corner = nearest_corner(&circle.center, &corners);
        let separation = circle.center.sub(&nearest_corner);
        if separation.sq_len() == 0.0 {
            true
        } else {
            let closest_corner_axis = separation.unit();
            intersects_on_axis_moving(bbox, circle, dv, &closest_corner_axis)
        }
    }
}

pub fn collides(
    bbox: &BBox,
    circle: &Circle,
    dv: &(f64, f64),
) -> Option<Collision> {
    if !intersects_moving(bbox, circle, dv) || intersects(bbox, circle) {
        return None;
    }

    match (collision_on_axis(bbox, circle, dv, &UNIT_X), collision_on_axis(bbox, circle, dv, &UNIT_Y))
    {
        (Some(x_push), Some(y_push)) => if x_push.dt > y_push.dt {
            corner_collision(bbox, circle, dv, x_push)
        } else {
            corner_collision(bbox, circle, dv, y_push)
        },
        (Some(x_push), None) => corner_collision(bbox, circle, dv, x_push),
        (None, Some(y_push)) => corner_collision(bbox, circle, dv, y_push),
        (None, None) => None
    }
}

// at the point where the box/bounding box collision occurs, is the side that hits the circle
// flush to the circle, or below, or above?
// if flush, use that collision
// if not flush, then take the appropriate corner and use that for
// circle/point (ie 0-radius circle) collisions
pub fn corner_collision(
    bbox: &BBox,
    circle: &Circle,
    dv: &(f64, f64),
    side_collision: Collision
) -> Option<Collision> {
    let dv_to_collision = dv.scale(&side_collision.dt);
    let box_at_time_of_collision = translate(bbox, &dv_to_collision);
    if hits_circle_side_flush(circle, &box_at_time_of_collision, side_collision.push) {
        Some(side_collision)
    } else {
        let nearest_corner = nearest_corner(&circle.center, &corners(&box_at_time_of_collision));
        let nearest_corner_start_point = nearest_corner.sub(&dv_to_collision);
        circle::collides(&Circle { center: nearest_corner_start_point, radius: 0.0}, circle, dv)
    }
}

fn hits_circle_side_flush(circle: &Circle, box_at_time_of_collision: &BBox, push: (f64, f64)) -> bool {
    let side_axis = &push.unit().perpendicular();
    let side_projection = box_at_time_of_collision.project(side_axis);
    let center_proj = circle.center.dot(side_axis);
    side_projection.min <= center_proj && center_proj <= side_projection.max
}

pub fn nearest_corner(point: &(f64, f64), candidates: &Vec<(f64, f64)>) -> (f64, f64) {
    let mut nearest_corner = (f64::NAN, f64::NAN);
    let mut closest_sq_dist = f64::INFINITY;
    for candidate in candidates {
        let sq_dist = point.sub(candidate).sq_len();
        if sq_dist < closest_sq_dist {
            nearest_corner = *candidate;
            closest_sq_dist = sq_dist;
        }
    }
    nearest_corner
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shapes::shape::collision::eq_collision;
    use googletest::assert_that;
    use googletest::matchers::{none, some};

    #[test]
    fn find_nearest_corner() {
        assert_eq!(nearest_corner(&(0.0, 0.0), &vec![(0.0, 2.0), (3.0, 0.0), (1.0, 1.0)]), (1.0, 1.0));
    }

    // two interesting cases for static box/circle intersection checking:
    // 1: circle center is more than a radius away from any point, but circle intersects edge
    // 2: projection overlaps both x and y axes, but there is a separating axis between circle and nearest corner
    #[test]
    fn intersects_with_edge() {
        let bbox = BBox { left: 0.0, right: 4.0, top: 4.0, bottom: 2.0 };
        let circle = Circle { center: (2.0, 5.0), radius: 2.0 };
        assert_eq!(intersects(&bbox, &circle), true);
    }

    // here, the bounding box of the circle intersects the bbox, but there's
    // a diagonal separating axis of (1.0, 1.0)
    #[test]
    fn separated_on_corner() {
        let bbox = BBox { left: 4.0, right: 6.0, top: 6.0, bottom: 4.0 };
        let circle = Circle { center: (0.0, 0.0), radius: 5.0 };
        assert_eq!(intersects(&bbox, &circle), false);
    }

    #[test]
    fn intersects_and_moves_away() {
        let bbox = BBox { left: 4.0, right: 6.0, top: 6.0, bottom: 4.0 };
        let circle = Circle { center: (5.0, 5.0), radius: 2.0 };

        assert_eq!(intersects_moving(&bbox, &circle, &(10.0, 10.0)), true)
    }

    #[test]
    fn moves_into_intersection() {
        let bbox = BBox { left: 4.0, right: 6.0, top: 6.0, bottom: 4.0 };
        let circle = Circle { center: (15.0, 15.0), radius: 2.0 };

        assert_eq!(intersects_moving(&bbox, &circle, &(10.0, 10.0)), true)
    }

    #[test]
    fn moves_through_intersection() {
        let bbox = BBox { left: 4.0, right: 6.0, top: 6.0, bottom: 4.0 };
        let circle = Circle { center: (10.0, 10.0), radius: 2.0 };

        assert_eq!(intersects_moving(&bbox, &circle, &(10.0, 10.0)), true)
    }

    #[test]
    fn moves_past_horizontally() {
        let bbox = BBox { left: 4.0, right: 6.0, top: 6.0, bottom: 4.0 };
        let circle = Circle { center: (10.0, 10.0), radius: 2.0 };

        assert_eq!(intersects_moving(&bbox, &circle, &(10.0, 0.0)), false)
    }

    #[test]
    fn moves_past_vertically() {
        let bbox = BBox { left: 4.0, right: 6.0, top: 6.0, bottom: 4.0 };
        let circle = Circle { center: (10.0, 10.0), radius: 2.0 };

        assert_eq!(intersects_moving(&bbox, &circle, &(0.0, 10.0)), false)
    }

    #[test]
    fn moves_past_diagonally() {
        let bbox = BBox { left: 4.0, right: 6.0, top: 6.0, bottom: 4.0 };
        let circle = Circle { center: (15.0, 5.0), radius: 2.0 };

        assert_eq!(intersects_moving(&bbox, &circle, &(10.0, 10.0)), false)
    }

    #[test]
    fn separated_on_initial_corner() {
        let bbox = BBox { left: 4.0, right: 6.0, top: 6.0, bottom: 4.0 };
        let circle = Circle { center: (0.0, 0.0), radius: 5.0 };

        assert_eq!(intersects_moving(&bbox, &circle, &(10.0, 10.0)), false);
    }

    #[test]
    fn separated_on_final_corner() {
        let bbox = BBox { left: 4.0, right: 6.0, top: 16.0, bottom: 14.0 };
        let circle = Circle { center: (0.0, 0.0), radius: 5.0 };

        assert_eq!(intersects_moving(&bbox, &circle, &(0.0, -10.0)), false);
    }

    #[test]
    fn already_intersecting_vertically_hits_side() {
        let bbox = BBox { left: 4.0, right: 6.0, top: 6.0, bottom: 4.0 };
        let circle = Circle { center: (0.0, 5.0), radius: 3.0 };

        assert_that!(
            collides(&bbox, &circle, &(-4.0, 0.0)),
            some(eq_collision(0.25, (3.0, 0.0)))
        )
    }

    #[test]
    fn already_intersecting_horizontally_hits_bottom() {
        let bbox = BBox { left: 4.0, right: 6.0, top: 6.0, bottom: 4.0 };
        let circle = Circle { center: (5.0, 0.0), radius: 2.0 };

        assert_that!(
            collides(&bbox, &circle, &(0.0, -4.0)),
            some(eq_collision(0.5, (0.0, 2.0)))
        )
    }

    #[test]
    fn separated_on_corner_for_collision() {
        let bbox = BBox { left: 6.0, right: 8.0, top: 8.0, bottom: 6.0 };
        let circle = Circle { center: (0.0, 0.0), radius: 5.0 };
        assert_that!(
            collides(&bbox, &circle, &(-2.0, -2.0)),
            none()
        )
    }

    #[test]
    fn hits_on_corner() {
        let bbox = BBox { left: 8.0, right: 12.0, bottom: 6.0, top: 8.0 };
        let circle = Circle { center: (0.0, 0.0), radius: 5.0 };
        assert_that!(
            collides(&bbox, &circle, &(-8.0, -6.0)),
            some(eq_collision(0.5, (4.0, 3.0)))
        )
    }

    #[test]
    fn hits_on_corner_from_the_other_direction() {
        let bbox = BBox { left: -12.0, right: -8.0, bottom: -8.0, top: -6.0 };
        let circle = Circle { center: (0.0, 0.0), radius: 5.0 };
        assert_that!(
            collides(&bbox, &circle, &(8.0, 6.0)),
            some(eq_collision(0.5, (-4.0, -3.0)))
        )
    }

    #[test]
    fn overpasses_and_hits_on_reverse_side() {
        let bbox = BBox { left: 2.0, right: 3.0, top: 22.0, bottom: 16.0 };
        let circle = Circle { center: (0.0, 0.0), radius: 5.0 };
        assert_that!(
            collides(&bbox, &circle, &(-8.0, -16.0)),
            some(eq_collision(0.75, (-1.2, 1.6)))
        )
    }
}