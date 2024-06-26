# SturdyHex
Yet another hex library for Rust!  I've made it available in an early state with partially completed documentation and tests and no published crate, but these things are in the works.
## Coordinates and Basic Usage
SturdyHex uses the axial coordinate system (and several of the algorithms) from Amit Patel's [excellent resource](https://www.redblobgames.com/grids/hexagons/) on hex grids.  The `hex_field` struct relates a worldspace coordinate system with a hex coordinate system:
```rust
let hex_field = HexField::new(
	Vector3::<f32>::zero(), //center of hex (0, 0, 0)
	Vector3::<f32>::new(0.0, 0.0, 1.0), //up direction
	Vector3::<f32>::new(0.0, 1.0, 0.0), //center of hex with coord (0, 1, -1)
);

let hex_containing_origin = hex_field.get_hex_coord(Vector3::<f32>::zero());
let position_of_center = hex_field.get_position(HexCoord::ZERO);
```
You can choose between Amit Patel's "flat-top" and "pointy-top" orientations by your choice of the center for hex *(0, -1, 1)*; in the example above, it's on the *y*-axis, providing flat-top coordinates.

SturdyHex currently uses [cgmath](https://docs.rs/cgmath/latest/cgmath/) for vector math, but I intend to switch to [glam](https://docs.rs/glam/latest/glam/) and provide [mint](https://docs.rs/mint/latest/mint/) compatibility.

## Overview of Advanced Features
In this section I'll provide a brief overview of other features and tell you where to look in the source code to understand them better.  Further documentation forthcoming.
### Grid traversal
SturdyHex provides methods to interact with a hex grid as a [doubly-connected edge list](https://en.wikipedia.org/wiki/Doubly_connected_edge_list).  Because hex grids are regular, all connectivity information is implicit; we can calculate how hexes, vertices, and edges connect, so we don't need to store any connection information.

See `HexCoord`, `HexVertex`, and `HexHalfEdge` for the relevant methods.
### Chunking
Sometimes it's useful to partition a hex grid into hex-shaped chunks.  Unfortunately, this is not as straightforward as it is with squares and square-shaped chunks.  You can create a `HexChunker` instance with a specified chunk radius to perform conversions from hex coordinate to chunk coordinate and back.  SturdyHex uses Sander Ever's [algorithm](https://observablehq.com/@sanderevers/hexagon-tiling-of-an-hexagonal-grid) for converting in the hex-to-chunk direction.
### Transforms
The `HexTransform` struct represents a translation and rotation in hex coordinates.  You can apply them to hexes, vertices, edges, and other transforms via the `*` operator.
### Shapes
The `HexShape` class stores a list of hex coordinates and supports the `contains` operation.  You can create a `HexShapeView` that references a `HexShape` and encodes a transformation on it; this allows you to query against a shape without copying its data.
### Range Iteration
You can create a `HexRangeIterator` instance with a center coordinate and radius to iterate over the corresponding hexagonal range of hexes.

## License
Licensed under either of [Apache License, Version 2.0](APACHE-LICENSE) or [MIT License](MIT-LICENSE) at your option.
