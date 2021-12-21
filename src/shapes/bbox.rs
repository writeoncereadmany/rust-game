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