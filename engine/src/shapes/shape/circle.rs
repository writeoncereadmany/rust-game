use crate::shapes::shape::collision::Collision;
use crate::shapes::shape::line::Line;
use crate::shapes::vec2d::Vec2d;
use super::projection::{intersects_on_axis, Projection, Projects};

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

fn intersects(
    Circle { center: c1, radius: r1 }: &Circle,
    Circle { center: c2, radius: r2 }: &Circle
) -> bool {
    let distance_sq = c1.sub(&c2).sq_len();
    let sum_radii_sq = (r1 + r2) * (r1 + r2);
    distance_sq < sum_radii_sq
}

fn intersects_moving(
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

fn translate(&Circle { center: (cx, cy), radius}: &Circle, (dx, dy): &(f64, f64)) -> Circle {
    Circle { center: (cx + dx, cy + dy), radius }
}

fn collide(
    Circle { center: c1, radius: r1 }: &Circle,
    Circle { center: c2, radius: r2 }: &Circle,
    dv: &(f64, f64)
) -> Option<Collision> {
    let sum_of_radii = r1 + r2;
    // find the two points on the movement vector where the circles exactly touch
    // ie, the entry/exit points of the collision
    // first: find the nearest point on movement vector to circle2.
    // this will be normal to movement vector:
    let normal_movement: (f64, f64) = dv.perpendicular().unit();
    // multiplied by the distance between the projections of the circles centers:
    let proj_distance: f64 = normal_movement.dot(c1) - normal_movement.dot(c2);
    // which gives us a separating vector of:
    let shortest_line = normal_movement.scale(&proj_distance);
    if shortest_line.len() > sum_of_radii {
        None
    } else {
        // the vectors from c2 to points along the movement vector where radii touch exactly
        // make the hypotenuse of right-angled triangle with one side being that shortest line.
        // find their length:
        let nearest_point = c2.plus(&shortest_line);
        let movement_vector_unit = dv.unit();
        let offset = f64::sqrt((sum_of_radii * sum_of_radii) - shortest_line.sq_len());
        let entry_point = nearest_point.sub(&movement_vector_unit.scale(&offset));
        // if entry point is not on the movement vector, no collision
        // (or circles were already overlapping):
        let movement_proj = Line::new(*c1, c1.plus(&dv)).project(&movement_vector_unit);
        let entry_proj = entry_point.dot(&movement_vector_unit);
        if entry_proj < movement_proj.min || entry_proj > movement_proj.max {
            None
        } else {
            // push is vector from c2 to entry point, scaled by push
            let overlap = movement_proj.max - entry_proj;
            let push_vec_unit = entry_point.sub(&c2).unit();
            let push_vec_scale = -overlap * push_vec_unit.dot(&movement_vector_unit);
            let dist_to_collision = entry_proj - movement_proj.min;
            let total_dist_to_move = movement_proj.max - movement_proj.min;
            let dt = dist_to_collision / total_dist_to_move;
            let push = push_vec_unit.scale(&push_vec_scale);
            Some(Collision { dt, push })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::assert_that;
    use googletest::matchers::{none, some};
    use crate::shapes::shape::collision::eq_collision;

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
            collide(&circle1, &circle2, &(4.0, 0.0)),
            some(eq_collision(0.5, (-2.0, 0.0))));
    }

    #[test]
    fn horizontal_collision_stops_short() {
        let circle1 = Circle { center: (2.0, 0.0), radius: 2.0 };
        let circle2 = Circle { center: (8.0, 0.0), radius: 2.0 };
        assert_that!(
            collide(&circle1, &circle2, &(1.5, 0.0)),
            none());
    }

    #[test]
    fn vertical_collision_offset() {
        let circle1 = Circle { center: (2.0, 2.0), radius: 2.0 };
        let circle2 = Circle { center: (2.0, 9.0), radius: 2.0 };
        assert_that!(
            collide(&circle1, &circle2, &(0.0, 5.0)),
            some(eq_collision(0.6, (0.0, -2.0))));
    }

    #[test]
    fn off_axis_collision() {
        // sum of radii = 5, collision point is at (6, 3) pushing on vector (-4, 3)
        let circle1 = Circle { center: (0.0, 3.0), radius: 2.0 };
        let circle2 = Circle { center: (10.0, 0.0), radius: 3.0 };
        assert_that!(
            collide(&circle1, &circle2, &(10.0, 0.0)),
            some(eq_collision(0.6, (-2.56, 1.92 ))));
    }

    #[test]
    fn off_axis_near_miss() {
        // separated by 4 normal to movement, but sum of radii is 3 - sails straight past
        let circle1 = Circle { center: (0.0, 4.0), radius: 2.0 };
        let circle2 = Circle { center: (10.0, 0.0), radius: 1.0 };
        assert_that!(
            collide(&circle1, &circle2, &(20.0, 0.0)),
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