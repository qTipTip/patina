use patina::math::{PatinaFloat, points::PatinaPoint2f, rays::ray3::PatinaRay3f};

struct SampledSpectrum {}

struct CameraRay {
    ray: PatinaRay3f,
    weight: SampledSpectrum,
}
struct CameraSample {
    p_film: PatinaPoint2f, // Point on the film in (x, y)
    p_lens: PatinaPoint2f, // Point on the lens in (x, y)
    time: PatinaFloat,
    filterWeight: PatinaFloat,
}

struct Film {}

trait Camera {
    fn generate_ray() -> Option<CameraRay>;
    fn get_film() -> Film;
    // Maps u in [0, 1) to [shutter_open, shutter_close).
    fn sample_time(u: PatinaFloat) -> PatinaFloat;
}

// Meant to serve as a minimal working render. We should render points in a sphere as white, and
// everything else black.
fn main() {
    println!("Render sphere");
}
