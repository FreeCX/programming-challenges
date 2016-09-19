pub mod pseudo;

use std::time::SystemTime;
use pseudo::Rng;

fn generate(gen: &mut Rng, alphabet: &str, length: u32) -> String {
    let mut password = String::new();
    let max_index = alphabet.len() as u32;
    for _ in 0..length {
        let index = (gen.rand() % max_index) as usize;
        password.push(alphabet.chars().skip(index).next().unwrap());
    }
    password
}

fn main() {
    let time = SystemTime::now().elapsed().unwrap();
    let mut rng = Rng::new(time.subsec_nanos());
    let alphabet = "abcdefghijklmnopqrstuvwxyz1234567890";
    let length = 8;
    println!("password = `{}`", generate(&mut rng, alphabet, length))
}