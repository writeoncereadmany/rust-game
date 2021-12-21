// returns the shortest vector that other needs to be moved by to no longer
// be overlapping self, or Option.None if they are already not overlapping
pub trait Push<A> {
    fn push(&self, other: &A) -> Option<(f64, f64)>;
}