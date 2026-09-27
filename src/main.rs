use patina::math::{
    PatinaFloat, PatinaInt,
    tuples::{tuple2::Tuple2, tuple3::Tuple3},
};

fn main() {
    let t = Tuple3::<PatinaFloat>::new(0.0, 1.0, 2.0);
    let s = Tuple2::<PatinaInt>::new(0, 1);

    println!("Tuple3: {t:?}");
    println!("Tuple2: {s:?}");
}
