pub trait Push<A> {
    // returns the vector that this will apply to other as a result of the first collision between
    // other and this, based on other's motion relative to this and the collision normals of both shapes
    // (or None, if the two objects didn't first collide this iteration)
    fn push(&self, other: &A, _relative_translation: &(f64, f64)) -> Option<(f64, f64)>;
}