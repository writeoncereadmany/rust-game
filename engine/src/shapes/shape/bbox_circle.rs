use crate::shapes::shape::collision::Collision;
use crate::shapes::shape::circle::Circle;
use crate::shapes::shape::bbox::{BBox, corners};
use crate::shapes::shape::line::Line;
use crate::shapes::shape::projection;
use crate::shapes::vec2d::{UNIT_X, UNIT_Y, Vec2d};
use super::projection::{intersects_on_axis, Projection, Projects};

pub fn intersects(bbox: &BBox, circle @ Circle { center: c, radius: r }: &Circle) -> bool {
    let closest_corner = nearest_corner(c, &corners(bbox));
    let closest_corner_axis = c.sub(&closest_corner).unit();
    intersects_on_axis(bbox, circle, &UNIT_X)
        && intersects_on_axis(bbox, circle, &UNIT_Y)
        && intersects_on_axis(bbox, circle, &closest_corner_axis)
}

pub fn intersects_moving(bbox: &BBox, circle: &Circle, dv: &(f64, f64)) -> bool {
    false
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
    use googletest::assert_that;
    use googletest::matchers::{none, some};
    use crate::shapes::shape::collision::eq_collision;

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
}