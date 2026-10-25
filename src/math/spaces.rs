// Coordinate space tags.
//
// A Transform<To, From> carries two of these, so the compiler checks that
// transforms are composed in a consistent order: Transform<A, B> * Transform<B, C> is a
// Transform<A, C>.

/// The space the scene is described in.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum WorldSpace {}
/// The space rendering happens in: world, camera or camera-world space, chosen at runtime by
/// RenderingSpace.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum RenderSpace {}
/// The camera at the origin, looking down +z, with +y up.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CameraSpace {}
/// A shape's own space, e.g. a sphere centered at the origin.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ObjectSpace {}
/// The image plane after projection: the screen window spans x and y, and z is 0 at the near plane
/// and 1 at the far plane.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ScreenSpace {}
/// Normalized device coordinates: the screen window mapped to [0, 1] x [0, 1].
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum NdcSpace {}
/// Pixel coordinates on the film: [0, width] x [0, height], with y pointing down.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum RasterSpace {}
/// The local frame at a point on a surface, used by BSDFs. The shading normal is +z, and x and y
/// lie in the tangent plane. Angles to the normal are then simply the z-value.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ShadingSpace {}
