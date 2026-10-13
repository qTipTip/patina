use patina::math::{Normal3, PatinaFloat, PatinaInt, Point2, Vector3};

fn main() {
    let t = Vector3::<PatinaFloat>::new(0.0, 1.0, 2.0);
    let s = Point2::<PatinaInt>::new(0, 1);
    let n = Normal3::<PatinaInt>::new(0, 1, 2);

    println!("Vector3: {t:?}");
    println!("Point2: {s:?}");
    println!("Normal3: {n:?}");
}
