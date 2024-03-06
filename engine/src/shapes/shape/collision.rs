/*
 * Represents a collision between two moving objects, taken from the frame of reference
 * of the second object with the first object moving along a vector
 * dt: the point during the movement when the objects collide, in the range 0-1
 * push: the minimal vector that needs to be applied to the first object so it no longer
 * intersects with the second.
 */
#[derive(Debug, PartialEq)]
pub struct Collision {
    pub dt: f64,
    pub push: (f64, f64)
}