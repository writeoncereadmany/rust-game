use super::projection::{intersects_on_axis, intersects_on_axis_moving, Projects};
use crate::shapes::shape::bbox::{corners, corners_2, translate, BBox};
use crate::shapes::shape::circle::Circle;
use crate::shapes::shape::collision::Collision;
use crate::shapes::vec2d::{Vec2d, UNIT_X, UNIT_Y};

pub fn intersects(bbox: &BBox, circle @ Circle { center: c, radius: r }: &Circle) -> bool {
    intersects_on_axis(bbox, circle, &UNIT_X) &&
    intersects_on_axis(bbox, circle, &UNIT_Y) && {
        let closest_corner = nearest_corner(c, &corners(bbox));
        let closest_corner_axis = c.sub(&closest_corner).unit();
        intersects_on_axis(bbox, circle, &closest_corner_axis)
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
        let closest_corner_axis = circle.center.sub(&nearest_corner).unit();
        intersects_on_axis_moving(bbox, circle, dv, &closest_corner_axis)
    }
}

pub fn collides(
    bbox: &BBox,
    circle: &Circle,
    dv: &(f64, f64),
) -> Option<Collision> {
    None
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

    #[test]
    fn find_nearest_corner() {
        let circle = Circle { center: (4.0, 3.0), radius: 2.0 };
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
}