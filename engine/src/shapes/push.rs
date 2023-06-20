pub trait Push<A> {
    fn intersects(&self, other: &A, relative_translation: &(f64, f64)) -> bool {
        self.push(other, relative_translation).is_some()
    }

    // returns the vector that this will apply to other as a result of the first collision between
    // other and this, based on other's motion relative to this and the collision normals of both shapes
    // (or None, if the two objects didn't first collide this iteration)
    // on the basis that: both objects are in their position _after_ having moved
    fn push(&self, other: &A, relative_translation: &(f64, f64)) -> Option<(f64, f64)>;
}