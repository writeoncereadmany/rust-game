use super::vec2d::{Vec2d, UNIT_X, UNIT_Y};

const AXES: [&(f64, f64); 2] = [&UNIT_X, &UNIT_Y];
const EPSILON: f64 = 32.0 * f64::EPSILON;

struct BBox {
    left: f64,
    right: f64,
    bottom: f64,
    top: f64,
}

struct Circle {
    center: (f64, f64),
    radius: f64,
}

struct Convex {
    points: Vec<(f64, f64)>,
    normals: Vec<(f64, f64)>,
}

impl Convex {
    fn new(points: Vec<(f64, f64)>) -> Convex {
        let normals = non_axis_normals(&points);
        Convex { points, normals }
    }
}

enum Shape {
    BBox(BBox),
    Circle(Circle),
    Convex(Convex),
}

impl Shape {
    fn bbox((left, bottom): (f64, f64), width: f64, height: f64) -> Shape {
        Shape::BBox(BBox { left, right: left + width, bottom, top: bottom + height })
    }

    fn circle(center: (f64, f64), radius: f64) -> Shape {
        Shape::Circle(Circle { center, radius })
    }

    fn convex(points: Vec<(f64, f64)>) -> Shape {
        Shape::Convex(Convex::new(points))
    }
}

fn centerpoint(points: &Vec<(f64, f64)>) -> (f64, f64) {
    let (mut tot_x, mut tot_y) = (0.0, 0.0);
    for (x, y) in points {
        tot_x += x;
        tot_y += y;
    }
    (tot_x / points.len() as f64, tot_y / points.len() as f64)
}

fn non_axis_normals(points: &Vec<(f64, f64)>) -> Vec<(f64, f64)> {
    let centerpoint = centerpoint(points);
    let sorted_points = rotation_order_around(&centerpoint, points);
    let mut normals = Vec::new();
    for (i, point) in sorted_points.iter().enumerate() {
        let next_point = sorted_points.get((i + 1) % sorted_points.len()).unwrap();
        let normal = next_point.sub(point).perpendicular().unit();

        // only include non-axis normals, as they're very common so we don't want endless repetitions of them
        if normal.dot(&UNIT_Y).abs() > EPSILON && normal.dot(&UNIT_X).abs() > EPSILON
        {
            normals.push(normal);
        }
    }
    normals
}

fn rotation_order_around(centerpoint: &(f64, f64), points: &Vec<(f64, f64)>) -> Vec<(f64, f64)> {
    let mut sorted_points = points.clone();
    sorted_points.sort_by(|a, b| {
        let (ax, ay) = a.sub(&centerpoint);
        let a_theta = ay.atan2(ax);
        let (bx, by) = b.sub(&centerpoint);
        let b_theta = by.atan2(bx);
        a_theta.partial_cmp(&b_theta).unwrap()
    });
    sorted_points
}

#[derive(Debug, PartialEq)]
struct Projection {
    min: f64,
    max: f64,
}

impl Projection {
    fn overlaps(&self, other: &Projection) -> bool {
        !(self.min > other.max || self.max < other.min)
    }

    fn pushes(&self, other: &Projection) -> Option<Vec<f64>> {
        if self.overlaps(&other) {
            Some(vec![self.max - other.min, self.min - other.max])
        } else {
            None
        }
    }
}

trait Project {
    fn project(&self, axis: &(f64, f64)) -> Projection;
}

impl Project for BBox {
    fn project(&self, axis @ (ax, ay): &(f64, f64)) -> Projection {
        let (min_x, max_x) = if ax > &0.0 { (self.left, self.right) } else { (self.right, self.left) };
        let (min_y, max_y) = if ay > &0.0 { (self.bottom, self.top) } else { (self.top, self.bottom) };
        Projection { min: axis.dot(&(min_x, min_y)), max: axis.dot(&(max_x, max_y)) }
    }
}

impl Project for Circle {
    fn project(&self, axis: &(f64, f64)) -> Projection {
        let projected_center = axis.dot(&self.center);
        Projection { min: projected_center - self.radius, max: projected_center + self.radius }
    }
}

impl Project for Convex {
    fn project(&self, axis: &(f64, f64)) -> Projection {
        let (mut min, mut max) = (f64::NAN, f64::NAN);
        for point in &self.points {
            let projection = axis.dot(&point);
            (min, max) = (min.min(projection), max.max(projection));
        }
        Projection { min, max }
    }
}

impl Project for Shape {
    fn project(&self, axis: &(f64, f64)) -> Projection {
        match self {
            Shape::BBox(bbox) => bbox.project(axis),
            Shape::Circle(circle) => circle.project(axis),
            Shape::Convex(convex) => convex.project(axis)
        }
    }
}

trait Push<T> {
    fn pushes(&self, other: &T) -> Option<Vec<(f64, f64)>>;
}

impl Push<Circle> for Circle {
    fn pushes(&self, other: &Circle) -> Option<Vec<(f64, f64)>> {
        pushes(self, other, &[&other.center.sub(&self.center).unit()])
    }
}

impl Push<BBox> for BBox {
    fn pushes(&self, other: &BBox) -> Option<Vec<(f64, f64)>> {
        pushes(self, other, &AXES)
    }
}

impl Push<Convex> for Convex {
    fn pushes(&self, other: &Convex) -> Option<Vec<(f64, f64)>> {
        let normals: Vec<&(f64, f64)> = self.normals.iter().chain(&other.normals).chain(AXES).collect();
        pushes(self, other, &normals)
    }
}

fn pushes<A: Project, B: Project>(a: &A, b: &B, separating_axes: &[&(f64, f64)]) -> Option<Vec<(f64, f64)>> {
    if !a.project(&UNIT_X).overlaps(&b.project(&UNIT_X)) || !a.project(&UNIT_Y).overlaps(&b.project(&UNIT_Y)) {
        return None;
    }

    let mut all_pushes = Vec::new();
    for axis in separating_axes {
        let proj_a = a.project(axis);
        let proj_b = b.project(axis);
        if let Some(pushes) = proj_a.pushes(&proj_b) {
            for push in pushes {
                all_pushes.push(axis.scale(&push));
            }
        } else {
            return None;
        }
    }
    Some(all_pushes)
}


#[cfg(test)]
mod tests {
    use googletest::prelude::{approx_eq, none};
    use googletest::{assert_that, unordered_elements_are};
    use googletest::matcher::{Matcher, MatcherResult};
    use googletest::matchers::some;
    use super::*;
    use crate::shapes::vec2d::{UNIT_X, UNIT_Y, Vec2d};

    #[test]
    fn test_bbox_projections()
    {
        let bbox = Shape::bbox((1.0, 2.0), 2.0, 3.0);
        assert_eq!(bbox.project(&UNIT_X), Projection { min: 1.0, max: 3.0 });
        assert_eq!(bbox.project(&UNIT_Y), Projection { min: 2.0, max: 5.0 });
        assert_eq!(bbox.project(&UNIT_X.scale(&-1.0)), Projection { min: -3.0, max: -1.0 });
        assert_eq!(bbox.project(&UNIT_Y.scale(&-1.0)), Projection { min: -5.0, max: -2.0 });
    }

    #[test]
    fn test_circle_projections()
    {
        let circle = Shape::circle((4.0, 3.0), 2.0);
        assert_eq!(circle.project(&UNIT_X), Projection { min: 2.0, max: 6.0 });
        assert_eq!(circle.project(&UNIT_Y), Projection { min: 1.0, max: 5.0 });
        assert_eq!(circle.project(&(0.8, 0.6)), Projection { min: 3.0, max: 7.0 });
    }

    #[test]
    fn test_convex_projections()
    {
        let points = vec![(4.0, 3.0), (8.0, 6.0), (7.0, 2.0)];
        let convex = Shape::convex(points);
        assert_eq!(convex.project(&UNIT_X), Projection { min: 4.0, max: 8.0 });
        assert_eq!(convex.project(&UNIT_Y), Projection { min: 2.0, max: 6.0 });
        assert_eq!(convex.project(&(0.8, 0.6)), Projection { min: 5.0, max: 10.0 });
    }

    #[test]
    fn no_overlap_when_other_is_right()
    {
        assert_eq!(Projection { min: 5.0, max: 6.0 }.overlaps(&Projection { min: 10.0, max: 12.0 }), false);
    }

    #[test]
    fn no_overlap_when_other_is_left()
    {
        assert_eq!(Projection { min: 5.0, max: 6.0 }.overlaps(&Projection { min: 1.0, max: 2.0 }), false);
    }

    #[test]
    fn overlap_when_other_entirely_contained()
    {
        assert_eq!(Projection { min: 1.0, max: 6.0 }.overlaps(&Projection { min: 3.0, max: 4.0 }), true);
    }

    #[test]
    fn overlap_when_other_entirely_contains()
    {
        assert_eq!(Projection { min: 3.0, max: 4.0 }.overlaps(&Projection { min: 1.0, max: 6.0 }), true);
    }


    #[test]
    fn overlap_left_end() {
        assert_eq!(Projection { min: 2.0, max: 4.0 }.overlaps(&Projection { min: 1.0, max: 3.0 }), true);
    }

    #[test]
    fn overlap_right_end() {
        assert_eq!(Projection { min: 3.0, max: 6.0 }.overlaps(&Projection { min: 1.0, max: 3.0 }), true);
    }

    #[test]
    fn calculate_normals_for_convex_hull() {
        assert_that!(
            non_axis_normals(&vec![(0.0, 0.0), (6.0, 8.0), (7.0, 7.0)]),
            unordered_elements_are!(
                approx((-0.8, 0.6)),
                approx((f64::sqrt(0.5), f64::sqrt(0.5))),
                approx((f64::sqrt(0.5), -f64::sqrt(0.5)))
            ));
    }

    #[test]
    fn exclude_axis_normals_for_convex_hull() {
        // note: we exclude axis normals, which is why we only get one normal
        assert_that!(
            non_axis_normals(&vec![(2.0, 2.0), (6.0, 2.0), (2.0, 5.0)]),
            unordered_elements_are!(approx((0.6, 0.8))));
    }

    #[test]
    fn non_overlapping_circles() {
        let circle1 = Circle { center: (0.0, 0.0), radius: 5.0 };
        let circle2 = Circle { center: (0.0, 10.0), radius: 3.0 };

        assert_that!(circle1.pushes(&circle2), none());
    }

    #[test]
    fn overlapping_circles_provides_pushout_along_line_joining_centers() {
        let circle1 = Circle { center: (0.0, 0.0), radius: 10.0 };
        let circle2 = Circle { center: (0.0, 10.0), radius: 5.0 };
        assert_that!(
            circle1.pushes(&circle2),
            some(unordered_elements_are!(approx((0.0, 5.0)), approx((0.0, -25.0))))
        );
    }

    #[test]
    fn non_overlapping_boxes() {
        let box1 = BBox { left: 1.0, right: 4.0, bottom: 2.0, top: 5.0 };
        let box2 = BBox { left: 5.0, right: 6.0, bottom: 3.0, top: 4.0 };
        assert_that!(box1.pushes(&box2), none());
    }

    #[test]
    fn overlapping_boxes_provide_pushouts_on_x_any_y_axes() {
        let box1 = BBox { left: 1.0, right: 4.0, bottom: 2.0, top: 5.0 };
        let box2 = BBox { left: 3.0, right: 6.0, bottom: 3.0, top: 4.0 };
        assert_that!(
            box1.pushes(&box2),
            some(unordered_elements_are!(
                approx((1.0, 0.0)), approx((-5.0, 0.0)),
                approx((0.0, 2.0)), approx((0.0, -2.0))
            )));
    }

    #[test]
    fn non_overlapping_convex_meshes() {
        let triangle1 = Convex::new(vec![(4.0, 10.0), (10.0, 12.0), (7.0, 5.0)]);
        let triangle2 = Convex::new(vec![(4.0, 2.0), (12.0, 8.0), (7.0, -2.0)]);
        assert_that!(triangle1.pushes(&triangle2), none());
    }

    #[test]
    fn overlapping_convex_meshes_push_on_all_axes() {
        let triangle1 = Convex::new(vec![(0.0, 0.0), (0.0, 10.0), (10.0, 0.0)]);
        let triangle2 = Convex::new(vec![(4.0, 4.0), (14.0, 4.0), (14.0, 14.0)]);

        assert_that!(
            triangle1.pushes(&triangle2),
            some(unordered_elements_are!(
                // h-push
                approx((6.0, 0.0)),
                approx((-14.0, 0.0)),
                // v-pushes
                approx((0.0, 6.0)),
                approx((0.0, -14.0)),
                // triangle 1 slope pushes
                approx((1.0, 1.0)),
                approx((-14.0, -14.0)),
                // triangle 2 slope pushes
                approx((-10.0, 10.0)),
                approx((5.0, -5.0))
            )));
    }

    struct Vec2dMatcher {
        expected: (f64, f64),
    }

    fn approx(expected: (f64, f64)) -> Vec2dMatcher {
        Vec2dMatcher { expected }
    }

    impl Matcher for Vec2dMatcher {
        type ActualT = (f64, f64);

        fn matches(&self, (actual_x, actual_y): &Self::ActualT) -> MatcherResult {
            let (expected_x, expected_y) = self.expected;
            let (match_x, match_y) = (approx_eq(expected_x), approx_eq(expected_y));
            match (match_x.matches(actual_x), match_y.matches(actual_y)) {
                (MatcherResult::Match, MatcherResult::Match) => MatcherResult::Match,
                _otherwise => MatcherResult::NoMatch,
            }
        }

        fn describe(&self, result: MatcherResult) -> String {
            match result {
                MatcherResult::Match => format!("is equal to {:?}, given some tolerance", self.expected),
                MatcherResult::NoMatch => format!("is not equal to {:?}, given some tolerance", self.expected),
            }
        }
    }
}