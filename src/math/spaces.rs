// Coordinate space tags.
//
// A Transform<To, From> carries two of these, so the compiler checks that
// transforms are composed in a consistent order: Transform<A, B> * Transform<B, C> is a
// Transform<A, C>.

/// The space the scene is described in.
pub enum WorldSpace {}
/// The space rendering happens in: world, camera or camera-world space, chosen at runtime by
/// RenderingSpace.
pub enum RenderSpace {}
/// The camera at the origin, looking down +z, with +y up.
pub enum CameraSpace {}
/// A shape's own space, e.g. a sphere centered at the origin.
pub enum ObjectSpace {}
/// The image plane after projection: the screen window spans x and y, and z is 0 at the near plane
/// and 1 at the far plane.
pub enum ScreenSpace {}
/// Normalized device coordinates: the screen window mapped to [0, 1] x [0, 1].
pub enum NdcSpace {}
/// Pixel coordinates on the film: [0, width] x [0, height], with y pointing down.
pub enum RasterSpace {}
