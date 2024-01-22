use crate::{HexTransform, HexCoord, HexHalfEdge, HexVertex};

pub struct HexRangeIterator {
    q: i32,
    r: i32,
    range: u32,
    transform: HexTransform,
}

impl HexRangeIterator {
    pub fn new(range: u32, transform: HexTransform) -> Self {
        let q = -(range as i32) - 1; //-1 so first next() call gives first hex instead of second
        let r = range as i32; //set to cause q to increment on the first next()

        Self {
            q,
            r,
            range,
            transform,
        }
    }

}

impl Iterator for HexRangeIterator {
    type Item = HexCoord;
    
    // todo: consider storing range as i32 to avoid casts
    fn next(&mut self) -> Option<Self::Item> {
        if self.r < std::cmp::min(self.range as i32, -self.q + (self.range) as i32) {
            self.r += 1;
        } else {
            self.q += 1;
            self.r = std::cmp::max(-(self.range as i32), -self.q - (self.range as i32));
        }
        
        if self.q > self.range as i32 {
            None
        } else {
            Some(self.transform * HexCoord::new(self.q, self.r))
        }
    }
}


pub struct HexHalfEdgeIterator {
    face: HexCoord,
    i: i32,
}

impl HexHalfEdgeIterator {
    pub fn new(face: HexCoord) -> Self {
        Self {
            face,
            i: -1,
        }
    }
}

impl Iterator for HexHalfEdgeIterator {
    type Item = HexHalfEdge;

    fn next(&mut self) -> Option<Self::Item> {
        self.i += 1;
        if self.i >= 6 {
            None
        } else {
            Some(self.face.get_half_edge(self.i))
        }
    }
}

pub struct HexVertexIterator {
    face: HexCoord,
    i: i32,
}

impl HexVertexIterator {
    pub fn new(face: HexCoord) -> Self {
        Self {
            face,
            i: -1,
        }
    }
}

impl Iterator for HexVertexIterator {
    type Item = HexVertex;

    fn next(&mut self) -> Option<Self::Item> {
        self.i += 1;
        if self.i >= 6 {
            None
        } else {
            Some(self.face.get_vertex(self.i))
        }
    }
}