use rand::Rng;
fn main() {
    let mut rng = rand::thread_rng();
    println!("{}", f32::sin(rng.gen_range(0..360) as f32));
}
