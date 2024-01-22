use crate::*;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct HexHalfEdge {
    source: HexVertex,
    destination: HexVertex,
}

impl HexHalfEdge {
    // accessors
    pub fn source(&self) -> HexVertex {
        self.source
    }

    pub fn destination(&self) -> HexVertex {
        self.destination
    }

    // constructor
    pub fn new(source: HexVertex, destination: HexVertex) -> HexHalfEdge {
        HexHalfEdge {
            source,
            destination,
        }
    }

    // methods
    pub fn hex(&self) -> HexCoord {
        // vector from source to destination
        let displacement = HexVertex::new(
            self.destination.three_q() - self.source.three_q(),
            self.destination.three_r() - self.source.three_r()
        );

        // rotate it so it points from source to center instead of source to destination
        let center_from_source = HexVertex::new(-displacement.three_r(), -displacement.three_s());

        // add source back in
        let center = HexVertex::new(
            self.source().three_q() + center_from_source.three_q(),
            self.source().three_r() + center_from_source.three_r(),
        );

        // convert to hex coordinates and return
        HexCoord::new(
            center.three_q() / 3,
            center.three_r() / 3,
        )
    }

    pub fn twin(&self) -> HexHalfEdge {
        HexHalfEdge {
            source: self.destination(),
            destination: self.source(),
        }
    }

    pub fn next(&self) -> HexHalfEdge {
        // vector from source to destination
        let displacement = HexVertex::new(
            self.destination.three_q() - self.source.three_q(),
            self.destination.three_r() - self.source.three_r()
        );

        // rotate it so it gives the displacement for the next edge instead of the current one
        let next_edge_displacement = HexVertex::new(-displacement.three_r(), -displacement.three_s());

        // add destination in as the new source
        HexHalfEdge {
            source: self.destination(),
            destination: HexVertex::new(
                self.destination.three_q() + next_edge_displacement.three_q(),
                self.destination.three_r() + next_edge_displacement.three_r(),
            )
        }
    }

    pub fn prev(&self) -> HexHalfEdge {
        // vector from source to destination
        let displacement = HexVertex::new(
            self.destination.three_q() - self.source.three_q(),
            self.destination.three_r() - self.source.three_r()
        );

        // rotate it so it gives the displacement for the previous edge instead of the current one
        let prev_edge_displacement = HexVertex::new(-displacement.three_s(), -displacement.three_q());

        // add source in as the new destination
        HexHalfEdge {
            source: HexVertex::new(
                self.source.three_q() - prev_edge_displacement.three_q(),
                self.source.three_r() - prev_edge_displacement.three_r(),
            ),
            destination: self.source(),
        }
    }

    pub fn translate(&self, translation: &HexCoord) -> HexHalfEdge {
        HexHalfEdge::new(
            self.source.translate(translation),
            self.destination.translate(translation),
        )
    }
}