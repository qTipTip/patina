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
        // Like pbrt, accept z_min and z_max in either order and clamp them to the sphere, where
        // -radius and radius mean the sphere isn't cut at that end. phi_max is in radians, and is
        // clamped to a full turn.
        let z_min_clamped = z_min.min(z_max).clamp(-radius, radius);
        let z_max_clamped = z_min.max(z_max).clamp(-radius, radius);
        let phi_max = phi_max.clamp(0.0, 2.0 * PI);

        let object_from_render = render_from_object.inverse();
        Self {
            radius,
            reverse_orientation,
            render_from_object,
            object_from_render,
            z_min: z_min_clamped,
            z_max: z_max_clamped,
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

#[cfg(test)]
mod test_basic_intersection {
    use std::f64::consts::PI;

    use approx::assert_relative_eq;

    use crate::{
        math::{PatinaFloat, PatinaPoint3f, PatinaRay3f, PatinaVec3f, Transform},
        shapes::sphere::Sphere,
    };

    fn ray(
        o: (PatinaFloat, PatinaFloat, PatinaFloat),
        d: (PatinaFloat, PatinaFloat, PatinaFloat),
    ) -> PatinaRay3f {
        PatinaRay3f::new(
            PatinaPoint3f::new(o.0, o.1, o.2),
            PatinaVec3f::new(d.0, d.1, d.2),
            0.0,
        )
    }

    // A full sphere of the given radius, centered at the origin of render space.
    fn full_sphere(radius: PatinaFloat) -> Sphere {
        Sphere::new(
            radius,
            Transform::identity(),
            -radius,
            radius,
            2.0 * PI,
            false,
        )
    }

    #[test]
    fn test_hit_from_outside() {
        let s = full_sphere(1.0);
        let hit = s
            .basic_intersection(
                ray((0.0, 0.0, -5.0), (0.0, 0.0, 1.0)),
                PatinaFloat::INFINITY,
            )
            .expect("ray through the center should hit");

        assert_relative_eq!(hit.t_hit, 4.0);
        assert_relative_eq!(hit.p_object.z, -1.0);
    }

    #[test]
    fn test_hit_off_axis() {
        // Hits at z = -sqrt(1 - 0.5^2 - 0.5^2) = -sqrt(0.5), at an angle of 45 degrees around z.
        let s = full_sphere(1.0);
        let hit = s
            .basic_intersection(
                ray((0.5, 0.5, -5.0), (0.0, 0.0, 1.0)),
                PatinaFloat::INFINITY,
            )
            .expect("ray should hit");

        let z = -(0.5 as PatinaFloat).sqrt();
        assert_relative_eq!(hit.t_hit, 5.0 + z);
        assert_relative_eq!(hit.p_object, PatinaPoint3f::new(0.5, 0.5, z));
        assert_relative_eq!(hit.phi, PI / 4.0);
    }

    #[test]
    fn test_hit_point_is_on_surface() {
        let s = full_sphere(2.0);
        let hit = s
            .basic_intersection(
                ray((0.3, -0.7, -10.0), (0.1, 0.05, 1.0)),
                PatinaFloat::INFINITY,
            )
            .expect("ray should hit");

        assert_relative_eq!(hit.p_object.distance(PatinaPoint3f::zero()), 2.0);
    }

    #[test]
    fn test_hit_on_axis_nudges_x() {
        // A hit exactly on the z-axis is moved slightly off it, so phi is well defined.
        let s = full_sphere(1.0);
        let hit = s
            .basic_intersection(
                ray((0.0, 0.0, -5.0), (0.0, 0.0, 1.0)),
                PatinaFloat::INFINITY,
            )
            .expect("ray should hit");

        assert_relative_eq!(hit.p_object.x, 1e-5);
        assert_relative_eq!(hit.p_object.y, 0.0);
    }

    #[test]
    fn test_miss() {
        let s = full_sphere(1.0);
        let hit = s.basic_intersection(
            ray((2.0, 0.0, -5.0), (0.0, 0.0, 1.0)),
            PatinaFloat::INFINITY,
        );

        assert!(hit.is_none());
    }

    #[test]
    fn test_sphere_behind_ray() {
        let s = full_sphere(1.0);
        let hit =
            s.basic_intersection(ray((0.0, 0.0, 5.0), (0.0, 0.0, 1.0)), PatinaFloat::INFINITY);

        assert!(hit.is_none());
    }

    #[test]
    fn test_origin_inside_hits_far_side() {
        let s = full_sphere(1.0);
        let hit = s
            .basic_intersection(ray((0.0, 0.0, 0.0), (0.0, 0.0, 1.0)), PatinaFloat::INFINITY)
            .expect("ray from inside should hit the far side");

        assert_relative_eq!(hit.t_hit, 1.0);
        assert_relative_eq!(hit.p_object.z, 1.0);
    }

    #[test]
    fn test_t_max_before_sphere() {
        // The sphere is entered at t = 4, beyond t_max.
        let s = full_sphere(1.0);
        let hit = s.basic_intersection(ray((0.0, 0.0, -5.0), (0.0, 0.0, 1.0)), 3.0);

        assert!(hit.is_none());
    }

    #[test]
    fn test_z_cut_falls_through_to_far_hit() {
        // Only the bottom half (z <= 0) exists, so a ray from above passes through the missing top
        // and hits the inside of the bottom.
        let s = Sphere::new(1.0, Transform::identity(), -1.0, 0.0, 2.0 * PI, false);
        let hit = s
            .basic_intersection(
                ray((0.0, 0.0, 5.0), (0.0, 0.0, -1.0)),
                PatinaFloat::INFINITY,
            )
            .expect("ray should hit the bottom half");

        assert_relative_eq!(hit.t_hit, 6.0);
        assert_relative_eq!(hit.p_object.z, -1.0);
    }

    #[test]
    fn test_z_cut_on_both_hits() {
        // A band around the equator: a ray along the z-axis hits the sphere only where it's cut away.
        let s = Sphere::new(1.0, Transform::identity(), -0.5, 0.5, 2.0 * PI, false);
        let hit = s.basic_intersection(
            ray((0.0, 0.0, -5.0), (0.0, 0.0, 1.0)),
            PatinaFloat::INFINITY,
        );

        assert!(hit.is_none());
    }

    #[test]
    fn test_phi_cut_falls_through_to_far_hit() {
        // Only phi in [0, pi] exists (y >= 0). The near hit has y < 0 and is cut away and the far hit at
        // y = sqrt(0.75) remains.
        let s = Sphere::new(1.0, Transform::identity(), -1.0, 1.0, PI, false);
        let hit = s
            .basic_intersection(
                ray((0.5, -5.0, 0.0), (0.0, 1.0, 0.0)),
                PatinaFloat::INFINITY,
            )
            .expect("ray should hit the remaining half");

        let y = (0.75 as PatinaFloat).sqrt();
        assert_relative_eq!(hit.t_hit, 5.0 + y);
        assert_relative_eq!(hit.p_object, PatinaPoint3f::new(0.5, y, 0.0));
        assert_relative_eq!(hit.phi, PI / 3.0);
    }

    #[test]
    fn test_render_from_object_transform() {
        // A sphere moved to z = 10 in render space. The hit is reported in object space, where the
        // sphere is still centered at the origin.
        let s = Sphere::new(
            1.0,
            Transform::translate(&PatinaVec3f::new(0.0, 0.0, 10.0)),
            -1.0,
            1.0,
            2.0 * PI,
            false,
        );
        let hit = s
            .basic_intersection(ray((0.0, 0.0, 0.0), (0.0, 0.0, 1.0)), PatinaFloat::INFINITY)
            .expect("ray should hit the moved sphere");

        assert_relative_eq!(hit.t_hit, 9.0);
        assert_relative_eq!(hit.p_object.z, -1.0);
    }
}

#[cfg(test)]
mod test_constructor {
    use std::f64::consts::PI;

    use approx::assert_relative_eq;

    use crate::{math::Transform, shapes::sphere::Sphere};

    #[test]
    fn test_z_range_is_ordered_and_clamped() {
        // Swapped, and both outside the sphere.
        let s = Sphere::new(1.0, Transform::identity(), 2.0, -3.0, 2.0 * PI, false);

        assert_relative_eq!(s.z_min, -1.0);
        assert_relative_eq!(s.z_max, 1.0);
    }

    #[test]
    fn test_z_range_inside_sphere_is_kept() {
        let s = Sphere::new(2.0, Transform::identity(), 0.5, -0.25, 2.0 * PI, false);

        assert_relative_eq!(s.z_min, -0.25);
        assert_relative_eq!(s.z_max, 0.5);
    }

    #[test]
    fn test_phi_max_is_clamped() {
        let too_large = Sphere::new(1.0, Transform::identity(), -1.0, 1.0, 10.0, false);
        let negative = Sphere::new(1.0, Transform::identity(), -1.0, 1.0, -1.0, false);

        assert_relative_eq!(too_large.phi_max, 2.0 * PI);
        assert_relative_eq!(negative.phi_max, 0.0);
    }
}
