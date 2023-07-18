use std::cmp::Ordering;

use super::bbox::BBox;
use super::push::Push;
use super::vec2d::{Vec2d, UNIT_X, ZERO, UNIT_Y};

const PUSH_EPSILON: f64 = 0.001;

#[derive(Clone)]
pub struct ConvexMesh {
    points: Vec<(f64, f64)>,
    pub normals: Vec<(f64, f64)>
}

#[derive(Clone)]
pub struct Meshed<A>
where A: Clone {
    pub item: A,
    pub mesh: Mesh
}

#[derive(Clone)]
pub enum Mesh {
    Convex(ConvexMesh),
    AABB(BBox),
}

impl ConvexMesh {
    pub fn new(points: Vec<(f64, f64)>, normals: Vec<(f64, f64)>) -> Self {
        ConvexMesh {
            points,
            normals
        }
    }

    pub fn rect(left: f64, bottom: f64, width: f64, height: f64) -> Self {
        ConvexMesh::new(
            vec![(left, bottom), (left+width, bottom), (left+width, bottom+height), (left, bottom+height)],
            vec![(-1.0, 0.0), (1.0, 0.0), (0.0, -1.0), (0.0, 1.0)]
        )
    }

    pub fn translate(&self, dx: f64, dy: f64) -> ConvexMesh {
        ConvexMesh {
            points: self.points.iter().map(|(x, y)| (x + dx, y + dy)).collect(),
            normals: self.normals.clone()
        }
    }

    pub fn project(&self, normal: &(f64, f64), trans: &(f64, f64)) -> (f64, f64) {
        // NAN.min(x) and NAN.max(x) both always return x, so we'll set both min and max to the
        // projection on first iteration. 
        let (min, max) = self.points.iter().fold((f64::NAN, f64::NAN), |(min, max), point| { 
            let proj = normal.dot(point); 
            (min.min(proj), max.max(proj))
        });
        let trans_proj = -normal.dot(trans);
        (min + trans_proj.min(0.0), max + trans_proj.max(0.0))
    }
}

fn overlaps((min_first, max_first): &(f64, f64), (min_second, max_second): &(f64, f64)) -> bool {
    min_first < max_second && max_first > min_second
}

impl Push<Mesh> for Mesh {
    fn push(&self, other: &Mesh, relative_translation: &(f64, f64)) -> Option<(f64, f64)> {
        match (self, other) {
            (Mesh::Convex(this), Mesh::Convex(other)) => this.push(other, relative_translation),
            (Mesh::AABB(this), Mesh::AABB(that)) => this.push(that, relative_translation),
            _ => None
        }
    }
}



impl Mesh {
    pub fn rect(left: f64, bottom: f64, width: f64, height: f64) -> Self {
        Mesh::Convex(ConvexMesh::new(
            vec![(left, bottom), (left+width, bottom), (left+width, bottom+height), (left, bottom+height)],
            vec![(-1.0, 0.0), (1.0, 0.0), (0.0, -1.0), (0.0, 1.0)]
        ))
    }

    pub fn convex(points: Vec<(f64, f64)>, normals: Vec<(f64, f64)>) -> Self {
        Mesh::Convex(ConvexMesh::new(points, normals))
    }

    pub fn project(&self, normal: &(f64, f64), trans: &(f64, f64)) -> (f64, f64) {
        match self {
            Mesh::Convex(mesh) => mesh.project(normal, trans),
            Mesh::AABB(mesh) => mesh.project(normal, trans)
        }
    }

    pub fn translate(&self, dx: f64, dy: f64) -> Mesh {
        match self {
            Mesh::Convex(mesh) => Mesh::Convex(mesh.translate(dx, dy)),
            Mesh::AABB(mesh) => Mesh::AABB(mesh.translate(dx, dy))
        }
    }
}

impl Push<ConvexMesh> for ConvexMesh {

    fn push(&self, other: &ConvexMesh, relative_travel: &(f64, f64)) -> Option<(f64, f64)> {
        let mut latest_push: Option<(f64, f64)> = None;

        let travel_axis = relative_travel.perpendicular();
        let (my_min, my_max) = self.project(&travel_axis, &(0.0, 0.0));
        let (their_min, their_max) = other.project(&travel_axis, &(0.0, 0.0));

        if (my_min > their_max) || (their_min > my_max) { return None }

        for normal in normals_as_applied_to_other(self, other) {
            let (my_min, my_max) = self.project(&normal, &(0.0, 0.0));
            let (their_min, their_max) = other.project(&normal, relative_travel);
            if my_max < their_min || my_min > their_max { return None; } 
            
            let push_distance = my_max - their_min;
            if push_distance + normal.dot(relative_travel) <= PUSH_EPSILON {
                let potential_push = normal.scale(push_distance);
                latest_push = latest_push.map_or(Some(potential_push), |push| Some(latest(push, potential_push, relative_travel)));
            }
        }
        latest_push
    }
}

fn normals_as_applied_to_other(first: &ConvexMesh, other: &ConvexMesh) -> Vec<(f64, f64)> {
    first.normals.iter()
        .map(|mine| mine.scale(1.0))
        .chain(other.normals.iter().map(|theirs| theirs.scale(-1.0)))
        .collect()
}

fn latest(a: (f64, f64), b: (f64, f64), rt: &(f64, f64)) -> (f64, f64) {
    // we want the push with the smallest component along the axis of travel, as that's what's hit last
    // ie they're separated until hitting that axis
    let proj_a = a.sq_len() / a.unit().dot(rt);
    let proj_b = b.sq_len() / b.unit().dot(rt);
    match proj_a.partial_cmp(&proj_b) {
        Some(Ordering::Greater) => a,
        _otherwise => b
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn project_shows_places_on_relative_normals_with_no_translation_vector()
    {
        let rect = Mesh::rect(4.0, 6.0, 3.0, 5.0);
        assert_eq!(rect.project(&(1.0, 0.0), &(0.0, 0.0)), (4.0, 7.0));
        assert_eq!(rect.project(&(0.0, 1.0), &(0.0, 0.0)), (6.0, 11.0));
    }   

    #[test]
    fn project_shows_places_on_relative_normals_with_translation_vector()
    {
        let rect = Mesh::rect(4.0, 6.0, 3.0, 5.0);
        assert_eq!(rect.project(&(1.0, 0.0), &(7.0, 3.0)), (-3.0, 7.0));
        assert_eq!(rect.project(&(1.0, 0.0), &(-2.0, -3.0)), (4.0, 9.0));

        assert_eq!(rect.project(&(0.0, 1.0), &(7.0, 3.0)), (3.0, 11.0));
        assert_eq!(rect.project(&(0.0, 1.0), &(-2.0, -3.0)), (6.0, 14.0));

    }   

    #[test]
    fn pushes_against_motion(){
        let base = Mesh::rect(0.0, -10.0, 10.0, 10.0);
        let square = Mesh::rect(-1.0, -1.0, 2.0, 2.0);

        assert_eq!(base.push(&square, &(0.0, -5.0)), Some((0.0, 1.0)));
    }

    #[test]
    fn does_not_push_with_motion(){
        let deep_rect = Mesh::rect(0.0, -20.0, 10.0, 20.0);
        let square = Mesh::rect(-1.0, -1.0, 2.0, 2.0);

        assert_eq!(deep_rect.push(&square, &(0.0, 5.0)), None);
    }

    #[test]
    fn does_not_push_more_than_movement() {
        let deep_rect = Mesh::rect(0.0, -20.0, 10.0, 20.0);
        let square = Mesh::rect(-1.0, -1.0, 2.0, 2.0);

        // was already 1 unit to other side of barrier, but has only moved 0.5 this frame: was already through
        // the object, let it continue
        assert_eq!(deep_rect.push(&square, &(0.0, -0.5)), None);
    }

    #[test]
    fn does_not_push_more_than_movement_in_normal_component() {
        let deep_rect = Mesh::rect(-100.0, -20.0, 100.0, 0.0);
        let square = Mesh::rect(-1.0, -1.0, 2.0, 2.0);

        // horizontal speed should not matter here, only vertical
        assert_eq!(deep_rect.push(&square, &(20.0, -0.5)), None);
    }

    #[test]
    fn pushes_against_first_impacted_edge_from_side(){
        let base = Mesh::rect(0.0, -10.0, 10.0, 10.0);
        let square = Mesh::rect(-1.0, -1.0, 2.0, 2.0);

        // intersects evenly deeply into top and left, but hits left side, therefore push should be all left
        assert_eq!(base.push(&square, &(10.0, -2.0)), Some((-1.0, 0.0)));
    }

    #[test]
    fn pushes_against_first_impacted_edge_from_side_with_even_approach(){
        let base = Mesh::rect(0.0, -10.0, 10.0, 10.0);
        let square = Mesh::rect(-1.0, -2.0, 2.0, 2.0);

        // approaches left and down at equal rates, but ends up hitting left side
        assert_eq!(base.push(&square, &(10.0, -10.0)), Some((-1.0, 0.0)));
    }

    #[test]
    fn pushes_against_first_impacted_edge_from_top_with_even_approach(){
        let base = Mesh::rect(0.0, -10.0, 10.0, 10.0);
        let square = Mesh::rect(0.0, -1.0, 2.0, 2.0);

        // approaches left and down at equal rates, but ends up hitting top side
        assert_eq!(base.push(&square, &(10.0, -10.0)), Some((0.0, 1.0)));
    }

    #[test]
    fn pushes_against_first_impacted_edge_from_top(){
        let pushes_up_and_left = Mesh::rect(0.0, -10.0, 10.0, 10.0);
        let square = Mesh::rect(-1.0, -1.0, 2.0, 2.0);

        // intersects evenly deeply into left and top sides, but with larger vertical component to approach,
        // therefore push should be all up
        assert_eq!(pushes_up_and_left.push(&square, &(2.0, -10.0)), Some((0.0, 1.0)));
    }

    #[test]
    fn no_collision_where_axis_of_separation_exists() {
        let rect = Mesh::rect(0.0, 0.0, 10.0, 10.0);

        assert_eq!(rect.push(&Mesh::rect(-10.0, 0.0, -2.0, 10.0), &(1.0, 0.0)), None);
        assert_eq!(rect.push(&Mesh::rect(12.0, 0.0, 14.0, 10.0), &(-1.0, 0.0)), None);
        assert_eq!(rect.push(&Mesh::rect(0.0, -10.0, 10.0, -3.0), &(0.0, 1.0)), None);
        assert_eq!(rect.push(&Mesh::rect(0.0, 12.0, 10.0, 15.0), &(0.0, -1.0)), None);
    }

    #[test]
    fn no_collision_where_objects_had_already_collided() {
        let rect = Mesh::rect(0.0, 0.0, 10.0, 10.0);

        assert_eq!(rect.push(&Mesh::rect(5.0, 5.0, 8.0, 8.0), &(1.0, 0.0)), None);
    }

    #[test]
    fn collision_where_objects_are_newly_collided() {
        let bbox = Mesh::rect(0.0, 0.0, 10.0, 10.0);

        // a bunch of cases where the incoming box would end up embedded in the center of the pushing box
        // but coming from the outside, at speed, from different directions
        assert_eq!(bbox.push(&Mesh::rect(4.0, 4.0, 2.0, 2.0), &(10.0, 2.0)), Some((-6.0, 0.0)));
        assert_eq!(bbox.push(&Mesh::rect(4.0, 4.0, 2.0, 2.0), &(10.0, -2.0)), Some((-6.0, 0.0)));

        assert_eq!(bbox.push(&Mesh::rect(4.0, 4.0, 2.0, 2.0), &(-10.0, 2.0)), Some((6.0, 0.0)));
        assert_eq!(bbox.push(&Mesh::rect(4.0, 4.0, 2.0, 2.0), &(-10.0, -2.0)), Some((6.0, 0.0)));

        assert_eq!(bbox.push(&Mesh::rect(4.0, 4.0, 2.0, 2.0), &(2.0, -10.0)), Some((0.0, 6.0)));
        assert_eq!(bbox.push(&Mesh::rect(4.0, 4.0, 2.0, 2.0), &(-2.0, -10.0)), Some((0.0, 6.0)));

        assert_eq!(bbox.push(&Mesh::rect(4.0, 4.0, 2.0, 2.0), &(2.0, 10.0)), Some((0.0, -6.0)));
        assert_eq!(bbox.push(&Mesh::rect(4.0, 4.0, 2.0, 2.0), &(-2.0, 10.0)), Some((0.0, -6.0)));
    }

    // tunneling is a phenomenon wherein an object goes from not colliding with an object on one side to not colliding
    // with it on the other, passing through the object. if we just look at collision meshes at a point in time,
    // if the movement is too great, we see them pass through each other.
    // this is (one of the reasons) why we pass in the translation vector: this allows us to compare not just the
    // collision meshes, but also the path travelled, to check that for intersections too
    #[test]
    fn should_not_tunnel() {
        let bbox = Mesh::rect(0.0, 0.0, 10.0, 10.0);

        // cardinal directions of travel
        assert_eq!(bbox.push(&Mesh::rect(12.0, 4.0, 2.0, 2.0), &(20.0, 0.0)), Some((-14.0, 0.0)));
        assert_eq!(bbox.push(&Mesh::rect(-4.0, 4.0, 2.0, 2.0), &(-20.0, 0.0)), Some((14.0, 0.0)));
        assert_eq!(bbox.push(&Mesh::rect(4.0, 14.0, 2.0, 2.0), &(0.0, 20.0)), Some((0.0, -16.0)));
        assert_eq!(bbox.push(&Mesh::rect(4.0, -4.0, 2.0, 2.0), &(0.0, -20.0)), Some((0.0, 14.0)));

        // diagonal travel across object: pushes along whichever axis is breached _last_
        assert_eq!(bbox.push(&Mesh::rect(12.0, 14.0, 2.0, 2.0), &(20.0, 20.0)), Some((-14.0, 0.0)));
        assert_eq!(bbox.push(&Mesh::rect(14.0, 12.0, 2.0, 2.0), &(20.0, 20.0)), Some((0.0, -14.0)));
    }

    // here, snagging means an object passing diagonally past another without colliding, but such that the
    // alinged box around the beginning and ending positions of the moving object _does_ overlap with the
    // relatively static object. this means we need to consider the direction of travel as a separating axis,
    // as well as the cardinal directions, although no push can be exerted by this axis.
    // eg:
    //      ____
    //     |    |\
    //     | A  | \
    //     |____|  \
    //      \    ___\
    //   ___ \  |    |
    //  |   | \ | A' |
    //  | B |  \|____|
    //  |___|  
    // 
    // in this case, the aligned box overlaps, but the swept volume doesn't
    #[test]
    fn should_not_snag() {
        let bbox = Mesh::rect(0.0, 0.0, 10.0, 10.0);

        assert_eq!(bbox.intersects(&Mesh::rect(14.0, 5.0, 2.0, 2.0), &(10.0, -10.0)), true);
        assert_eq!(bbox.intersects(&Mesh::rect(16.0, 5.0, 2.0, 2.0), &(10.0, -10.0)), false);
    }
}