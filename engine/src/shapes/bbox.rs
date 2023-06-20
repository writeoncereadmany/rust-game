use super::push::Push;

#[derive(Clone, Copy)]
pub struct BBox {
    left: f64,
    right: f64,
    top: f64,
    bottom: f64
}

pub struct BoxBuilder {
    left: f64,
    bottom: f64
}

impl BBox {
    pub fn from(left: f64, bottom: f64) -> BoxBuilder {
        BoxBuilder { left, bottom }
    }

    pub fn left(&self) -> f64 {
        self.left
    }

    pub fn right(&self) -> f64 {
        self.right
    }

    pub fn top(&self) -> f64 {
        self.top
    }

    pub fn bottom(&self) -> f64 {
        self.bottom
    }

    pub fn width(&self) -> f64 {
        self.right - self.left
    }

    pub fn height(&self) -> f64 {
        self.top - self.bottom
    }

    pub fn translate(&self, dx: f64, dy: f64) -> BBox{
        BBox {
            left: self.left + dx,
            right: self.right + dx,
            top: self.top + dy,
            bottom: self.bottom + dy
        }
    }

    pub fn touches(&self, other: &BBox) -> bool {
        self.left < other.right && 
        self.right > other.left &&
        self.bottom < other.top &&
        self.top > other.bottom
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
        let swept_left = other.left() - rx.max(0.0);
        let swept_right = other.right() - rx.min(0.0);
        let swept_bottom = other.bottom() - ry.max(0.0);
        let swept_top = other.top() - ry.min(0.0);

        if !self.intersects(other, rt) { return None }
        
        let hpush = if rx > 0.0 { self.left() - swept_right } else { self.right() - swept_left };
        let vpush = if ry > 0.0 { self.bottom() - swept_top } else { self.top() - swept_bottom };
        
        match(hpush.abs() > rx.abs(), vpush.abs() > ry.abs())
        {
            (true, true) => None,
            (false, true) => Some((hpush, 0.0)),
            (true, false) => Some((0.0, vpush)),
            (false, false) => if hpush.abs() / rx.abs() > vpush.abs() / ry.abs() {
                Some((hpush, 0.0))
            } else {
                Some((0.0, vpush))
            }
        }
    }

    fn intersects(&self, other: &BBox, (rx, ry): &(f64, f64)) -> bool {
        (other.left() - rx.max(0.0)) < self.right() 
        && self.left() < (other.right() - rx.min(0.0)) 
        && (other.bottom() - ry.max(0.0)) < self.top() 
        && self.bottom() <(other.top() - ry.min(0.0))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn no_collision_where_axis_of_separation_exists() {
        let bbox = BBox::from(0.0, 0.0).to(10.0, 10.0);

        assert_eq!(bbox.push(&BBox::from(-10.0, 0.0).to(-2.0, 10.0), &(1.0, 0.0)), None);
        assert_eq!(bbox.push(&BBox::from(12.0, 0.0).to(14.0, 10.0), &(-1.0, 0.0)), None);
        assert_eq!(bbox.push(&BBox::from(0.0, -10.0).to(10.0, -3.0), &(0.0, 1.0)), None);
        assert_eq!(bbox.push(&BBox::from(0.0, 12.0).to(10.0, 15.0), &(0.0, -1.0)), None);
    }

    #[test]
    fn no_collision_where_objects_had_already_collided() {
        let bbox = BBox::from(0.0, 0.0).to(10.0, 10.0);

        assert_eq!(bbox.push(&BBox::from(5.0, 5.0).to(8.0, 8.0), &(1.0, 0.0)), None);
    }

    #[test]
    fn collision_where_objects_are_newly_collided() {
        let bbox = BBox::from(0.0, 0.0).to(10.0, 10.0);

        assert_eq!(bbox.push(&BBox::from(-5.0, 5.0).to(5.0, 15.0), &(10.0, 0.0)), Some((-5.0, 0.0)));

    }
}