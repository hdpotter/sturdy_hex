use std::ops;
use crate::{*, hex_range_iterator::{HexVertexIterator, HexHalfEdgeIterator}};

/// A coordinate specifying a hex on a hex grid.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct HexCoord {
    q: i32,
    r: i32,
}

impl HexCoord {
    // accessors
    /// Returns the *q* coordinate of the hex in the cubical coordinate system described [here](https://www.redblobgames.com/grids/hexagons/).
    pub fn q(&self) -> i32 {
        self.q
    }

    /// Returns the *r* coordinate of the hex in the cubical coordinate system described [here](https://www.redblobgames.com/grids/hexagons/).
    pub fn r(&self) -> i32 {
        self.r
    }

    /// Returns the *s* coordinate of the hex in the cubical coordinate system described [here](https://www.redblobgames.com/grids/hexagons/).
    pub fn s(&self) -> i32 {
        -(self.q + self.r)
    }

    // constructor
    /// Creates a `HexCoord` with the given *q* and *r* coordinates, as described [here](https://www.redblobgames.com/grids/hexagons/).
    /// It is not necessary to specify an *s* coordinate, because SturdyHex enforces that *q*, *r*, and *s* sum to zero.
    pub fn new(q: i32, r: i32) -> HexCoord {
        HexCoord {
            q,
            r,
        }
    }

    // constants
    /// A hex coordinate representing the origin of the hex coordinate system.
    pub const ZERO: HexCoord = HexCoord{q: 0, r: 0};

    // methods
    /// Returns a hex coordinate representing one of the six unit directions from a hex to its neighbors.
    /// Parameter `i` specifies which neighbor.
    /// For convenience, `i` is wrapped (not clamped) to the range [0, 5].
    pub fn get_unit_coord(i: i32) -> HexCoord {
        match i.rem_euclid(6) {
            0 => HexCoord::new(0, -1),
            1 => HexCoord::new(0, 1),
            2 => HexCoord::new(-1, 0),
            3 => HexCoord::new(1, 0),
            4 => HexCoord::new(-1, 1),
            5 => HexCoord::new(1, -1),
            _ => panic!("logic error; rem_euclid(6) should return one of the previous cases"),
        }
    }

    /// Returns the `i`th vertex of `self`.
    /// For convenience, `i` is wrapped (not clamped) to the range [0, 5].
    pub fn get_vertex(&self, i: i32) -> HexVertex {
        HexVertex::get_unit_coord(i).translate(self)
    }

    /// Returns an iterator over the vertices of `self`.
    pub fn vertices(&self) -> HexVertexIterator {
        HexVertexIterator::new(*self)
    }

    /// Returns the `i`th half-edge of `self`.
    /// For convenience, `i` is wrapped (not clamped) to the range [0, 5].
    pub fn get_half_edge(&self, i: i32) -> HexHalfEdge {
        HexHalfEdge::new(
            self.get_vertex(i),
            self.get_vertex(i+1),
        )
    }

    /// Returns an iterator over the half-edges of `self`.
    pub fn edges(&self) -> HexHalfEdgeIterator {
        HexHalfEdgeIterator::new(*self)
    }

    /// Returns a hex coordinate representing `self` rotated a sixth-turn CCW around the hex grid origin.
    pub fn rotate_back(&self) -> HexCoord {
        HexCoord {
            q: -self.s(),
            r: -self.q(),
        }
    }

    /// Returns a hex coordinate representing `self` rotated a sixth-turn CW around the hex grid origin.
    pub fn rotate_forward(&self) -> HexCoord {
        HexCoord {
            q: -self.r(),
            r: -self.s(),
        }
    }

    /// Returns a hex coordinate representing `self` rotated `i` sixth-turns CCW around `pivot`.
    pub fn rotate_around(&self, pivot: HexCoord, rotation: i32) -> HexCoord {
        let relative = *self - pivot;
        
        // mathematica: Table[{{0,-1},{1,1}}^n.{q,r},{n,0,5}]
        let relative = match rotation.rem_euclid(6) {
            0 => HexCoord::new(relative.q(), relative.r()),
            1 => HexCoord::new(-relative.r(), relative.q() + relative.r()),
            2 => HexCoord::new(-relative.q() - relative.r(), relative.q()),
            3 => HexCoord::new(-relative.q(), -relative.r()),
            4 => HexCoord::new(relative.r(), -relative.q() - relative.r()),
            5 => HexCoord::new(relative.q() + relative.r(), -relative.q()),
            _ => panic!("logic error; rem_euclid(6) should return one of the previous cases"),
        };
                
        pivot + relative
    }

    /// Returns the number of hexes in the shortest path along the hex grid from `a` to `b`.
    /// Includes `b` in the count but not `a`; if `a` and `b` are neighbors, `hex_distance(a, b)` returns 1.
    pub fn hex_distance(a: HexCoord, b: HexCoord) -> i32 {
        ((a.q() - b.q()).abs() + (a.r() - b.r()).abs() + (a.s() - b.s()).abs()) / 2
    }




}


// operator overloads
impl ops::Add for HexCoord {
    type Output = HexCoord;

    fn add(self, other: HexCoord) -> HexCoord {
        HexCoord {
            q: self.q() + other.q(),
            r: self.r() + other.r(),
        }
    }
}

impl ops::AddAssign for HexCoord {
    fn add_assign(&mut self, other: HexCoord) {
        *self = *self + other;
    }
}

impl ops::Sub for HexCoord {
    type Output = HexCoord;

    fn sub(self, other: HexCoord) -> HexCoord {
        HexCoord {
            q: self.q() - other.q(),
            r: self.r() - other.r(),
        }
    }
}

impl ops::SubAssign for HexCoord {
    fn sub_assign(&mut self, other: HexCoord) {
        *self = *self - other;
    }
}

impl ops::Neg for HexCoord {
    type Output = HexCoord;

    fn neg(self) -> HexCoord {
        HexCoord {
            q: -self.q(),
            r: -self.r(),
        }
    }
}

impl ops::Mul::<i32> for HexCoord {
    type Output = HexCoord;

    fn mul(self, other: i32) -> HexCoord {
        HexCoord {
            q: self.q * other,
            r: self.r * other,
        }
    }
}

impl ops::Mul::<HexCoord> for i32 {
    type Output = HexCoord;

    fn mul(self, other: HexCoord) -> HexCoord {
        HexCoord {
            q: self * other.q(),
            r: self * other.r(),
        }
    }
}

impl ops::MulAssign::<i32> for HexCoord {
    fn mul_assign(&mut self, other: i32) {
        *self = *self * other;
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn unit_coords_sum_zero() {
        let mut sum = HexCoord::ZERO;
        for i in 0..6 {
            sum += HexCoord::get_unit_coord(i);
        }
        assert_eq!(sum, HexCoord::ZERO);
    }
}