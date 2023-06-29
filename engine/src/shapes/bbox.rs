use super::{push::Push, vec2d::Vec2d};

#[derive(Clone, Copy)]
pub struct BBox {
    pub left: f64,
    pub right: f64,
    pub top: f64,
    pub bottom: f64
}

pub struct BoxBuilder {
    left: f64,
    bottom: f64
}

impl BBox {
    pub fn from(left: f64, bottom: f64) -> BoxBuilder {
        BoxBuilder { left, bottom }
    }

    pub fn width(&self) -> f64 {
        self.right - self.left
    }

    pub fn height(&self) -> f64 {
        self.top - self.bottom
    }

    pub fn project(&self, normal@&(nx, ny): &(f64, f64)) -> (f64, f64) {
        let (min_x, max_x) = if nx > 0.0 { (self.left, self.right) } else { (self.right, self.left) };
        let (min_y, max_y) = if ny > 0.0 { (self.bottom, self.top) } else { (self.top, self.bottom) };
         
        (normal.dot(&(min_x, min_y)), normal.dot(&(max_x, max_y)))
    }

    pub fn translate(&self, dx: f64, dy: f64) -> BBox{
        BBox {
            left: self.left + dx,
            right: self.right + dx,
            top: self.top + dy,
            bottom: self.bottom + dy
        }
    }
}

impl BoxBuilder {
    pub fn to(&self, right: f64, top: f64) -> BBox {
        BBox { left: self.left, right, bottom: self.bottom, top }
    }

    pub fn size(&self, width: f64, height: f64) -> BBox {
        BBox { left: self.left, right: self.left + width, bottom: self.bottom, top: self.bottom + height }
    }
}

impl Push<BBox> for BBox {

    fn push(&self, other: &BBox, rt@&(rx, ry): &(f64, f64)) -> Option<(f64, f64)> {
        let swept_left = other.left - rx.max(0.0);
        let swept_right = other.right - rx.min(0.0);
        let swept_bottom = other.bottom - ry.max(0.0);
        let swept_top = other.top - ry.min(0.0);

        if !self.intersects(other, rt) { return None }
        
        let hpush = if rx > 0.0 { self.left - swept_right } else { self.right - swept_left };
        let vpush = if ry > 0.0 { self.bottom - swept_top } else { self.top - swept_bottom };
        
        match(hpush.abs() > rx.abs(), vpush.abs() > ry.abs())
        {
            (true, true) => None,
            (false, true) => Some((hpush, 0.0)),
            (true, false) => Some((0.0, vpush)),
            (false, false) => if hpush.abs() / rx.abs() < vpush.abs() / ry.abs() {
                Some((hpush, 0.0))
            } else {
                Some((0.0, vpush))
            }
        }
    }

    fn intersects(&self, other: &BBox, rel_trans@(rx, ry): &(f64, f64)) -> bool {
        let swept_boxes_intersect = (other.left - rx.max(0.0)) < self.right 
        && self.left < (other.right - rx.min(0.0)) 
        && (other.bottom - ry.max(0.0)) < self.top 
        && self.bottom < (other.top - ry.min(0.0));

        // we don't need to bother normalising here, because we don't care how much the boxes 
        // are separated along this axis: just whether they are or not
        let direction_of_travel_axis = rel_trans.perpendicular();

        let (this_min, this_max) = self.project(&direction_of_travel_axis);
        let (that_min, that_max) = other.project(&direction_of_travel_axis);

        let separated_by_axis_of_travel = this_min > that_max || that_min > this_max;

        swept_boxes_intersect && !separated_by_axis_of_travel
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn no_collision_where_axis_of_separation_exists() {
        let bbox = BBox::from(0.0, 0.0).to(10.0, 10.0);

        assert_eq!(bbox.push(&BBox::from(-10.0, 0.0).size(8.0, 10.0), &(1.0, 0.0)), None);
        assert_eq!(bbox.push(&BBox::from(12.0, 0.0).size(2.0, 10.0), &(-1.0, 0.0)), None);
        assert_eq!(bbox.push(&BBox::from(0.0, -10.0).size(10.0, 7.0), &(0.0, 1.0)), None);
        assert_eq!(bbox.push(&BBox::from(0.0, 12.0).size(10.0, 3.0), &(0.0, -1.0)), None);
    }

    #[test]
    fn no_collision_where_objects_had_already_collided() {
        let bbox = BBox::from(0.0, 0.0).size(10.0, 10.0);

        assert_eq!(bbox.push(&BBox::from(5.0, 5.0).size(3.0, 3.0), &(1.0, 0.0)), None);
    }

    #[test]
    fn collision_where_objects_are_newly_collided() {
        let bbox = BBox::from(0.0, 0.0).to(10.0, 10.0);

        // a bunch of cases where the incoming box would end up embedded in the center of the pushing box
        // but coming from the outside, at speed, from different directions
        assert_eq!(bbox.push(&BBox::from(4.0, 4.0).size(2.0, 2.0), &(10.0, 2.0)), Some((-6.0, 0.0)));
        assert_eq!(bbox.push(&BBox::from(4.0, 4.0).size(2.0, 2.0), &(10.0, -2.0)), Some((-6.0, 0.0)));

        assert_eq!(bbox.push(&BBox::from(4.0, 4.0).size(2.0, 2.0), &(-10.0, 2.0)), Some((6.0, 0.0)));
        assert_eq!(bbox.push(&BBox::from(4.0, 4.0).size(2.0, 2.0), &(-10.0, -2.0)), Some((6.0, 0.0)));

        assert_eq!(bbox.push(&BBox::from(4.0, 4.0).size(2.0, 2.0), &(2.0, -10.0)), Some((0.0, 6.0)));
        assert_eq!(bbox.push(&BBox::from(4.0, 4.0).size(2.0, 2.0), &(-2.0, -10.0)), Some((0.0, 6.0)));

        assert_eq!(bbox.push(&BBox::from(4.0, 4.0).size(2.0, 2.0), &(2.0, 10.0)), Some((0.0, -6.0)));
        assert_eq!(bbox.push(&BBox::from(4.0, 4.0).size(2.0, 2.0), &(-2.0, 10.0)), Some((0.0, -6.0)));
    }

    // tunneling is a phenomenon wherein an object goes from not colliding with an object on one side to not colliding
    // with it on the other, passing through the object. if we just look at collision meshes at a point in time,
    // if the movement is too great, we see them pass through each other.
    // this is (one of the reasons) why we pass in the translation vector: this allows us to compare not just the
    // collision meshes, but also the path travelled, to check that for intersections too
    #[test]
    fn should_not_tunnel() {
        let bbox = BBox::from(0.0, 0.0).to(10.0, 10.0);

        // cardinal directions of travel
        assert_eq!(bbox.push(&BBox::from(12.0, 4.0).size(2.0, 2.0), &(20.0, 0.0)), Some((-14.0, 0.0)));
        assert_eq!(bbox.push(&BBox::from(-4.0, 4.0).size(2.0, 2.0), &(-20.0, 0.0)), Some((14.0, 0.0)));
        assert_eq!(bbox.push(&BBox::from(4.0, 14.0).size(2.0, 2.0), &(0.0, 20.0)), Some((0.0, -16.0)));
        assert_eq!(bbox.push(&BBox::from(4.0, -4.0).size(2.0, 2.0), &(0.0, -20.0)), Some((0.0, 14.0)));

        // diagonal travel across object: pushes along whichever axis is breached _last_
        assert_eq!(bbox.push(&BBox::from(12.0, 14.0).size(2.0, 2.0), &(20.0, 20.0)), Some((-14.0, 0.0)));
        assert_eq!(bbox.push(&BBox::from(14.0, 12.0).size(2.0, 2.0), &(20.0, 20.0)), Some((0.0, -14.0)));
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
    // in this case, the aligned box for A overlaps B, but the swept volume doesn't.
    #[test]
    fn should_not_snag() {
        let bbox = BBox::from(0.0, 0.0).to(10.0, 10.0);

        assert_eq!(bbox.intersects(&BBox::from(14.0, 5.0).size(2.0, 2.0), &(10.0, -10.0)), true);
        assert_eq!(bbox.intersects(&BBox::from(16.0, 5.0).size(2.0, 2.0), &(10.0, -10.0)), false);
    }
}