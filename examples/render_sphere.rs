struct CameraRay {}
struct CameraSample {}

trait Camera {
    fn generate_ray() -> Option<CameraRay>;
}

fn main() {
    println!("Render sphere");
}
