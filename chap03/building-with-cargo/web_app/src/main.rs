use rand::prelude::*;

fn generate_float(generator: &mut ThreadRng) -> f64 {
    generator.random_range(1.0..10.0)
}

fn main() {
    let mut rng: ThreadRng = rand::rng();
    let random_number = generate_float(&mut rng);
    println!("{}", random_number);
}