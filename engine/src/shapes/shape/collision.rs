/*
 * Represents a collision between two moving objects, taken from the frame of reference
 * of the second object with the first object moving along a vector
 * dt: the point during the movement when the objects collide, in the range 0-1
 * push: the minimal vector that needs to be applied to the first object so it no longer
 * intersects with the second.
 */
use googletest::matcher::{Matcher, MatcherResult};
use googletest::matchers::approx_eq;
use googletest::matches_pattern;

#[derive(Debug, PartialEq)]
pub struct Collision {
    pub dt: f64,
    pub push: (f64, f64)
}

pub struct CollisionMatcher {
    dt: f64,
    push: (f64, f64)
}

impl Matcher for CollisionMatcher {
    type ActualT = Collision;

    fn matches(&self, Collision { dt: actual_dt, push: actual_push } : &Collision) -> MatcherResult {
        let dt_match = approx_eq(self.dt).matches(actual_dt);
        let push_x_match = approx_eq(self.push.0).matches(&actual_push.0);
        let push_y_match = approx_eq(self.push.1).matches(&actual_push.1);
        match (dt_match, push_x_match, push_y_match) {
            (MatcherResult::Match, MatcherResult::Match, MatcherResult::Match) => MatcherResult::Match,
            _ => MatcherResult::NoMatch
        }
    }

    fn describe(&self, matcher_result: MatcherResult) -> String {
        match matcher_result {
            MatcherResult::Match => {
                format!("is equal to Collision {{ dt: {0:?}, push: {1:?} }}", self.dt, self.push)
            }
            MatcherResult::NoMatch => {
                format!("is not equal to Collision {{ dt: {0:?}, push: {1:?} }}", self.dt, self.push)
            }
        }
    }
}

pub fn eq_collision(dt: f64, push: (f64, f64)) -> CollisionMatcher {
    CollisionMatcher { dt, push }
}