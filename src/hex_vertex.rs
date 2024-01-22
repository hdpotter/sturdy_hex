use crate::*;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct HexVertex {
    three_q: i32,
    three_r: i32,
}

impl HexVertex {
    // accessors
    pub fn three_q(&self) -> i32 {
        self.three_q
    }

    pub fn three_r(&self) -> i32 {
        self.three_r
    }

    pub fn three_s(&self) -> i32 {
        -(self.three_q + self.three_r)
 
    }

    // constructors
    pub fn new(three_q: i32, three_r: i32) -> HexVertex {
        HexVertex {
            three_q,
            three_r,
        }
    }

    // other methods
    pub fn on_positive_basis(&self) -> bool {
        (self.three_q() + 1).rem_euclid(3) == 0 && (self.three_r() + 1).rem_euclid(3) == 0
    }

    pub fn on_negative_basis(&self) -> bool {
        (self.three_q() - 1).rem_euclid(3) == 0 && (self.three_r() - 1).rem_euclid(3) == 0
    }

    pub fn get_outgoing_edge(&self, i: i32) -> HexHalfEdge {
        let direction =
            if self.on_positive_basis() {
                HexVertex::get_unit_coord(2*i)
            } else if self.on_negative_basis() {
                HexVertex::get_unit_coord(2*i + 1)
            } else {
                panic!("HexVertex is not on a vertex");
            };
        
        HexHalfEdge::new(
            *self,
            HexVertex::new(
                self.three_q() + direction.three_q(),
                self.three_r() + direction.three_r(),
            )
        )
    }

    pub fn get_incoming_edge(&self, i: i32) -> HexHalfEdge {
        self.get_outgoing_edge(i).twin()
    }

    pub fn get_unit_coord(i: i32) -> HexVertex {
        match i.rem_euclid(6) {
            0 => HexVertex::new(2, -1),
            1 => HexVertex::new(1, 1),
            2 => HexVertex::new(-1, 2),
            3 => HexVertex::new(-2, 1),
            4 => HexVertex::new(-1, -1),
            5 => HexVertex::new(1, -2),
            _ => panic!("logic error; rem_euclid(6) should return one of the previous cases"),
        }
    }

    pub fn translate(&self, translation: &HexCoord) -> HexVertex {
        HexVertex::new(
            self.three_q() + 3 * translation.q(),
            self.three_r() + 3 * translation.r(),
        )
    }
}


