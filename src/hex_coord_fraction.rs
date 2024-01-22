use std::ops;
use crate::*;

#[derive(Copy, Clone, Debug)]
pub struct HexCoordFraction {
    q: f32,
    r: f32
}

impl HexCoordFraction {
    // accessors
    pub fn q(&self) -> f32 {
        self.q
    }

    pub fn r(&self) -> f32 {
        self.r
    }

    pub fn s(&self) -> f32 {
        -(self.q + self.r)
    }

    // constructor
    pub fn new(q: f32, r: f32) -> HexCoordFraction {
        HexCoordFraction {
            q,
            r,
        }
    }

    // constants
    pub const ZERO: HexCoordFraction = HexCoordFraction {q: 0.0, r: 0.0};

    // methods
    pub fn get_unit_coord(i: i32) -> HexCoordFraction {
        match i.rem_euclid(6) {
            0 => HexCoordFraction::new(0.0, -1.0),
            1 => HexCoordFraction::new(0.0, 1.0),
            2 => HexCoordFraction::new(-1.0, 0.0),
            3 => HexCoordFraction::new(1.0, 0.0),
            4 => HexCoordFraction::new(-1.0, 1.0),
            5 => HexCoordFraction::new(1.0, -1.0),
            _ => panic!("logic error; rem_euclid(6) should return one of the previous cases"),
        }
    }

    pub fn rotate_around(&self, pivot: HexCoordFraction, rotation: i32) -> HexCoordFraction {
        let relative = *self - pivot;
        
        // mathematica: Table[{{0,-1},{1,1}}^n.{q,r},{n,0,5}]
        let relative = match rotation.rem_euclid(6) {
            0 => HexCoordFraction::new(relative.q(), relative.r()),
            1 => HexCoordFraction::new(-relative.r(), relative.q() + relative.r()),
            2 => HexCoordFraction::new(-relative.q() - relative.r(), relative.q()),
            3 => HexCoordFraction::new(-relative.q(), -relative.r()),
            4 => HexCoordFraction::new(relative.r(), -relative.q() - relative.r()),
            5 => HexCoordFraction::new(relative.q() + relative.r(), -relative.q()),
            _ => panic!("logic error; rem_euclid(6) should return one of the previous cases"),
        };
                
        pivot + relative
    }

    pub fn hex_distance(a: HexCoordFraction, b: HexCoordFraction) -> f32 {
        ((a.q() - b.q()).abs() + (a.r() - b.r()).abs() + (a.s() - b.s()).abs()) / 2.0
    }

    pub fn round(&self) -> HexCoord {
        let mut q_round = self.q().round();
        let mut r_round = self.r().round();
        let s_round = self.s().round();
        
        let q_delta = (q_round - self.q()).abs();
        let r_delta = (r_round - self.r()).abs();
        let s_delta = (s_round - self.s()).abs();
        
        if q_delta > r_delta && q_delta > s_delta {
            q_round = -(r_round + s_round);
        } else if r_delta > s_delta {
            r_round = -(q_round + s_round);
        }
        // third possibility is irrelevant because we don't use s_round in the final answer
        
        // todo!("handle float overflow gracefully");
        HexCoord::new(
            q_round as i32,
            r_round as i32,
        )
    }
}

// into
impl From<HexCoord> for HexCoordFraction {
    fn from(item: HexCoord) -> HexCoordFraction {
        HexCoordFraction {
            q: item.q() as f32,
            r: item.r() as f32,
        }
    }
}

impl From<HexVertex> for HexCoordFraction {
    fn from(item: HexVertex) -> HexCoordFraction {
        HexCoordFraction {
            q: item.three_q() as f32 / 3.0,
            r: item.three_r() as f32 / 3.0,
        }
    }
}


// operator overloads
impl ops::Add for HexCoordFraction {
    type Output = HexCoordFraction;

    fn add(self, other: HexCoordFraction) -> HexCoordFraction {
        HexCoordFraction {
            q: self.q() + other.q(),
            r: self.r() + other.r(),
        }
    }
}

impl ops::AddAssign for HexCoordFraction {
    fn add_assign(&mut self, other: HexCoordFraction) {
        *self = *self + other;
    }
}

impl ops::Sub for HexCoordFraction {
    type Output = HexCoordFraction;

    fn sub(self, other: HexCoordFraction) -> HexCoordFraction {
        HexCoordFraction {
            q: self.q() - other.q(),
            r: self.r() - other.r(),
        }
    }
}

impl ops::SubAssign for HexCoordFraction {
    fn sub_assign(&mut self, other: HexCoordFraction) {
        *self = *self - other;
    }
}

impl ops::Neg for HexCoordFraction {
    type Output = HexCoordFraction;

    fn neg(self) -> HexCoordFraction {
        HexCoordFraction{
            q: -self.q(),
            r: -self.r(),
        }
    }
}

impl ops::Mul::<i32> for HexCoordFraction {
    type Output = HexCoordFraction;

    fn mul(self, other: i32) -> HexCoordFraction {
        HexCoordFraction {
            q: self.q * other as f32,
            r: self.r * other as f32,
        }
    }
}

impl ops::Mul::<HexCoordFraction> for i32 {
    type Output = HexCoordFraction;

    fn mul(self, other: HexCoordFraction) -> HexCoordFraction {
        HexCoordFraction {
            q: self as f32 * other.q(),
            r: self as f32 * other.r(),
        }
    }
}

impl ops::MulAssign::<i32> for HexCoordFraction {
    fn mul_assign(&mut self, other: i32) {
        *self = *self * other;
    }
}

impl ops::Mul::<f32> for HexCoordFraction {
    type Output = HexCoordFraction;

    fn mul(self, other: f32) -> HexCoordFraction {
        HexCoordFraction {
            q: self.q * other,
            r: self.r * other,
        }
    }
}

impl ops::Mul::<HexCoordFraction> for f32 {
    type Output = HexCoordFraction;

    fn mul(self, other: HexCoordFraction) -> HexCoordFraction {
        HexCoordFraction {
            q: self * other.q(),
            r: self * other.r(),
        }
    }
}

impl ops::MulAssign::<f32> for HexCoordFraction {
    fn mul_assign(&mut self, other: f32) {
        *self = *self * other;
    }
}