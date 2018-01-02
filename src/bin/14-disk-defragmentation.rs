extern crate aoc2017;

use std::io;
use aoc2017::knothash::full_hash;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read input");
    input.pop();

    let result: Vec<_> = (0..128)
        .map(|i| {
            let hash_input = format!("{}-{}", input, i);
            let hashed = full_hash(&hash_input, 256);
            
        }).collect();

    println!("{}", result.len());
}
