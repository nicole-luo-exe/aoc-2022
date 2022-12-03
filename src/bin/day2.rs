use std::io::{self, stdin, BufReader};

fn part_one(input: &String) {}

fn main() {
    let input = io::read_to_string(BufReader::new(stdin())).unwrap();
    part_one(&input);
}
