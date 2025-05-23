use super::projection::{collision_on_axis, intersects_on_axis, Projection, Projects};
use crate::shapes::shape::collision::Collision;
use crate::shapes::shape::line::Line;
use crate::shapes::vec2d::Vec2d;

#[derive(Clone, Debug)]
pub struct Circle {
    pub center: (f64, f64),
    pub radius: f64
}

impl Projects for Circle {
    fn project(&self, axis: &(f64, f64)) -> Projection {
        let center_projection = axis.dot(&self.center);
        Projection { min: center_projection - self.radius, max: center_projection + self.radius }
    }
}

pub fn intersects(
    Circle { center: c1, radius: r1 }: &Circle,
    Circle { center: c2, radius: r2 }: &Circle
) -> bool {
    let distance_sq = c1.sub(&c2).sq_len();
    let sum_radii_sq = (r1 + r2) * (r1 + r2);
    distance_sq < sum_radii_sq
}

pub fn intersects_moving(
    circle1: &Circle,
    circle2: &Circle,
    dv: &(f64, f64)
) -> bool {
    if dv.sq_len() == 0.0 {
        return intersects(circle1, circle2)
    }

    intersects(circle1, circle2) ||
    intersects(&translate(circle1, dv), circle2) || {
        let unit_dv = dv.unit();
        intersects_on_axis(circle1, circle2, &unit_dv.perpendicular()) && {
            let sweep = Line::new(circle1.center, circle1.center.plus(dv));
            intersects_on_axis(&sweep, &circle2.center, &unit_dv)
        }
    }
}

pub fn translate(&Circle { center: (cx, cy), radius}: &Circle, (dx, dy): &(f64, f64)) -> Circle {
    Circle { center: (cx + dx, cy + dy), radius }
}

pub fn collides(
    circle1 @ Circle { center: c1, radius: r1 }: &Circle,
    circle2 @ Circle { center: c2, radius: r2 }: &Circle,
    dv: &(f64, f64)
) -> Option<Collision> {
    if !intersects_moving(circle1, circle2, dv) || intersects(circle1, circle2) {
        return None;
    }

    let sum_of_radii = r1 + r2;
    let movement_vector_unit = dv.unit();
    let normal_movement_unit = movement_vector_unit.perpendicular();
    let opposite_length = normal_movement_unit.dot(c1) - normal_movement_unit.dot(c2);
    let adjacent_length = f64::sqrt((sum_of_radii * sum_of_radii) - (opposite_length * opposite_length));
    let separation_axis = normal_movement_unit.scale(&opposite_length).sub(&movement_vector_unit.scale(&adjacent_length));

    collision_on_axis(circle1, circle2, dv, &separation_axis.unit())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shapes::shape::collision::eq_collision;
    use googletest::assert_that;
    use googletest::matchers::{none, some};

    // projection tests

    #[test]
    fn project_x_axis() {
        let circle = Circle { center: (4.0, 3.0), radius: 2.0 };
        assert_eq!(circle.project(&(1.0, 0.0)), Projection { min: 2.0, max: 6.0 });
    }

    #[test]
    fn project_y_axis() {
        let circle = Circle { center: (4.0, 3.0), radius: 2.0 };
        assert_eq!(circle.project(&(0.0, 1.0)), Projection { min: 1.0, max: 5.0 });
    }

    #[test]
    fn project_diagonal_axis() {
        let circle = Circle { center: (4.0, 3.0), radius: 2.0 };
        assert_eq!(circle.project(&(0.8, 0.6)), Projection { min: 3.0, max: 7.0 });
    }

    // intersection tests

    #[test]
    fn horizontal_overlap() {
        let circle1 = Circle { center: (0.0, 0.0), radius: 2.0 };
        let circle2 = Circle { center: (3.0, 0.0), radius: 2.0 };
        assert_eq!(intersects(&circle1, &circle2), true);
    }

    #[test]
    fn horizontal_separation() {
        let circle1 = Circle { center: (0.0, 0.0), radius: 2.0 };
        let circle2 = Circle { center: (5.0, 0.0), radius: 2.0 };
        assert_eq!(intersects(&circle1, &circle2), false);
    }

    #[test]
    fn vertical_overlap() {
        let circle1 = Circle { center: (0.0, 0.0), radius: 2.0 };
        let circle2 = Circle { center: (0.0, 3.0), radius: 2.0 };
        assert_eq!(intersects(&circle1, &circle2), true);
    }

    #[test]
    fn vertical_separation() {
        let circle1 = Circle { center: (0.0, 0.0), radius: 2.0 };
        let circle2 = Circle { center: (0.0, 5.0), radius: 2.0 };
        assert_eq!(intersects(&circle1, &circle2), false);
    }

    #[test]
    fn diagonal_separation() {
        // overlaps on both horizontal and vertical axes, but there is a diagonal
        // line of separation
        let circle1 = Circle { center: (0.0, 0.0), radius: 5.0 };
        let circle2 = Circle { center: (8.0, 8.0), radius: 5.0 };
        assert_eq!(intersects(&circle1, &circle2), false);
    }

    // collision tests
    #[test]
    fn horizontal_collision() {
        let circle1 = Circle { center: (2.0, 0.0), radius: 2.0 };
        let circle2 = Circle { center: (8.0, 0.0), radius: 2.0 };
        assert_that!(
            collides(&circle1, &circle2, &(4.0, 0.0)),
            some(eq_collision(0.5, (-2.0, 0.0))));
    }

    #[test]
    fn horizontal_collision_stops_short() {
        let circle1 = Circle { center: (2.0, 0.0), radius: 2.0 };
        let circle2 = Circle { center: (8.0, 0.0), radius: 2.0 };
        assert_that!(
            collides(&circle1, &circle2, &(1.5, 0.0)),
            none());
    }

    #[test]
    fn vertical_collision_offset() {
        let circle1 = Circle { center: (2.0, 2.0), radius: 2.0 };
        let circle2 = Circle { center: (2.0, 9.0), radius: 2.0 };
        assert_that!(
            collides(&circle1, &circle2, &(0.0, 5.0)),
            some(eq_collision(0.6, (0.0, -2.0))));
    }

    #[test]
    fn off_axis_collision() {
        // sum of radii = 5, collision point is at (6, 3) pushing on vector (-4, 3)
        let circle1 = Circle { center: (0.0, 3.0), radius: 2.0 };
        let circle2 = Circle { center: (10.0, 0.0), radius: 3.0 };
        assert_that!(
            collides(&circle1, &circle2, &(10.0, 0.0)),
            some(eq_collision(0.6, (-2.56, 1.92 ))));
    }

    #[test]
    fn off_axis_near_miss() {
        // separated by 4 normal to movement, but sum of radii is 3 - sails straight past
        let circle1 = Circle { center: (0.0, 4.0), radius: 2.0 };
        let circle2 = Circle { center: (10.0, 0.0), radius: 1.0 };
        assert_that!(
            collides(&circle1, &circle2, &(20.0, 0.0)),
            none());
    }

    #[test]
    fn intersects_when_initial_circles_overlap()
    {
        let circle1 = Circle { center: (0.0, 0.0), radius: 2.0 };
        let circle2 = Circle { center: (2.0, 0.0), radius: 1.0 };

        assert_eq!(
            intersects_moving(&circle1, &circle2, &(0.0, 2.0)),
            true
        );
    }

    #[test]
    fn intersects_when_final_circles_overlap()
    {
        let circle1 = Circle { center: (0.0, 0.0), radius: 2.0 };
        let circle2 = Circle { center: (2.0, 2.0), radius: 1.0 };

        assert_eq!(
            intersects_moving(&circle1, &circle2, &(0.0, 2.0)),
            true
        );
    }

    #[test]
    fn intersects_swept_area()
    {
        let circle1 = Circle { center: (0.0, 0.0), radius: 2.0 };
        let circle2 = Circle { center: (2.0, 5.0), radius: 1.0 };

        assert_eq!(
            intersects_moving(&circle1, &circle2, &(0.0, 10.0)),
            true
        );
    }

    #[test]
    fn no_intersection_when_too_far_from_swept_area()
    {
        let circle1 = Circle { center: (0.0, 0.0), radius: 2.0 };
        let circle2 = Circle { center: (3.5, 5.0), radius: 1.0 };

        assert_eq!(
            intersects_moving(&circle1, &circle2, &(0.0, 10.0)),
            false
        );
    }

    #[test]
    fn no_intersection_when_outside_range_of_motion()
    {
        let circle1 = Circle { center: (0.0, 0.0), radius: 2.0 };
        let circle2 = Circle { center: (2.0, 5.0), radius: 1.0 };

        assert_eq!(
            intersects_moving(&circle1, &circle2, &(0.0, 2.0)),
            false
        );
    }

}