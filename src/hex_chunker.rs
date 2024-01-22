use crate::*;

pub struct HexChunker {
    chunk_radius: u32,
    area: u32,
    shift: u32,
}

impl HexChunker {
    pub fn new(chunk_radius: u32) -> HexChunker {
        HexChunker {
            chunk_radius,
            area: 3 * chunk_radius * chunk_radius + 3 * chunk_radius + 1,
            shift: 3 * chunk_radius + 2,
        }
    }

    pub fn get_chunk_center(&self, chunk_coord: HexCoord) -> HexCoord {
        HexCoord::new(
            (2 * self.chunk_radius + 1) as i32 * chunk_coord.q() + self.chunk_radius as i32 * chunk_coord.r(),
            self.chunk_radius as i32 * -chunk_coord.q() + (self.chunk_radius + 1) as i32 * chunk_coord.r(),
        )
    }

    // algo from https://observablehq.com/@sanderevers/hexagon-tiling-of-an-hexagonal-grid
    pub fn get_containing_chunk(&self, coord: HexCoord) -> HexCoord {
        let xh = (coord.r() + self.shift as i32 * coord.q()).div_euclid(self.area as i32);
        let yh = (coord.s() + self.shift as i32 * coord.r()).div_euclid(self.area as i32);
        let zh = (coord.q() + self.shift as i32 * coord.s()).div_euclid(self.area as i32);

        HexCoord::new (
            (1 + xh - yh).div_euclid(3),
            (1 + yh - zh).div_euclid(3),
        )
    }
}