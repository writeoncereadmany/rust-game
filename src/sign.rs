#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum Sign {
    POSITIVE,
    ZERO,
    NEGATIVE
}

impl Sign {
    pub fn unit_f64(&self) -> f64 {
        match self {
            Sign::POSITIVE => 1.0,
            Sign::ZERO => 0.0,
            Sign::NEGATIVE => -1.0
        }
    }
}

pub trait Signed {
    fn sign(&self) -> Sign;
}

impl Signed for f64 {
    fn sign(&self) -> Sign {
        match *self {
            x if x > 0.0 => Sign::POSITIVE,
            x if x < 0.0 => Sign::NEGATIVE,
            _ => Sign::ZERO
        }
    }
}