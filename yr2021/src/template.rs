use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Err: Could not read file.");

    for line in input.lines() {}
}
