use crate::*;
use std::ops;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct HexTransform {
    translation: HexCoord,
    rotation: i32,
}

impl HexTransform {
    // accessors
    pub fn translation(&self) -> HexCoord {
        self.translation
    }

    pub fn rotation(&self) -> i32 {
        self.rotation
    }

    // constructor
    pub fn new(translation: HexCoord, rotation: i32) -> HexTransform {
        HexTransform {
            translation,
            rotation,
        }
    }

    pub fn from_translation(translation: HexCoord) -> HexTransform {
        HexTransform {
            translation,
            rotation: 0,
        }
    }

    pub fn from_rotation(rotation: i32) -> HexTransform {
        HexTransform {
            translation: HexCoord::ZERO,
            rotation,
        }
    }

    // constants
    pub const IDENTITY:HexTransform = HexTransform {
        translation: HexCoord::ZERO,
        rotation: 0,
    };

    // methods
    pub fn apply_to(&self, a: HexCoord) -> HexCoord {
        self.translation() + a.rotate_around(HexCoord::ZERO, self.rotation())
    }

    pub fn inverse(&self) -> HexTransform {
        todo!("implement");
    }

    pub fn transformed(&self, transform: HexTransform) -> HexTransform{
        transform * *self
    }

    pub fn translated(&self, translation: HexCoord) -> HexTransform {
        HexTransform::from_translation(translation) * *self
    }

    pub fn rotated(&self, rotation: i32) -> HexTransform {
        HexTransform::from_rotation(rotation) * *self
    }
}

impl ops::Mul for HexTransform {
    type Output = HexTransform;

    fn mul(self, other: HexTransform) -> HexTransform {
        HexTransform::new(
            other.translation().rotate_around(HexCoord::ZERO, self.rotation()) + self.translation(),
            self.rotation() + other.rotation(),
        )
    }
}

impl ops::MulAssign for HexTransform {
    fn mul_assign(&mut self, other: HexTransform) {
        *self = other * *self;
    }
}

impl ops::Mul<HexCoord> for HexTransform {
    type Output = HexCoord;

    fn mul(self, other: HexCoord) -> HexCoord {
        self.apply_to(other)
    }
}

impl ops::MulAssign<HexTransform> for HexCoord {
    fn mul_assign(&mut self, other: HexTransform) {
        *self = other * *self;
    }

}