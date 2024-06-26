use crate::*;
use cgmath::Vector3;
use cgmath::InnerSpace;

pub enum HexOrientation {
    FlatTop,
    PointyTop,
}

pub enum Handedness {
    LeftHanded,
    RightHanded,
}

/// A mapping between worldspace and hex coordinates. 
/// Contains methods to determine which hex contains a worldspace point and calculate the worldspace positions of hex cells and vertices.
/// 
/// SturdyHex allows you to set up a `HexField` at any position, orientation, and scale factor.
/// For example, you can create a `HexField` with a center hex offset to worldspace coordinate *(100, 100, 0)* to place the lower-left corner at the worldspace origin.
/// Likewise, you can create a `HexField` rotated and positioned to be on an overhead display rectangle in worldspace.
/// 
/// When querying which hex contains a worldspace point, `HexField` projects the query point onto the plane of the hex grid.
pub struct HexField {
    origin: Vector3<f32>,
    
    x_basis: Vector3<f32>,
    y_basis: Vector3<f32>,
    z_basis: Vector3<f32>,

    q_basis: Vector3<f32>,
    r_basis: Vector3<f32>,
    s_basis: Vector3<f32>,

    _inner_radius: f32,
    outer_radius: f32,
}

impl HexField {
    /// You can conceptualize a `HexField` as a plane with a hex grid on it, positioned and oriented somewhere in worldspace.
    /// This function returns the worldspace direction of this plane's x-axis.
    pub fn x_basis(&self) -> Vector3<f32> {
        self.x_basis
    }

    /// You can conceptualize a `HexField` as a plane with a hex grid on it, positioned and oriented somewhere in worldspace.
    /// This function returns the worldspace direction of this plane's x-axis.
    pub fn y_basis(&self) -> Vector3<f32> {
        self.y_basis
    }

    /// You can conceptualize a `HexField` as a plane with a hex grid on it, positioned and oriented somewhere in worldspace.
    /// This function returns the worldspace direction normal to this plane's surface.
    pub fn z_basis(&self) -> Vector3<f32> {
        self.z_basis
    }

    /// Creates a `HexField` with the specified parameters.
    /// * `origin`: The worldspace position of the center of hex *(0, 0, 0)*.
    /// * `z_direction`: The worldspace direction of `up` on the hex grid, perpendicular to the plane of hexes
    /// * `pos_y_hex_displacement`: The vector from `origin` to the center of hex *(0, 1, -1)*.  Hex *(0, 1, -1)* is adjacent to `origin`; you can control the scale and rotation (around `z_direction`) of the `HexField` via this parameter.
    pub fn new(
        origin: Vector3<f32>,
        z_direction: Vector3<f32>,
        pos_y_hex_displacement: Vector3<f32>
    ) -> HexField {
        let z_relative = z_direction - origin;
        let y_basis = pos_y_hex_displacement.normalize();

        // orthonormalize z against y and calculate x
        let z_basis = (z_relative - InnerSpace::dot(z_relative, y_basis) * z_relative).normalize();
        let x_basis = Vector3::cross(y_basis, z_basis);

        // calculate q, r, and s (degenerate) basis
        let q_basis = x_basis;
        let r_basis =
            f32::cos(std::f32::consts::TAU / 3.0) * x_basis +
            f32::sin(std::f32::consts::TAU / 3.0) * y_basis;
        let s_basis =
            f32::cos(2.0 * std::f32::consts::TAU / 3.0) * x_basis +
            f32::sin(2.0 * std::f32::consts::TAU / 3.0) * y_basis;
        
        // calculate inner and outer radius
        let inner_radius = pos_y_hex_displacement.magnitude() / 2.0;
        let outer_radius = inner_radius * 2.0 / f32::sqrt(3.0);

        // return struct
        HexField {
            origin,

            x_basis,
            y_basis,
            z_basis,

            q_basis,
            r_basis,
            s_basis,

            _inner_radius: inner_radius,
            outer_radius,
        }
    }

    fn project_onto_basis(point: Vector3<f32>, basis_0: Vector3<f32>, basis_1: Vector3<f32>, basis_2: Vector3<f32>) -> Vector3<f32> {
        let denominator =
            basis_0.x * basis_1.y * basis_2.z - basis_0.z * basis_1.y * basis_2.x +
            basis_0.y * basis_1.z * basis_2.x - basis_0.x * basis_1.z * basis_2.y +
            basis_0.z * basis_1.x * basis_2.y - basis_0.y * basis_1.x * basis_2.z;
        
        let basis_0_numerator =
            point.x * basis_1.y * basis_2.z - point.z * basis_1.y * basis_2.x +
            point.y * basis_1.z * basis_2.x - point.x * basis_1.z * basis_2.y +
            point.z * basis_1.x * basis_2.y - point.y * basis_1.x * basis_2.z;

        let basis_1_numerator =
            basis_0.x * point.y * basis_2.z - basis_0.z * point.y * basis_2.x +
            basis_0.y * point.z * basis_2.x - basis_0.x * point.z * basis_2.y +
            basis_0.z * point.x * basis_2.y - basis_0.y * point.x * basis_2.z;

        let basis_2_numerator =
            basis_0.x * basis_1.y * point.z - basis_0.z * basis_1.y * point.x +
            basis_0.y * basis_1.z * point.x - basis_0.x * basis_1.z * point.y +
            basis_0.z * basis_1.x * point.y - basis_0.y * basis_1.x * point.z;
        
        Vector3 {
            x: basis_0_numerator / denominator,
            y: basis_1_numerator / denominator,
            z: basis_2_numerator / denominator,
        }
    }

    /// Projects an arbitrary vector to lie on the plane of the hex grid.  Returns a result in the local cartesian coordinates of the hex grid.
    pub fn project_onto_plane(&self, position: Vector3<f32>) -> Vector3<f32> {
        HexField::project_onto_basis(position - self.origin, self.x_basis, self.y_basis, self.z_basis)
    }

    /// Returns the exact, fractional coordinates of a given worldspace vector projected onto the hex grid.
    pub fn get_hex_coord_fraction(&self, position: Vector3<f32>) -> HexCoordFraction {
        // calculate q and r coordinates that do not respect the q + r + s = 0 invariant
        let qr = HexField::project_onto_basis((position - self.origin) / self.outer_radius, self.q_basis, self.r_basis, self.z_basis);

        // recalculate qrs to keep the same position but also respect the q + r + s = 0 invariant
        let q = qr.x;
        let r = qr.y; // and s starts as 0, breaking the invariant
        let a: f32 = -0.5; // horizontal component of q and r vectors

        let qp = (q + a * r) / (1.0 - a); // hold position constant and solve for qp, rp to satisfy invariant
        let rp = (q + r * (2.0 * a - 1.0)) / (2.0 * a - 2.0);

        HexCoordFraction::new(
            qp,
            rp,
        )
    }

    /// Returns the hex coordinates, rounded to the nearest hex, of a given worldspace vector projected onto the hex grid.
    pub fn get_hex_coord(&self, position: Vector3<f32>) -> HexCoord {
        self.get_hex_coord_fraction(position).round()
    }

    /// Returns the worldspace coordinates of a hex coordinate.
    pub fn get_position<T: Into<HexCoordFraction>>(&self, coord: T) -> Vector3<f32> {
        let coord = coord.into();
        self.outer_radius * (coord.q() * self.q_basis + coord.r() * self.r_basis + coord.s() * self.s_basis)
    }

    /// Returns the worldspace coordinates of a hex coordinate, translated to be at `height` above the plane of the hex grid.
    pub fn get_position_with_height<T: Into<HexCoordFraction>>(&self, coord: T, height: f32) -> Vector3<f32> {
        self.get_position(coord) + height * self.z_basis
    }

    /// Returns the worldspace coordinates of the `i`th vertex of the hex at the given coordinates, translated to be at `height` above the plane of the hex grid.
    pub fn get_face_vertex_position(&self, face: HexCoord, scale: f32, height: f32, i: i32) -> Vector3<f32> {
        let center = self.get_position_with_height(face, height);
        let outer = self.get_position_with_height(face.get_vertex(i), height);
        center + scale * (outer - center)
    }

    pub fn get_source_vertex_position(&self, edge: HexHalfEdge, scale: f32, height: f32) -> Vector3<f32> {
        let center = self.get_position_with_height(edge.hex(), height);
        let outer = self.get_position_with_height(edge.source(), height);

        center + scale * (outer - center)
    }

    pub fn get_destination_vertex_position(&self, edge: HexHalfEdge, scale: f32, height: f32) -> Vector3<f32> {
        let center = self.get_position_with_height(edge.hex(), height);
        let outer = self.get_position_with_height(edge.destination(), height);

        center + scale * (outer - center)
    }

}