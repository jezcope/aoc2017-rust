extern crate aoc2017;

use std::io;
use aoc2017::knothash::{simple_hash, full_hash};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read input");

    let simple_result = simple_hash(input.trim().split(',')
                                    .map(|x| x.parse().unwrap()), 256);

    println!("{}", simple_result);

    let full_input = input.trim();
    let full_result = full_hash(full_input, 256);

    println!("{}", full_result);
}
