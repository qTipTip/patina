use std::f64::consts::PI;

use crate::{
    math::{
        Apply, Bounds3f, ObjectSpace, PatinaFloat, PatinaNorm3f, PatinaPoint2f, PatinaPoint3f,
        PatinaRay3f, PatinaVec3f, RenderSpace, Transform,
    },
    shapes::Shape,
};

use super::{
    QuadraticIntersection, ShapeIntersection, ShapeSample, ShapeSampleContext, SurfaceInteraction,
};

/// A flat disk in the plane z = height of its object space, centered on the z-axis. An
/// inner radius above zero makes it a ring, and phi_max below a full turn cuts out a wedge.
pub struct Disk {
    height: PatinaFloat,
    radius: PatinaFloat,
    inner_radius: PatinaFloat,
    phi_max: PatinaFloat,
    // Whether to reverse the direction of the surface normal, which is +z in object space.
    reverse_orientation: bool,
    render_from_object: Transform<RenderSpace, ObjectSpace>,
    object_from_render: Transform<ObjectSpace, RenderSpace>,
}

impl Disk {
    pub fn new(
        height: PatinaFloat,
        radius: PatinaFloat,
        inner_radius: PatinaFloat,
        phi_max: PatinaFloat,
        render_from_object: Transform<RenderSpace, ObjectSpace>,
        reverse_orientation: bool,
    ) -> Self {
        debug_assert!(0.0 <= inner_radius && inner_radius < radius);
        Self {
            height,
            radius,
            inner_radius,
            phi_max: phi_max.clamp(0.0, 2.0 * PI),
            reverse_orientation,
            object_from_render: render_from_object.inverse(),
            render_from_object,
        }
    }

    /// Intersects a ray (in render space) with the disk, returning the hit in (0, t_max) in object
    /// space.
    fn basic_intersection(
        &self,
        r: PatinaRay3f<RenderSpace>,
        t_max: PatinaFloat,
    ) -> Option<QuadraticIntersection> {
        let r_object = self.object_from_render.apply(r);

        // A ray parallel to the disk's plane never reaches it.
        if r_object.d.z == 0.0 {
            return None;
        }
        // Where the ray crosses the plane z = height.
        let t_shape_hit = (self.height - r_object.o.z) / r_object.d.z;
        if t_shape_hit <= 0.0 || t_shape_hit >= t_max {
            return None;
        }

        // Whether that point is within the disk's rings and angle.
        let p_hit = r_object.eval(t_shape_hit);
        let dist2 = p_hit.x * p_hit.x + p_hit.y * p_hit.y;
        if dist2 > self.radius * self.radius || dist2 < self.inner_radius * self.inner_radius {
            return None;
        }
        let mut phi = p_hit.y.atan2(p_hit.x);
        if phi < 0.0 {
            phi += 2.0 * PI;
        }
        if phi > self.phi_max {
            return None;
        }

        Some(QuadraticIntersection {
            t_hit: t_shape_hit,
            // The hit is exactly in the disk's plane.
            p_object: PatinaPoint3f::new(p_hit.x, p_hit.y, self.height),
            phi,
        })
    }

    // Builds the SurfaceInteraction for a hit found by basic_intersection, transformed to render
    // space. `wo` is the direction back towards the ray's origin, in render space.
    fn interaction_from_intersection(
        &self,
        isect: &QuadraticIntersection,
        wo: PatinaVec3f<RenderSpace>,
        time: PatinaFloat,
    ) -> SurfaceInteraction {
        let p_object = isect.p_object;

        // (u, v): u goes around the z-axis from 0 to phi_max, v from the outer to the inner radius.
        let r_hit = (p_object.x * p_object.x + p_object.y * p_object.y).sqrt();
        let u = isect.phi / self.phi_max;
        let v = (self.radius - r_hit) / (self.radius - self.inner_radius);

        let mut n_object: PatinaNorm3f<ObjectSpace> = PatinaNorm3f::new(0.0, 0.0, 1.0);
        if self.reverse_orientation {
            n_object = -n_object;
        }

        SurfaceInteraction {
            p: self.render_from_object.apply(p_object),
            n: self.render_from_object.apply(n_object).normalize(),
            uv: PatinaPoint2f::new(u, v),
            wo: wo.normalize(),
            time,
        }
    }
}

#[allow(unused_variables)]
impl Shape for Disk {
    fn bounds(&self) -> Bounds3f<RenderSpace> {
        self.render_from_object.apply(Bounds3f::new(
            PatinaPoint3f::new(-self.radius, -self.radius, self.height),
            PatinaPoint3f::new(self.radius, self.radius, self.height),
        ))
    }

    fn intersect(
        &self,
        ray: &PatinaRay3f<RenderSpace>,
        t_max: Option<PatinaFloat>,
    ) -> Option<ShapeIntersection> {
        let isect = self.basic_intersection(*ray, t_max.unwrap_or(PatinaFloat::INFINITY))?;
        let interaction = self.interaction_from_intersection(&isect, -ray.d, ray.t);

        Some(ShapeIntersection {
            interaction,
            t_hit: isect.t_hit,
        })
    }

    fn intersects(&self, ray: &PatinaRay3f<RenderSpace>, t_max: Option<PatinaFloat>) -> bool {
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

    fn pdf_from_point(
        &self,
        ctx: &ShapeSampleContext,
        w_incident: PatinaVec3f<RenderSpace>,
    ) -> PatinaFloat {
        todo!()
    }
}

#[cfg(test)]
mod test_disk {
    use std::f64::consts::PI;

    use approx::assert_relative_eq;

    use crate::{
        math::{
            ObjectSpace, PatinaNorm3f, PatinaPoint2f, PatinaPoint3f, PatinaRay3f, PatinaVec3f,
            RenderSpace, Transform,
        },
        shapes::{Disk, Shape},
    };

    fn ray(o: (f64, f64, f64), d: (f64, f64, f64)) -> PatinaRay3f<RenderSpace> {
        PatinaRay3f::new(
            PatinaPoint3f::new(o.0, o.1, o.2),
            PatinaVec3f::new(d.0, d.1, d.2).normalize(),
            0.0,
        )
    }

    // A unit disk in the plane z = 2, as seen from the origin.
    fn disk_at_z2(inner_radius: f64, phi_max: f64) -> Disk {
        Disk::new(
            2.0,
            1.0,
            inner_radius,
            phi_max,
            Transform::identity(),
            false,
        )
    }

    #[test]
    fn test_hit() {
        let isect = disk_at_z2(0.0, 2.0 * PI)
            .intersect(&ray((0.3, 0.4, 0.0), (0.0, 0.0, 1.0)), None)
            .expect("ray should hit the disk");

        assert_relative_eq!(isect.t_hit, 2.0);
        assert_relative_eq!(isect.interaction.p, PatinaPoint3f::new(0.3, 0.4, 2.0));
        assert_relative_eq!(isect.interaction.n, PatinaNorm3f::new(0.0, 0.0, 1.0));
        // u = phi / phi_max, v = (radius - r_hit) / (radius - inner_radius) with r_hit = 0.5.
        let phi = (0.4f64).atan2(0.3);
        assert_relative_eq!(
            isect.interaction.uv,
            PatinaPoint2f::new(phi / (2.0 * PI), 0.5)
        );
    }

    #[test]
    fn test_misses() {
        let disk = disk_at_z2(0.0, 2.0 * PI);

        // Outside the radius, parallel to the plane, the plane behind the ray, and t_max too short.
        assert!(
            disk.intersect(&ray((1.5, 0.0, 0.0), (0.0, 0.0, 1.0)), None)
                .is_none()
        );
        assert!(
            disk.intersect(&ray((0.0, 0.0, 0.0), (1.0, 0.0, 0.0)), None)
                .is_none()
        );
        assert!(
            disk.intersect(&ray((0.0, 0.0, 3.0), (0.0, 0.0, 1.0)), None)
                .is_none()
        );
        assert!(
            disk.intersect(&ray((0.0, 0.0, 0.0), (0.0, 0.0, 1.0)), Some(1.5))
                .is_none()
        );
    }

    #[test]
    fn test_inner_radius_and_phi_max_cut_away() {
        // A ring: the center is cut away.
        let ring = disk_at_z2(0.5, 2.0 * PI);
        assert!(
            ring.intersect(&ray((0.1, 0.0, 0.0), (0.0, 0.0, 1.0)), None)
                .is_none()
        );
        assert!(
            ring.intersect(&ray((0.8, 0.0, 0.0), (0.0, 0.0, 1.0)), None)
                .is_some()
        );

        // Half a disk: only y >= 0 (phi in [0, pi]) remains.
        let half = disk_at_z2(0.0, PI);
        assert!(
            half.intersect(&ray((0.2, 0.5, 0.0), (0.0, 0.0, 1.0)), None)
                .is_some()
        );
        assert!(
            half.intersect(&ray((0.2, -0.5, 0.0), (0.0, 0.0, 1.0)), None)
                .is_none()
        );
    }

    #[test]
    fn test_ground_plane() {
        // A disk turned to lie horizontally at y = -1: object z maps to render +y, so the normal
        // points up. Rotating by -90 degrees around x takes +z to +y.
        let render_from_object: Transform<RenderSpace, ObjectSpace> =
            Transform::translate(&PatinaVec3f::new(0.0, -1.0, 0.0))
                * Transform::<ObjectSpace, ObjectSpace>::rotate_x(-PI / 2.0);
        let ground = Disk::new(0.0, 100.0, 0.0, 2.0 * PI, render_from_object, false);

        let isect = ground
            .intersect(&ray((0.0, 0.0, 0.0), (0.0, -1.0, 1.0)), None)
            .expect("a ray looking down should hit the ground");
        assert_relative_eq!(
            isect.interaction.p,
            PatinaPoint3f::new(0.0, -1.0, 1.0),
            epsilon = 1e-12
        );
        assert_relative_eq!(
            isect.interaction.n,
            PatinaNorm3f::new(0.0, 1.0, 0.0),
            epsilon = 1e-12
        );

        // The bounds are flat in y.
        let bounds = ground.bounds();
        assert_relative_eq!(bounds.p_min.y, -1.0, epsilon = 1e-12);
        assert_relative_eq!(bounds.p_max.y, -1.0, epsilon = 1e-12);

        // A ray looking up misses it.
        assert!(
            ground
                .intersect(&ray((0.0, 0.0, 0.0), (0.0, 1.0, 1.0)), None)
                .is_none()
        );
    }

    #[test]
    fn test_reverse_orientation_flips_the_normal() {
        let disk = Disk::new(2.0, 1.0, 0.0, 2.0 * PI, Transform::identity(), true);
        let isect = disk
            .intersect(&ray((0.0, 0.2, 0.0), (0.0, 0.0, 1.0)), None)
            .expect("ray should hit the disk");

        assert_relative_eq!(isect.interaction.n, PatinaNorm3f::new(0.0, 0.0, -1.0));
    }
}
