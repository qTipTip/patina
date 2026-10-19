use std::{f64::consts::PI, mem::swap};

use crate::{
    math::{
        Apply, Bounds3f, PatinaFloat, PatinaPoint2f, PatinaPoint3f, PatinaRay3f, PatinaVec3f,
        Transform,
    },
    shapes::Shape,
};

use super::{ShapeSample, ShapeSampleContext};

/// A sphere is sweeped out using polar coordinates around the origin (in object space). A
/// transformation (render_from_object/object_from_render) is used to place the sphere in render
/// space.
pub struct Sphere {
    // Radius of the sphere
    radius: PatinaFloat,
    // Whether to reverse the direction of surface normals to point inwards.
    reverse_orientation: bool,
    // Places the object in rendering space.
    render_from_object: Transform,
    object_from_render: Transform,
    // z_min and z_max for truncated sphere if desired
    z_min: PatinaFloat,
    z_max: PatinaFloat,
    // phi_max for partial sphere
    phi_max: PatinaFloat,
}

impl Sphere {
    pub fn new(
        radius: PatinaFloat,
        render_from_object: Transform,
        z_min: PatinaFloat,
        z_max: PatinaFloat,
        phi_max: PatinaFloat,
        reverse_orientation: bool,
    ) -> Self {
        let object_from_render = render_from_object.inverse();
        Self {
            radius,
            reverse_orientation,
            render_from_object,
            object_from_render,
            z_min,
            z_max,
            phi_max,
        }
    }

    /// Intersects a ray (in render space) with the sphere, returning the first hit in (0, t_max].
    /// The hit is reported in object space; `intersect` turns it into a SurfaceInteraction.
    fn basic_intersection(
        &self,
        r: PatinaRay3f,
        t_max: PatinaFloat,
    ) -> Option<QuadraticIntersection> {
        debug_assert!(self.z_min < self.z_max);
        debug_assert!(self.z_min > -self.radius && self.z_min < self.radius);
        debug_assert!(self.z_max > -self.radius && self.z_max < self.radius);

        // Intersect in object space, where the sphere is centered at the origin.
        let r_object = self.object_from_render.apply(r);
        let o = PatinaVec3f::from(r_object.o);
        let d = r_object.d;

        // TODO: Implement interval arithmetic for the intersection checks.
        let a = d.dot(&d);
        let b = 2.0 * d.dot(&o);
        let c = o.dot(&o) - self.radius * self.radius;

        // Compute the discriminant b^2 - 4ac, in pbrt's more accurate form.
        let v = o - d * (b / (2.0 * a));
        let l = v.len();
        let discrim = 4.0 * a * (self.radius + l) * (self.radius - l);
        if discrim < 0.0 {
            return None;
        }

        // Compute the roots t0 <= t1, avoiding cancellation between b and the root.
        let root_discrim = discrim.sqrt();
        let q = if b < 0.0 {
            -0.5 * (b - root_discrim)
        } else {
            -0.5 * (b + root_discrim)
        };
        let mut t0 = q / a;
        let mut t1 = c / q;
        if t0 > t1 {
            swap(&mut t0, &mut t1);
        }

        // Both hits are outside (0, t_max].
        if t0 > t_max || t1 <= 0.0 {
            return None;
        }

        // Use the nearest hit in front of the ray's origin.
        let mut t_shape_hit = t0;
        if t_shape_hit <= 0.0 {
            t_shape_hit = t1;
            if t_shape_hit > t_max {
                return None;
            }
        }

        let (mut p_hit, mut phi) = self.hit_point_and_phi(&r_object, t_shape_hit);

        // A partial sphere may cut away the nearest hit, then try the far one.
        if self.is_cut_away(&p_hit, phi) {
            if t_shape_hit == t1 || t1 > t_max {
                return None;
            }
            t_shape_hit = t1;
            (p_hit, phi) = self.hit_point_and_phi(&r_object, t_shape_hit);
            if self.is_cut_away(&p_hit, phi) {
                return None;
            }
        }

        Some(QuadraticIntersection {
            t_hit: t_shape_hit,
            p_object: p_hit,
            phi,
        })
    }

    // The object-space point at distance t along the ray, and its angle phi around the z-axis.
    fn hit_point_and_phi(
        &self,
        r_object: &PatinaRay3f,
        t: PatinaFloat,
    ) -> (PatinaPoint3f, PatinaFloat) {
        let mut p_hit = r_object.eval(t);

        // Refine the hit point by projecting it back onto the sphere's surface.
        p_hit = p_hit * (self.radius / p_hit.distance(PatinaPoint3f::zero()));
        // Avoid a degenerate phi (and later derivatives) for hits exactly on the z-axis.
        if p_hit.x == 0.0 && p_hit.y == 0.0 {
            p_hit.x = 1e-5 * self.radius;
        }

        let mut phi = p_hit.y.atan2(p_hit.x);
        if phi < 0.0 {
            phi += 2.0 * PI;
        }
        (p_hit, phi)
    }

    // Whether a hit point lies outside the part of the sphere bounded by z_min, z_max and phi_max.
    fn is_cut_away(&self, p_hit: &PatinaPoint3f, phi: PatinaFloat) -> bool {
        (self.z_min > -self.radius && p_hit.z < self.z_min)
            || (self.z_max < self.radius && p_hit.z > self.z_max)
            || phi > self.phi_max
    }
}

// The result of a ray-sphere intersection in object space. pbrt calls this QuadricIntersection, as
// it's shared by the quadric shapes (spheres, cylinders, disks).
struct QuadraticIntersection {
    t_hit: PatinaFloat,
    p_object: PatinaPoint3f,
    phi: PatinaFloat,
}

impl Shape for Sphere {
    fn bounds(&self) -> Bounds3f {
        self.render_from_object.apply(Bounds3f::new(
            PatinaPoint3f::new(-self.radius, -self.radius, self.z_min),
            PatinaPoint3f::new(self.radius, self.radius, self.z_max),
        ))
    }

    fn intersect(
        &self,
        ray: &PatinaRay3f,
        t_max: Option<PatinaFloat>,
    ) -> Option<super::ShapeIntersection> {
        todo!()
    }

    fn intersects(&self, ray: &PatinaRay3f, t_max: Option<PatinaFloat>) -> bool {
        todo!()
    }

    fn area(&self) -> PatinaFloat {
        todo!()
    }

    fn sample(&self, u: PatinaPoint2f) -> Option<ShapeSample> {
        todo!()
    }

    fn pdf(&self, interaction: &super::Interaction) -> PatinaFloat {
        todo!()
    }

    fn sample_from_point(&self, ctx: &ShapeSampleContext, u: PatinaPoint2f) -> Option<ShapeSample> {
        todo!()
    }

    fn pdf_from_point(&self, ctx: &ShapeSampleContext, w_incident: PatinaVec3f) -> PatinaFloat {
        todo!()
    }
}
